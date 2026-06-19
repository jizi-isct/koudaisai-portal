//! 本番用の [`AccessTokenIssuer`]。短命アクセストークン(JWT)を発行する。
//!
//! `sid`(セッション=ファミリ id)をクレームに含め，失効の即時反映や監査に使う。
//! 検証はミドルウェアが行う(alg/iss/aud/exp/typ を厳密化)。本型は発行のみ。

use crate::application::ports::access_token_issuer::AccessTokenIssuer;
use crate::domain::session_id::SessionId;
use crate::domain::user_id::UserId;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 自前アクセストークンのクレーム。ミドルウェアの検証でも使う。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub iss: String,
    pub sub: Uuid,
    pub sid: Uuid,
    pub exp: i64,
    pub iat: i64,
    /// トークンクラス。アクセストークンは固定文字列 `"access_token"`。
    pub typ: String,
}

pub const ACCESS_TOKEN_TYP: &str = "access_token";

pub struct JwtAccessTokenIssuer {
    algorithm: Algorithm,
    iss: String,
    encoding_key: EncodingKey,
}

impl JwtAccessTokenIssuer {
    pub fn new(algorithm: Algorithm, iss: impl Into<String>, encoding_key: EncodingKey) -> Self {
        Self {
            algorithm,
            iss: iss.into(),
            encoding_key,
        }
    }

    /// 設定から RS256・iss・署名鍵(RSA 秘密鍵)を読んで構築する。
    pub fn from_config(
        auth: &crate::config::Auth,
        auth_v3: &crate::config::AuthV3,
    ) -> anyhow::Result<Self> {
        let encoding_key = auth.get_jwt_encoding_key()?;
        Ok(Self::new(
            Algorithm::RS256,
            auth_v3.access_token_iss.clone(),
            encoding_key,
        ))
    }
}

impl AccessTokenIssuer for JwtAccessTokenIssuer {
    fn issue(&self, user_id: UserId, sid: SessionId, ttl: Duration) -> anyhow::Result<String> {
        let now = Utc::now().timestamp();
        let claims = AccessClaims {
            iss: self.iss.clone(),
            sub: Uuid::from(user_id),
            sid: Uuid::from(sid),
            exp: now + ttl.num_seconds(),
            iat: now,
            typ: ACCESS_TOKEN_TYP.to_string(),
        };
        Ok(jsonwebtoken::encode(
            &Header::new(self.algorithm),
            &claims,
            &self.encoding_key,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{DecodingKey, Validation};

    #[test]
    fn issues_token_decodable_with_expected_claims() {
        let key = b"unit-test-secret";
        let issuer =
            JwtAccessTokenIssuer::new(Algorithm::HS256, "koudaisai", EncodingKey::from_secret(key));
        let uid = UserId::new(Uuid::new_v4());
        let sid = SessionId::new(Uuid::new_v4());

        let token = issuer.issue(uid, sid, Duration::minutes(5)).unwrap();

        let mut v = Validation::new(Algorithm::HS256);
        v.validate_aud = false;
        let data =
            jsonwebtoken::decode::<AccessClaims>(&token, &DecodingKey::from_secret(key), &v).unwrap();
        assert_eq!(data.claims.sub, Uuid::from(uid));
        assert_eq!(data.claims.sid, Uuid::from(sid));
        assert_eq!(data.claims.typ, ACCESS_TOKEN_TYP);
        assert_eq!(data.claims.iss, "koudaisai");
        assert!(data.claims.exp > data.claims.iat);
    }
}
