//! テスト用の決定的 [`SecretGenerator`]。CSPRNG を使わず連番で値を払い出す。
//! 回転値や reuse を厳密にアサートできるようにする。

use crate::application::ports::secret_generator::SecretGenerator;
use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

pub struct MemorySecretGenerator {
    counter: AtomicU64,
}

impl MemorySecretGenerator {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(1),
        }
    }

    fn next(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::SeqCst)
    }
}

impl Default for MemorySecretGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretGenerator for MemorySecretGenerator {
    fn generate_secret(&self) -> String {
        format!("secret-{}", self.next())
    }

    fn hash_secret(&self, secret: &str) -> String {
        format!("h:{secret}")
    }

    fn verify_secret(&self, secret: &str, hash: &str) -> bool {
        self.hash_secret(secret) == hash
    }

    fn new_id(&self) -> Uuid {
        Uuid::from_u128(self.next() as u128)
    }
}
