use regex::Regex;
use crate::domain::models::error::FactoryError;

// 小文字・大文字・数字・記号８文字以上
const PASSWORD_REGEX: &str = r#"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[~`!@#$%^&*()_+-={}[|;:'",<.>/?])(?=.{8,})"#;

pub struct Password {
    pub password: String,
}

impl Password {
    /// Password オブジェクトを生成．パスワード要件を満たさない場合はFactoryError::InvalidInputを返す
    /// ## 引数
    /// - `password` - パスワード文字列
    pub fn new(password: String) -> Result<Self, FactoryError> {
        let regex = Regex::new(PASSWORD_REGEX).unwrap();
        if !regex.is_match(password.as_str()) {
            return Err(FactoryError::InvalidInput("Invalid password".to_string()));
        }

        Ok(Password { password })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let password = Password::new("Password1!".to_string());
        assert!(password.is_ok());
    }

    #[test]
    fn test_invalid_password_no_lowercase() {
        let password = Password::new("PASSWORD1!".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_invalid_password_no_uppercase() {
        let password = Password::new("password1!".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_invalid_password_no_number() {
        let password = Password::new("Password!".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_invalid_password_no_special() {
        let password = Password::new("Password1".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_invalid_password_too_short() {
        let password = Password::new("Pass1!".to_string());
        assert!(password.is_err());
    }
}

