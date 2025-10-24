use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone)]
pub enum GroupId {
    String(String),
    Us,
}

impl Serialize for GroupId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            GroupId::String(string) => serializer.serialize_str(string),
            GroupId::Us => serializer.serialize_str("us"),
        }
    }
}

impl<'de> Deserialize<'de> for GroupId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserIdVisitor;

        impl<'de> serde::de::Visitor<'de> for UserIdVisitor {
            type Value = GroupId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a user id (id or 'us')")
            }

            fn visit_str<E>(self, value: &str) -> Result<GroupId, E>
            where
                E: serde::de::Error,
            {
                if value == "us" {
                    return Ok(GroupId::Us);
                }

                Ok(GroupId::String(value.to_string()))
            }
        }

        deserializer.deserialize_str(UserIdVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize() {
        // Test String variant
        let id = "M-001";
        let group_id = GroupId::String(id.to_string());
        let serialized = serde_json::to_string(&group_id).unwrap();
        assert_eq!(serialized, "\"M-001\"");

        // Test Us variant
        let group_id = GroupId::Us;
        let serialized = serde_json::to_string(&group_id).unwrap();
        assert_eq!(serialized, "\"us\"");
    }

    #[test]
    fn test_deserialize() {
        // Test String variant
        let id = "M-001";
        let deserialized: GroupId =
            serde_json::from_str("\"M-001\"").unwrap();
        match deserialized {
            GroupId::String(string) => assert_eq!(id, string),
            _ => panic!("Expected String variant"),
        }

        // Test Me variant
        let deserialized: GroupId = serde_json::from_str("\"us\"").unwrap();
        match deserialized {
            GroupId::Us => {}
            _ => panic!("Expected Us variant"),
        }
    }
}
