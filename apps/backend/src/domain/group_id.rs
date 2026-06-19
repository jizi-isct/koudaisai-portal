use crate::domain::error::FactoryError;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct GroupId {
    prefix: char,
    index: u16,
}

// `"G-001"` 形式の文字列として serde 表現する（FromStr の検証を通すため derive せず手実装）．
impl Serialize for GroupId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for GroupId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        GroupId::from_str(&s).map_err(D::Error::custom)
    }
}

impl GroupId {
    pub fn new(prefix: char, index: u16) -> Result<Self, FactoryError> {
        if index >= 1000 {
            return Err(FactoryError::InvalidInput(
                "Index must be less than 1000".to_string(),
            ));
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

impl FromStr for GroupId {
    type Err = FactoryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(FactoryError::InvalidInput("Invalid format".to_string()));
        }
        let prefix = parts[0]
            .chars()
            .next()
            .ok_or(FactoryError::InvalidInput("Invalid format".to_string()))?;
        let index_str = parts[1];
        let index = index_str
            .parse::<u16>()
            .map_err(|_| FactoryError::InvalidInput("Invalid format".to_string()))?;
        GroupId::new(prefix, index)
            .map_err(|_| FactoryError::InvalidInput("Invalid format".to_string()))
    }
}

impl Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.prefix(), self.index_str())
    }
}
