//! 本番用の [`SecretGenerator`]。
//!
//! - secret: 256bit を CSPRNG(`rand::rng()` = ChaCha ベースの暗号論的乱数)で生成し base64url 化。
//! - hash: HMAC-SHA256(pepper, secret) の hex。pepper は設定にのみ存在し DB には出さないため，
//!   DB 単独流出では有効トークンを再構成・照合できない。
//! - verify: 再計算した HMAC を定数時間で比較する(`subtle`)。

use crate::application::ports::secret_generator::SecretGenerator;
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;
use subtle::ConstantTimeEq;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

pub struct RandomSecretGenerator {
    pepper: Vec<u8>,
}

impl RandomSecretGenerator {
    pub fn new(pepper: impl Into<Vec<u8>>) -> Self {
        Self {
            pepper: pepper.into(),
        }
    }

    fn hmac_hex(&self, secret: &str) -> String {
        // HMAC は任意長の鍵を受け付けるため new_from_slice は失敗しない。
        let mut mac =
            HmacSha256::new_from_slice(&self.pepper).expect("HMAC accepts keys of any length");
        mac.update(secret.as_bytes());
        let tag = mac.finalize().into_bytes();
        let mut hex = String::with_capacity(tag.len() * 2);
        for b in tag.iter() {
            hex.push_str(&format!("{b:02x}"));
        }
        hex
    }
}

impl SecretGenerator for RandomSecretGenerator {
    fn generate_secret(&self) -> String {
        let mut buf = [0u8; 32];
        rand::rng().fill_bytes(&mut buf);
        base64url::encode(&buf)
    }

    fn hash_secret(&self, secret: &str) -> String {
        self.hmac_hex(secret)
    }

    fn verify_secret(&self, secret: &str, hash: &str) -> bool {
        let computed = self.hmac_hex(secret);
        bool::from(computed.as_bytes().ct_eq(hash.as_bytes()))
    }

    fn new_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_distinct_high_entropy_secrets() {
        let g = RandomSecretGenerator::new(b"pepper".to_vec());
        let a = g.generate_secret();
        let b = g.generate_secret();
        assert_ne!(a, b);
        // 32 バイトの base64url は 43 文字(パディングなし)。
        assert_eq!(a.len(), 43);
    }

    #[test]
    fn hash_is_deterministic_and_pepper_dependent() {
        let g1 = RandomSecretGenerator::new(b"pepper-1".to_vec());
        let g2 = RandomSecretGenerator::new(b"pepper-2".to_vec());
        assert_eq!(g1.hash_secret("s"), g1.hash_secret("s"));
        assert_ne!(g1.hash_secret("s"), g2.hash_secret("s"));
    }

    #[test]
    fn verify_matches_only_correct_secret() {
        let g = RandomSecretGenerator::new(b"pepper".to_vec());
        let secret = g.generate_secret();
        let hash = g.hash_secret(&secret);
        assert!(g.verify_secret(&secret, &hash));
        assert!(!g.verify_secret("wrong", &hash));
        assert!(!g.verify_secret(&secret, "deadbeef"));
    }

    #[test]
    fn new_id_is_unique() {
        let g = RandomSecretGenerator::new(b"pepper".to_vec());
        assert_ne!(g.new_id(), g.new_id());
    }
}
