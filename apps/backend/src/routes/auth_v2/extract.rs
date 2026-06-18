//! `ActorContext` 抽出子。api_v3 リソース系ハンドラが `actor: Actor` で認証コンテキストを得る。
//!
//! iss で分岐する:
//! - Authorization 無し → `ActorContext::NoLogin`
//! - 自前 access JWT(iss == access_iss)→ 厳格検証 + iat/Deactivated ゲート →
//!   `build_actor_context` で `ActorContext::User`
//! - その他(Keycloak)→ admin。**ただし role→claims 抽出は未設計のため現状 401**(下記 TODO)

use super::AuthV2State;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::user::UserStatus;
use crate::domain::user_id::UserId;
use crate::infra::jwt_access_token_issuer::{ACCESS_TOKEN_TYP, AccessClaims};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderMap, StatusCode, header};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde_json::Value;

/// 認証コンテキストの抽出子。
pub struct Actor(pub ActorContext);

/// `Authorization: Bearer <token>` を取り出す。
pub(crate) fn bearer(headers: &HeaderMap) -> Option<String> {
    let raw = headers.get(header::AUTHORIZATION)?.to_str().ok()?;
    raw.strip_prefix("Bearer ").map(|t| t.to_string())
}

/// 署名検証前に iss だけを非検証で覗く(分岐用。分岐後に必ず厳格検証する)。
fn peek_iss(token: &str) -> Option<String> {
    let payload_b64 = token.split('.').nth(1)?;
    let bytes = base64url::decode(payload_b64).ok()?;
    let payload: Value = serde_json::from_slice(&bytes).ok()?;
    payload.get("iss")?.as_str().map(|s| s.to_string())
}

/// 自前アクセストークン(RS256)を厳格に検証する。
/// alg=RS256 固定、iss/exp を必須検証、typ=="access_token" を要求する。
pub(crate) fn verify_access(
    token: &str,
    key: &DecodingKey,
    iss: &str,
) -> Result<AccessClaims, StatusCode> {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[iss]);
    validation.set_required_spec_claims(&["exp", "iss"]);
    validation.validate_exp = true;
    validation.validate_aud = false;
    let data =
        decode::<AccessClaims>(token, key, &validation).map_err(|_| StatusCode::UNAUTHORIZED)?;
    if data.claims.typ != ACCESS_TOKEN_TYP {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(data.claims)
}

impl FromRequestParts<AuthV2State> for Actor {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        st: &AuthV2State,
    ) -> Result<Self, Self::Rejection> {
        let Some(token) = bearer(&parts.headers) else {
            // トークン無しは未ログインとして通す(authz 側で弾く)。
            return Ok(Actor(ActorContext::NoLogin));
        };
        let iss = peek_iss(&token).ok_or(StatusCode::UNAUTHORIZED)?;

        if iss == st.access_iss {
            // 自前 access JWT → Group user。
            let claims = verify_access(&token, &st.access_decoding_key, &st.access_iss)?;
            let user_id = UserId::new(claims.sub);
            // iat / Deactivated ゲート。
            let user = st
                .app
                .find_user_for_auth(user_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::UNAUTHORIZED)?;
            match user.status() {
                UserStatus::Active {
                    password_credentials,
                } if claims.iat >= password_credentials.changed_at().timestamp() => {}
                _ => return Err(StatusCode::UNAUTHORIZED),
            }
            let actor = st
                .app
                .build_actor_context(user_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::UNAUTHORIZED)?;
            Ok(Actor(actor))
        } else {
            // TODO(admin): Keycloak access token の realm_access.roles から
            // ActorContext::Admin の claims を組み立てる(JWKS 検証 + role→claim マッピング)。
            // 現行 OIDC は EmptyAdditionalClaims で userinfo から role を取れないため設計が要る。
            // 確定まで admin の api_v3 アクセスは 401。
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
