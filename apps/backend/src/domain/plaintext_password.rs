use crate::domain::error::FactoryError;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// 検証済みの平文パスワードを表す値オブジェクト。
///
/// `new` でのみ生成でき，その時点で長さポリシー(NIST 800-63B: 長さ優先・合成ルール無し)を
/// 検証する。これにより「未検証の平文をハッシュ化・保存してしまう」事故を型で防ぐ。
/// 中身は `Debug` でマスクされ，`Drop` 時に [`Zeroize`] でメモリをゼロ消去する。
///
/// なお，ログイン照合の入力平文はこの型を通さず `&str` のまま扱う
/// (古い・短いパスワードでも照合できるようにするため。検証は設定時のみ)。
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PlaintextPassword(String);

impl PlaintextPassword {
    /// 最小文字数(Unicode スカラ値で数える)。
    pub const MIN_LEN: usize = 12;
    /// 最大バイト長。argon2 への過大入力(メモリ/CPU)を防ぐ上限。
    pub const MAX_LEN: usize = 1024;

    pub fn new(raw: String) -> Result<Self, FactoryError> {
        if raw.chars().count() < Self::MIN_LEN {
            return Err(FactoryError::InvalidInput(format!(
                "Password must be at least {} characters",
                Self::MIN_LEN
            )));
        }
        if raw.len() > Self::MAX_LEN {
            return Err(FactoryError::InvalidInput(format!(
                "Password must be at most {} bytes",
                Self::MAX_LEN
            )));
        }
        Ok(Self(raw))
    }

    /// ハッシュ化アダプタが内部で平文を参照するための入り口。
    /// クレート内に限定し，外部へ平文が漏れないようにする。
    pub(crate) fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for PlaintextPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PlaintextPassword(***)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_accepts_min_length() {
        let pw = "a".repeat(PlaintextPassword::MIN_LEN);
        assert!(PlaintextPassword::new(pw).is_ok());
    }

    #[test]
    fn test_new_rejects_too_short() {
        let pw = "a".repeat(PlaintextPassword::MIN_LEN - 1);
        assert!(PlaintextPassword::new(pw).is_err());
    }

    #[test]
    fn test_new_counts_unicode_scalar_values() {
        // 絵文字 12 個はバイト長では 48 だが文字数では 12 なので許可される。
        let pw = "😀".repeat(12);
        assert!(PlaintextPassword::new(pw).is_ok());
    }

    #[test]
    fn test_new_rejects_over_max_bytes() {
        let pw = "a".repeat(PlaintextPassword::MAX_LEN + 1);
        assert!(PlaintextPassword::new(pw).is_err());
    }

    #[test]
    fn test_debug_is_masked() {
        let pw = PlaintextPassword::new("supersecret123".to_string()).unwrap();
        assert_eq!(format!("{:?}", pw), "PlaintextPassword(***)");
    }
}
