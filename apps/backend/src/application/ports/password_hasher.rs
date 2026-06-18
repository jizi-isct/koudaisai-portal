use crate::domain::plaintext_password::PlaintextPassword;

/// パスワード照合の結果。
#[derive(Debug, PartialEq, Eq)]
pub enum VerifyOutcome {
    /// 一致。かつ現行アルゴリズム・現行パラメータでハッシュされている。
    Verified,
    /// 一致したが，旧アルゴリズム/旧パラメータのため再ハッシュ(透過昇格)すべき。
    /// `new_phc` は現行設定で再計算した PHC 文字列。
    VerifiedNeedsRehash { new_phc: String },
    /// 不一致。これは異常ではないので `Err` ではなく `Ok` の一バリアントとして返す。
    Mismatch,
}

/// パスワードハッシュ処理中に発生した内部エラー。
///
/// PHC のパース失敗のような「壊れた/未知のハッシュ」は **fail-closed** に
/// [`VerifyOutcome::Mismatch`] へ畳み込むため，ここには現れない。
/// このエラーはスレッドプールの join 失敗やパラメータ不正など真の内部障害のみを表す。
#[derive(Debug, thiserror::Error)]
pub enum PasswordHashError {
    #[error("internal password hashing error: {0}")]
    Internal(#[from] anyhow::Error),
}

/// パスワードのハッシュ化・照合を担うポート。
///
/// argon2 のような memory-hard ハッシュは CPU バウンド(数十 ms)なので，
/// [`crate::application::ports::email::Email`] と同様に非同期トレイトとし，
/// 実装側で `spawn_blocking` に逃がす。
#[async_trait::async_trait]
pub trait PasswordHasher: Send + Sync {
    /// 検証済み平文を PHC 文字列へハッシュ化する。
    async fn hash(&self, password: &PlaintextPassword) -> Result<String, PasswordHashError>;

    /// 入力平文 `attempt` を保存済み PHC `phc` と照合する。
    /// 照合は定数時間で行い，PHC が壊れていれば fail-closed に `Mismatch` を返す。
    async fn verify(&self, attempt: &str, phc: &str) -> Result<VerifyOutcome, PasswordHashError>;
}
