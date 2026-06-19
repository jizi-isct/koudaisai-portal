use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub m_address: String,
    pub password: String,
}

/// ログイン/リフレッシュ成功時のレスポンス。リフレッシュトークンは本文に含めず
/// `__Host-` Cookie で配送する。アクセストークンのみ本文で返す。
#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    /// アクセストークンの有効秒数。
    pub expires_in: i64,
}

#[derive(Deserialize, ToSchema)]
pub struct ActivateRequest {
    /// 不透明な有効化トークン `"{id}.{secret}"`。
    pub token: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PasswordResetRequest {
    pub m_address: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PasswordResetConfirmRequest {
    /// 不透明なリセットトークン `"{id}.{secret}"`。
    pub token: String,
    pub new_password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}
