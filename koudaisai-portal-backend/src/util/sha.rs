use sha2::digest::Update;
use sha2::{Digest, Sha256};

pub struct SHAManager {
    pub(crate) stretch_cost: i32,
}

impl SHAManager {
    pub async fn stretch_with_salt(&self, data: &str, salt: &str) -> String {
        stretch_with_salt(data, salt, self.stretch_cost).await
    }
}

pub async fn stretch_with_salt(data: &str, salt: &str, n: i32) -> String {
    (0..n).fold(data.to_string(), |data, _| digest_with_salt(&*data, salt))
}

pub async fn stretch(data: &str, n: i32) -> String {
    (0..n).fold(data.to_string(), |data, _| digest(&*data))
}

pub fn digest_with_salt(data: &str, salt: &str) -> String {
    hex(Sha256::new()
        .chain(salt)
        .chain(data.as_bytes())
        .finalize()
        .as_slice())
}

pub fn digest(data: &str) -> String {
    hex(Sha256::digest(data.as_bytes()).as_slice())
}

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold("".to_owned(), |s, b| s + &format!("{:x}", b))
}
