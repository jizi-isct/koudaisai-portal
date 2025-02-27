use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub nonce: String,
    pub typ: String,
    pub role: String,
}

pub enum Typ {
    RefreshToken,
    AccessToken,
}

impl Display for Typ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Typ::RefreshToken => String::from("refresh_token"),
            Typ::AccessToken => String::from("access_token"),
        };
        write!(f, "{}", str)
    }
}
pub enum Role {
    User,
    Admin,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Role::User => String::from("user"),
            Role::Admin => String::from("admin"),
        };
        write!(f, "{}", str)
    }
}

const ALGORITHM: Algorithm = Algorithm::RS256;
pub const ACCESS_TOKEN_EXPIRE_TIME: usize = 600; // 10 minutes
pub const REFRESH_TOKEN_EXPIRE_TIME: usize = 60 * 60 * 24 * 30 * 6; // 6 months
pub const JWT_ISS: &str = "https://portal.koudaisai.jp";
pub fn encode(claims: &Claims, key: &EncodingKey) -> jsonwebtoken::errors::Result<String> {
    jsonwebtoken::encode(&Header::new(ALGORITHM), claims, key)
}

pub fn decode(token: &str, key: &DecodingKey) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    jsonwebtoken::decode::<Claims>(token, &*key, &Validation::new(ALGORITHM))
}
