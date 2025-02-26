use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum AccountType {
    User = 0,
    Admin = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iss: String,
    pub typ: AccountType,
}

const ALGORITHM: Algorithm = Algorithm::RS256;
pub const ACCESS_TOKEN_EXPIRE_TIME: usize = 600; // 10 minutes
pub const REFRESH_TOKEN_EXPIRE_TIME: usize = 60 * 60 * 24 * 30 * 6; // 6 months
pub fn encode(claims: &Claims, key: &EncodingKey) -> jsonwebtoken::errors::Result<String> {
    jsonwebtoken::encode(&Header::new(ALGORITHM), claims, key)
}

pub fn decode(token: &str, key: &DecodingKey) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    jsonwebtoken::decode::<Claims>(token, &*key, &Validation::new(ALGORITHM))
}
