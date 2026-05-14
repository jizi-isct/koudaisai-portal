use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum UserId {
    Uuid(Uuid),
    Me,
}

impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            UserId::Uuid(uuid) => serializer.serialize_str(&uuid.to_string()),
            UserId::Me => serializer.serialize_str("me"),
        }
    }
}

impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserIdVisitor;

        impl<'de> serde::de::Visitor<'de> for UserIdVisitor {
            type Value = UserId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a user id (UUID or 'me')")
            }

            fn visit_str<E>(self, value: &str) -> Result<UserId, E>
            where
                E: serde::de::Error,
            {
                if value == "me" {
                    return Ok(UserId::Me);
                }

                match Uuid::parse_str(value) {
                    Ok(uuid) => Ok(UserId::Uuid(uuid)),
                    Err(_) => Err(E::custom(format!("Invalid UUID: {}", value))),
                }
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
        // Test Uuid variant
        let uuid = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
        let user_id = UserId::Uuid(uuid);
        let serialized = serde_json::to_string(&user_id).unwrap();
        assert_eq!(serialized, "\"12345678-1234-1234-1234-123456789012\"");

        // Test Me variant
        let user_id = UserId::Me;
        let serialized = serde_json::to_string(&user_id).unwrap();
        assert_eq!(serialized, "\"me\"");
    }

    #[test]
    fn test_deserialize() {
        // Test Uuid variant
        let uuid = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
        let deserialized: UserId =
            serde_json::from_str("\"12345678-1234-1234-1234-123456789012\"").unwrap();
        match deserialized {
            UserId::Uuid(id) => assert_eq!(id, uuid),
            _ => panic!("Expected Uuid variant"),
        }

        // Test Me variant
        let deserialized: UserId = serde_json::from_str("\"me\"").unwrap();
        match deserialized {
            UserId::Me => {}
            _ => panic!("Expected Me variant"),
        }

        // Test invalid format
        let result = serde_json::from_str::<UserId>("\"invalid-uuid-format\"");
        assert!(result.is_err());
    }
}
