use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// A wrapper type for sensitive information to prevent accidental logging of secrets.
///
/// This type implements Display trait to mask the actual content with "***" when
/// the value is formatted for output, ensuring that sensitive data like passwords,
/// tokens, or keys are not accidentally leaked into logs or error messages.
///
/// # Example
/// ```
/// use crate::util::Secret;
///
/// let password = Secret::String("sensitive_password".to_string());
/// println!("{}", password); // Outputs: "***"
/// ```
pub enum Secret {
    String(String),
}

impl Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Secret::String(_) => write!(f, "***"),
        }
    }
}

impl Default for Secret {
    fn default() -> Self {
        Secret::String("".to_string())
    }
}

impl Into<String> for Secret {
    fn into(self) -> String {
        match self {
            Secret::String(s) => s,
        }
    }
}

impl AsRef<str> for Secret {
    fn as_ref(&self) -> &str {
        match self {
            Secret::String(s) => s.as_str(),
        }
    }
}
