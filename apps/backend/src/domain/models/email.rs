use regex::Regex;
use crate::domain::models::error::FactoryError;

#[derive(Debug, Clone, PartialEq)]
pub struct Email {
    pub address: String,
}

impl Email {
    pub fn new(address: String) -> Result<Self, FactoryError> {
        let r = r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$";
        let regex = Regex::new(r).unwrap();
        if !regex.is_match(&*address) {

            return Err(FactoryError::InvalidInput("Invalid email address".to_string()));
        }

        Ok(Self { address })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::new("test@example.com".to_string());
        assert!(email.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let email = Email::new("invalid_email".to_string());
        assert!(email.is_err());
    }
}
