//! テスト用の決定的・高速な [`PasswordHasher`]。argon2 を回さない。
//!
//! - `hash(p)` は `"$fake$" + 平文` を返す。
//! - `verify` は `$fake$<X>` に対し `attempt == X` で [`VerifyOutcome::Verified`]。
//! - `$legacy$<X>` に対し `attempt == X` なら [`VerifyOutcome::VerifiedNeedsRehash`]
//!   (透過昇格の経路をテストするため)。
//! - それ以外は [`VerifyOutcome::Mismatch`]。

use crate::application::ports::password_hasher::{
    PasswordHashError, PasswordHasher, VerifyOutcome,
};
use crate::domain::plaintext_password::PlaintextPassword;

#[derive(Default)]
pub struct MemoryPasswordHasher;

impl MemoryPasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl PasswordHasher for MemoryPasswordHasher {
    async fn hash(&self, password: &PlaintextPassword) -> Result<String, PasswordHashError> {
        Ok(format!("$fake${}", password.expose()))
    }

    async fn verify(&self, attempt: &str, phc: &str) -> Result<VerifyOutcome, PasswordHashError> {
        if let Some(stored) = phc.strip_prefix("$fake$") {
            return Ok(if stored == attempt {
                VerifyOutcome::Verified
            } else {
                VerifyOutcome::Mismatch
            });
        }
        if let Some(stored) = phc.strip_prefix("$legacy$") {
            return Ok(if stored == attempt {
                VerifyOutcome::VerifiedNeedsRehash {
                    new_phc: format!("$fake${attempt}"),
                }
            } else {
                VerifyOutcome::Mismatch
            });
        }
        Ok(VerifyOutcome::Mismatch)
    }
}
