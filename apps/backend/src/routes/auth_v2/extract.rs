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
use oauth2::AccessToken;
use openidconnect::core::CoreUserInfoClaims;
use serde_json::Value;
use uuid::Uuid;

/// legacy 踏襲: Keycloak 認証済み admin に付与する全権限 claim。
/// (legacy は `matches!(current_user, CurrentUser::Admin(_))` で細粒度判定をしていなかった)
fn admin_claims() -> Vec<String> {
    [
        "koudaisai-portal:admin:user:read",
        "koudaisai-portal:admin:user:create",
        "koudaisai-portal:admin:user:update",
        "koudaisai-portal:admin:user:change-email",
        "koudaisai-portal:admin:user:delete",
        "koudaisai-portal:admin:group:read",
        "koudaisai-portal:admin:group:create",
        "koudaisai-portal:admin:group:update",
        "koudaisai-portal:admin:group:delete",
        "koudaisai-portal:admin:form:read",
        "koudaisai-portal:admin:form:create",
        "koudaisai-portal:admin:form:update",
        "koudaisai-portal:admin:form:delete",
        "koudaisai-portal:admin:document:read",
        "koudaisai-portal:admin:document:create",
        "koudaisai-portal:admin:document:update",
        "koudaisai-portal:admin:document:delete",
        "koudaisai-portal:admin:document-category:read",
        "koudaisai-portal:admin:document-category:create",
        "koudaisai-portal:admin:document-category:update",
        "koudaisai-portal:admin:document-category:delete",
        "koudaisai-portal:admin:notification:read",
        "koudaisai-portal:admin:notification:create",
        "koudaisai-portal:admin:notification:update",
        "koudaisai-portal:admin:notification:delete",
        "koudaisai-portal:admin:approval-request:read",
        "koudaisai-portal:admin:approval-request:approve",
        "koudaisai-portal:admin:approval-request:delete",
        "koudaisai-portal:admin:events26-project:create",
        "koudaisai-portal:admin:events26-project:update",
        "koudaisai-portal:admin:events26-project:delete",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

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

impl FromRequestParts<AuthV2State> for ActorContext {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        st: &AuthV2State,
    ) -> Result<Self, Self::Rejection> {
        let Some(token) = bearer(&parts.headers) else {
            // トークン無しは未ログインとして通す(authz 側で弾く)。
            return Ok(ActorContext::NoLogin);
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
            Ok(actor)
        } else {
            // Keycloak admin。legacy は「Keycloak 認証成功＝フル管理者」(細粒度 claim 判定なし)
            // だったため踏襲し、userinfo で正当性を確認した上で全 admin claim を付与する。
            // (将来 Keycloak realm_access.roles → 個別 claim の細粒度マッピングに差し替え可能)
            let access_token = AccessToken::new(token);
            let user_info_req = st
                .oidc_client
                .user_info(access_token, None)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let user_info: CoreUserInfoClaims = user_info_req
                .request_async(&st.http_client)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
            let subject = user_info.subject().as_str();
            let user_id =
                UserId::new(Uuid::parse_str(subject).map_err(|_| StatusCode::UNAUTHORIZED)?);
            let name = user_info
                .preferred_username()
                .map(|u| u.as_str().to_string())
                .unwrap_or_else(|| subject.to_string());
            Ok(ActorContext::Admin {
                user_id,
                name,
                claims: admin_claims(),
            })
        }
    }
}
