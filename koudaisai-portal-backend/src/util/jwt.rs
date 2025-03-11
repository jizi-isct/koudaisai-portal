use crate::entities::revoked_refresh_tokens;
use anyhow::Result;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub iss: String,
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub typ: Type,
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    AccessToken,
    RefreshToken,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Type::AccessToken => String::from("access_token"),
            Type::RefreshToken => String::from("refresh_token"),
        };
        write!(f, "{}", str)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tokens {
    refresh_token: String,
    access_token: String,
}

pub struct JWTManager {
    pub algorithm: Algorithm,
    pub access_token_expire_time: i64,
    pub refresh_token_expire_time: i64,
    pub iss: String,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    db_conn: DatabaseConnection,
}

impl JWTManager {
    pub fn new(
        algorithm: Algorithm,
        access_token_expire_time: i64,
        refresh_token_expire_time: i64,
        iss: impl ToString,
        encoding_key: EncodingKey,
        decoding_key: DecodingKey,
        db_conn: DatabaseConnection,
    ) -> Self {
        Self {
            algorithm,
            access_token_expire_time,
            refresh_token_expire_time,
            iss: iss.to_string(),
            encoding_key,
            decoding_key,
            db_conn,
        }
    }

    pub fn encode(&self, claims: &Claims) -> Result<String> {
        Ok(jsonwebtoken::encode(
            &Header::new(self.algorithm),
            claims,
            &self.encoding_key,
        )?)
    }

    pub fn decode(&self, token: &str) -> Result<TokenData<Claims>> {
        Ok(jsonwebtoken::decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::new(self.algorithm),
        )?)
    }

    pub fn issue_tokens(&self, sub: Uuid) -> Result<Tokens> {
        return Ok(Tokens {
            refresh_token: self.issue_refresh_token(sub.clone())?,
            access_token: self.issue_access_token(sub.clone())?,
        });
    }
    pub fn issue_refresh_token(&self, sub: Uuid) -> Result<String> {
        let refresh_token_claims = Claims {
            iss: self.iss.clone(),
            sub,
            exp: Utc::now().timestamp() + self.refresh_token_expire_time,
            iat: Utc::now().timestamp(),
            typ: Type::RefreshToken,
        };
        let refresh_token = self.encode(&refresh_token_claims)?;
        Ok(refresh_token)
    }
    pub fn issue_access_token(&self, sub: Uuid) -> Result<String> {
        let access_token_claims = Claims {
            iss: self.iss.clone(),
            sub,
            exp: Utc::now().timestamp() + self.access_token_expire_time,
            iat: Utc::now().timestamp(),
            typ: Type::AccessToken,
        };
        let access_token = self.encode(&access_token_claims)?;

        Ok(access_token)
    }

    /// 以下の条件がすべて満たされる場合true、それ以外はfalse
    /// - `claims.typ`が`refresh_token`である。
    /// - 有効期限が切れていない
    /// - revokeされていない
    pub async fn is_refresh_token_valid(&self, token: String, claims: &Claims) -> Result<bool> {
        // typ検証
        if claims.typ != Type::RefreshToken {
            return Ok(false);
        }

        // exp検証
        if claims.exp > Utc::now().timestamp() {
            return Ok(false);
        }

        // revoke検証
        if revoked_refresh_tokens::Entity::find_by_id(token)
            .one(&self.db_conn)
            .await?
            != None
        {
            return Ok(false);
        }

        Ok(true)
    }

    /// 以下の条件がすべて満たされる場合true、それ以外はfalse
    /// - `claims.typ`が`Access_token`である。
    /// - 有効期限が切れていない
    pub fn is_access_token_valid(&self, claims: &Claims) -> bool {
        // typ検証
        if claims.typ != Type::AccessToken {
            return false;
        }

        // exp検証
        if claims.exp < Utc::now().timestamp() {
            return false;
        }

        true
    }
}
