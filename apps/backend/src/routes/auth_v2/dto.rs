use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub m_address: String,
    pub password: String,
}

/// ログイン/リフレッシュ成功時のレスポンス。リフレッシュトークンは本文に含めず
/// `__Host-` Cookie で配送する。アクセストークンのみ本文で返す。
#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: &'static str,
    /// アクセストークンの有効秒数。
    pub expires_in: i64,
}

#[derive(Deserialize)]
pub struct ActivateRequest {
    /// 不透明な有効化トークン `"{id}.{secret}"`。
    pub token: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct PasswordResetRequest {
    pub m_address: String,
}

#[derive(Deserialize)]
pub struct PasswordResetConfirmRequest {
    /// 不透明なリセットトークン `"{id}.{secret}"`。
    pub token: String,
    pub new_password: String,
}
