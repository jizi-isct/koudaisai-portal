//! argon2id バックエンドの [`PasswordHasher`] 実装。
//!
//! OWASP 推奨パラメータ(m=19456KiB, t=2, p=1)を既定とし，CPU バウンドな
//! ハッシュ計算は `spawn_blocking` で非同期ランタイムのリアクタを塞がないようにする。
//! さらに `tokio::Semaphore` で同時実行数に上限を設け，未認証フラッドによる
//! メモリ枯渇(1 リクエストあたり ~19MiB)を防ぐ。

use crate::application::ports::password_hasher::{
    PasswordHashError, PasswordHasher, VerifyOutcome,
};
use crate::domain::plaintext_password::PlaintextPassword;
use anyhow::anyhow;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordHasher as _, PasswordVerifier as _, Version};
use std::sync::Arc;
use tokio::sync::Semaphore;
use zeroize::Zeroizing;

#[derive(Clone)]
pub struct Argon2PasswordHasher {
    m_cost_kib: u32,
    t_cost: u32,
    p_cost: u32,
    output_len: usize,
    semaphore: Arc<Semaphore>,
}

impl Argon2PasswordHasher {
    /// OWASP 推奨の既定パラメータ。
    pub const DEFAULT_M_COST_KIB: u32 = 19456;
    pub const DEFAULT_T_COST: u32 = 2;
    pub const DEFAULT_P_COST: u32 = 1;
    pub const DEFAULT_OUTPUT_LEN: usize = 32;

    pub fn new(
        m_cost_kib: u32,
        t_cost: u32,
        p_cost: u32,
        output_len: usize,
        max_parallelism: usize,
    ) -> Self {
        Self {
            m_cost_kib,
            t_cost,
            p_cost,
            output_len,
            semaphore: Arc::new(Semaphore::new(max_parallelism.max(1))),
        }
    }

    fn params(&self) -> Result<Params, PasswordHashError> {
        Params::new(self.m_cost_kib, self.t_cost, self.p_cost, Some(self.output_len))
            .map_err(|e| PasswordHashError::Internal(anyhow!("invalid argon2 params: {e}")))
    }

    fn argon2(params: Params) -> Argon2<'static> {
        Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
    }

    fn hash_bytes(params: Params, bytes: &[u8]) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(Self::argon2(params).hash_password(bytes, &salt)?.to_string())
    }
}

/// 照合に成功した PHC が，現行アルゴリズム・現行パラメータ未満なら再ハッシュが必要。
fn needs_rehash(parsed: &PasswordHash, cur_m: u32, cur_t: u32, cur_p: u32, cur_out: usize) -> bool {
    if parsed.algorithm != Algorithm::Argon2id.ident() {
        return true;
    }
    if parsed.version != Some(Version::V0x13 as u32) {
        return true;
    }
    match Params::try_from(parsed) {
        Ok(p) => {
            p.m_cost() < cur_m
                || p.t_cost() < cur_t
                || p.p_cost() < cur_p
                // 出力長が未指定/現行未満なら再ハッシュ対象。
                || p.output_len().is_none_or(|o| o < cur_out)
        }
        Err(_) => true,
    }
}

