use crate::entities::exhibitor::ExhibitionType;
use crate::entities::user::UserRead;
use sea_orm::DbConn;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::any::Any;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TargetSpecifier {
    ExhibitorTypeGeneral,
    ExhibitorTypeBooth,
    ExhibitorTypeStage,
    ExhibitorTypeLabo,
    ExhibitorId(String),
    UserId(Uuid),
    UserNologin,
    Unknown(String),
}

impl Serialize for TargetSpecifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: String = self.into();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for TargetSpecifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TargetSpecifierVisitor;

        impl<'de> serde::de::Visitor<'de> for TargetSpecifierVisitor {
            type Value = TargetSpecifier;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a target specifier")
            }

            fn visit_str<E>(self, value: &str) -> Result<TargetSpecifier, E>
            where
                E: serde::de::Error,
            {
                Ok(TargetSpecifier::from_string(value))
            }
        }

        deserializer.deserialize_str(TargetSpecifierVisitor)
    }
}

impl Into<String> for &TargetSpecifier {
    fn into(self) -> String {
        match self {
            TargetSpecifier::ExhibitorTypeGeneral => "exhibitor/type/general".into(),
            TargetSpecifier::ExhibitorTypeBooth => "exhibitor/type/booth".into(),
            TargetSpecifier::ExhibitorTypeStage => "exhibitor/type/stage".into(),
            TargetSpecifier::ExhibitorTypeLabo => "exhibitor/type/labo".into(),
            TargetSpecifier::ExhibitorId(id) => format!("exhibitor/id/{}", id),
            TargetSpecifier::UserId(user_id) => format!("user/id/{}", user_id).into(),
            TargetSpecifier::UserNologin => "user/nologin".into(),
            TargetSpecifier::Unknown(s) => s.into(),
        }
    }
}

impl TargetSpecifier {
    pub fn from_string<S: Into<String>>(value: S) -> Self {
        match value.into().as_str() {
            "exhibitor/type/general" => TargetSpecifier::ExhibitorTypeGeneral,
            "exhibitor/type/booth" => TargetSpecifier::ExhibitorTypeBooth,
            "exhibitor/type/stage" => TargetSpecifier::ExhibitorTypeStage,
            "exhibitor/type/labo" => TargetSpecifier::ExhibitorTypeLabo,
            "user/nologin" => TargetSpecifier::UserNologin,
            s if s.starts_with("user/id/") => {
                let user_id_str = &s[8..];
                match Uuid::parse_str(user_id_str) {
                    Ok(user_id) => TargetSpecifier::UserId(user_id),
                    Err(_) => TargetSpecifier::Unknown(s.parse().unwrap()),
                }
            }
            val => TargetSpecifier::Unknown(val.parse().unwrap()),
        }
    }

    pub async fn does_user_match(
        &self,
        user: Option<&UserRead>,
        db_conn: &DbConn,
    ) -> anyhow::Result<bool> {
        match self {
            TargetSpecifier::ExhibitorTypeGeneral => {
                if let Some(user) = user {
                    let exhibitor = user.get_exhibitor_read(db_conn).await?;
                    if let ExhibitionType::General { .. } = exhibitor.r#type {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            TargetSpecifier::ExhibitorTypeBooth => {
                if let Some(user) = user {
                    let exhibitor = user.get_exhibitor_read(db_conn).await?;
                    if let ExhibitionType::Booth { .. } = exhibitor.r#type {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            TargetSpecifier::ExhibitorTypeStage => {
                if let Some(user) = user {
                    let exhibitor = user.get_exhibitor_read(db_conn).await?;
                    if let ExhibitionType::Stage { .. } = exhibitor.r#type {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            TargetSpecifier::ExhibitorTypeLabo => {
                if let Some(user) = user {
                    let exhibitor = user.get_exhibitor_read(db_conn).await?;
                    if let ExhibitionType::Labo { .. } = exhibitor.r#type {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            TargetSpecifier::ExhibitorId(id) => {
                if let Some(user) = user {
                    let exhibitor = user.get_exhibitor_read(db_conn).await?;
                    Ok(exhibitor.id == *id)
                } else {
                    Ok(false)
                }
            }
            TargetSpecifier::UserId(uuid) => match user {
                Some(user_read) => Ok(user_read.id == *uuid),
                None => Ok(false),
            },
            TargetSpecifier::UserNologin => Ok(user.is_none()),
            TargetSpecifier::Unknown(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize() {
        // Test ExhibitorTypeGeneral
        let target = TargetSpecifier::ExhibitorTypeGeneral;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"exhibitor/type/general\"");

        // Test ExhibitorTypeBooth
        let target = TargetSpecifier::ExhibitorTypeBooth;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"exhibitor/type/booth\"");

        // Test ExhibitorTypeStage
        let target = TargetSpecifier::ExhibitorTypeStage;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"exhibitor/type/stage\"");

        // Test ExhibitorTypeLabo
        let target = TargetSpecifier::ExhibitorTypeLabo;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"exhibitor/type/labo\"");

        // Test UserId
        let user_id = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
        let target = TargetSpecifier::UserId(user_id);
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(
            serialized,
            "\"user/id/12345678-1234-1234-1234-123456789012\""
        );

        // Test UserNologin
        let target = TargetSpecifier::UserNologin;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"user/nologin\"");
    }

    #[test]
    fn test_deserialize() {
        // Test ExhibitorTypeGeneral
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"exhibitor/type/general\"").unwrap();
        assert!(matches!(
            deserialized,
            TargetSpecifier::ExhibitorTypeGeneral
        ));

        // Test ExhibitorTypeBooth
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"exhibitor/type/booth\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::ExhibitorTypeBooth));

        // Test ExhibitorTypeStage
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"exhibitor/type/stage\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::ExhibitorTypeStage));

        // Test ExhibitorTypeLabo
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"exhibitor/type/labo\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::ExhibitorTypeLabo));

        // Test UserId
        let user_id = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"user/id/12345678-1234-1234-1234-123456789012\"").unwrap();
        if let TargetSpecifier::UserId(id) = deserialized {
            assert_eq!(id, user_id);
        } else {
            panic!("Expected UserId variant");
        }

        // Test UserNologin
        let deserialized: TargetSpecifier = serde_json::from_str("\"user/nologin\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::UserNologin));

        // Test invalid format
        let result = serde_json::from_str::<TargetSpecifier>("\"invalid/format\"");
        assert!(result.is_err());
    }
}
