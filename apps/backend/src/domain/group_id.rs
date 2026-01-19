use std::fmt::Display;
use crate::domain::error::FactoryError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct GroupId {
    prefix: char,
    index: u16
}

impl GroupId {
    pub fn new(prefix: char, index: u16) -> Result<Self, FactoryError> {
        if index >= 1000 {
            return Err(FactoryError::InvalidInput("Index must be less than 1000".to_string()));
        }

        Ok(Self { prefix, index })
    }

    pub fn prefix(&self) -> char {
        self.prefix
    }

    pub fn index(&self) -> u16 {
        self.index
    }

    pub fn index_str(&self) -> String {
        format!("{:03}", self.index)
    }
}

impl Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.prefix(), self.index_str())
    }
}
