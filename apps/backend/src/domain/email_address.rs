use crate::domain::error::FactoryError;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct EmailAddress {
    pub address: String,
}

impl EmailAddress {
    pub fn new(address: String) -> Result<Self, FactoryError> {
        let r = r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$";
        let regex = Regex::new(r).unwrap();
        if !regex.is_match(&*address) {
            return Err(FactoryError::InvalidInput(
                "Invalid email address".to_string(),
            ));
        }

        Ok(Self { address })
    }
}