#[async_trait::async_trait]
impl PasswordHasher for Argon2PasswordHasher {
    async fn hash(&self, password: &PlaintextPassword) -> Result<String, PasswordHashError> {
        let params = self.params()?;
        // spawn_blocking へ渡す平文コピーはブロック終了時に確実にゼロ消去する。
        let bytes = Zeroizing::new(password.expose().as_bytes().to_vec());
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|e| PasswordHashError::Internal(anyhow!("semaphore closed: {e}")))?;
        tokio::task::spawn_blocking(move || Self::hash_bytes(params, bytes.as_slice()))
            .await
            .map_err(|e| PasswordHashError::Internal(anyhow!("join error: {e}")))?
            .map_err(|e| PasswordHashError::Internal(anyhow!("argon2 hash failed: {e}")))
    }

    async fn verify(&self, attempt: &str, phc: &str) -> Result<VerifyOutcome, PasswordHashError> {
        let params = self.params()?;
        let bytes = Zeroizing::new(attempt.as_bytes().to_vec());
        let phc = phc.to_string();
        let (cur_m, cur_t, cur_p, cur_out) =
            (self.m_cost_kib, self.t_cost, self.p_cost, self.output_len);
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|e| PasswordHashError::Internal(anyhow!("semaphore closed: {e}")))?;
        tokio::task::spawn_blocking(move || -> VerifyOutcome {
            // PHC のパース失敗は fail-closed: 不一致として扱う。
            let parsed = match PasswordHash::new(&phc) {
                Ok(p) => p,
                Err(_) => return VerifyOutcome::Mismatch,
            };
            match Self::argon2(params.clone()).verify_password(bytes.as_slice(), &parsed) {
                Ok(()) => {
                    if needs_rehash(&parsed, cur_m, cur_t, cur_p, cur_out) {
                        // 照合は成功済み。再ハッシュ失敗は致命ではないので Verified に落とす。
                        match Self::hash_bytes(params, bytes.as_slice()) {
                            Ok(new_phc) => VerifyOutcome::VerifiedNeedsRehash { new_phc },
                            Err(_) => VerifyOutcome::Verified,
                        }
                    } else {
                        VerifyOutcome::Verified
                    }
                }
                // 不一致もパース後の各種エラーも fail-closed に Mismatch。
                Err(_) => VerifyOutcome::Mismatch,
            }
        })
        .await
        .map(Ok)
        .map_err(|e| PasswordHashError::Internal(anyhow!("join error: {e}")))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テストはコストを小さくして高速化する(セキュリティ品質はパラメータ非依存で検証可能)。
    fn fast_hasher() -> Argon2PasswordHasher {
        Argon2PasswordHasher::new(256, 1, 1, 32, 2)
    }

    #[tokio::test]
    async fn test_hash_then_verify_roundtrip() {
        let h = fast_hasher();
        let pw = PlaintextPassword::new("correct horse battery".to_string()).unwrap();
        let phc = h.hash(&pw).await.unwrap();
        assert!(phc.starts_with("$argon2id$"));
        assert_eq!(h.verify("correct horse battery", &phc).await.unwrap(), VerifyOutcome::Verified);
    }

    #[tokio::test]
    async fn test_verify_rejects_wrong_password() {
        let h = fast_hasher();
        let pw = PlaintextPassword::new("correct horse battery".to_string()).unwrap();
        let phc = h.hash(&pw).await.unwrap();
        assert_eq!(h.verify("wrong password here", &phc).await.unwrap(), VerifyOutcome::Mismatch);
    }

    #[tokio::test]
    async fn test_verify_malformed_phc_is_mismatch_not_error() {
        let h = fast_hasher();
        assert_eq!(h.verify("whatever", "not-a-phc-string").await.unwrap(), VerifyOutcome::Mismatch);
    }

    #[tokio::test]
    async fn test_verify_flags_rehash_for_shorter_output_len() {
        // out=16 でハッシュ → 現行 out=32 で照合すると再ハッシュ要求。
        let short = Argon2PasswordHasher::new(256, 1, 1, 16, 2);
        let pw = PlaintextPassword::new("correct horse battery".to_string()).unwrap();
        let phc = short.hash(&pw).await.unwrap();
        let cur = Argon2PasswordHasher::new(256, 1, 1, 32, 2);
        match cur.verify("correct horse battery", &phc).await.unwrap() {
            VerifyOutcome::VerifiedNeedsRehash { .. } => {}
            other => panic!("expected VerifiedNeedsRehash, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_verify_flags_rehash_for_weaker_params() {
        // 弱いパラメータ(t=1)でハッシュ → 強い現行(t=3)で照合すると再ハッシュ要求。
        let weak = Argon2PasswordHasher::new(256, 1, 1, 32, 2);
        let pw = PlaintextPassword::new("correct horse battery".to_string()).unwrap();
        let phc = weak.hash(&pw).await.unwrap();

        let strong = Argon2PasswordHasher::new(256, 3, 1, 32, 2);
        match strong.verify("correct horse battery", &phc).await.unwrap() {
            VerifyOutcome::VerifiedNeedsRehash { new_phc } => {
                assert!(new_phc.starts_with("$argon2id$"));
                assert_ne!(new_phc, phc);
            }
            other => panic!("expected VerifiedNeedsRehash, got {other:?}"),
        }
    }
}
