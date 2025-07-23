use crate::entities::group::plan::PlanTypeRead;
use crate::entities::group::GroupTypeRead;
use crate::entities::user::UserRead;
use sea_orm::DbConn;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TargetSpecifier {
    GroupTypePlanGeneral,
    GroupTypePlanBooth,
    GroupTypePlanStage,
    GroupTypePlanLabo,
    GroupTypePress,
    GroupId(String),
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
            TargetSpecifier::GroupTypePlanGeneral => "group/type/plan_general".into(),
            TargetSpecifier::GroupTypePlanBooth => "group/type/plan_booth".into(),
            TargetSpecifier::GroupTypePlanStage => "group/type/plan_stage".into(),
            TargetSpecifier::GroupTypePlanLabo => "group/type/plan_labo".into(),
            TargetSpecifier::GroupTypePress => "group/type/press".into(),
            TargetSpecifier::GroupId(id) => format!("group/id/{}", id),
            TargetSpecifier::UserId(user_id) => format!("user/id/{}", user_id).into(),
            TargetSpecifier::UserNologin => "user/nologin".into(),
            TargetSpecifier::Unknown(s) => s.into(),
        }
    }
}

impl TargetSpecifier {
    pub fn from_string<S: Into<String>>(value: S) -> Self {
        match value.into().as_str() {
            // レガシーなタイプ
            "exhibitor/type/general" => TargetSpecifier::GroupTypePlanGeneral,
            "exhibitor/type/booth" => TargetSpecifier::GroupTypePlanBooth,
            "exhibitor/type/stage" => TargetSpecifier::GroupTypePlanStage,
            "exhibitor/type/labo" => TargetSpecifier::GroupTypePlanLabo,
            s if s.starts_with("exhibitor/id/") => {
                let exhibitor_id = &s[13..];
                TargetSpecifier::GroupId(exhibitor_id.to_owned())
            }

            "group/type/plan_general" => TargetSpecifier::GroupTypePlanGeneral,
            "group/type/plan_booth" => TargetSpecifier::GroupTypePlanBooth,
            "group/type/plan_stage" => TargetSpecifier::GroupTypePlanStage,
            "group/type/plan_labo" => TargetSpecifier::GroupTypePlanLabo,
            "group/type/press" => TargetSpecifier::GroupTypePress,
            "user/nologin" => TargetSpecifier::UserNologin,
            s if s.starts_with("user/id/") => {
                let user_id_str = &s[8..];
                match Uuid::parse_str(user_id_str) {
                    Ok(user_id) => TargetSpecifier::UserId(user_id),
                    Err(_) => TargetSpecifier::Unknown(s.parse().unwrap()),
                }
            }
            s if s.starts_with("group/id/") => {
                let group_id = &s[9..];
                TargetSpecifier::GroupId(group_id.to_owned())
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
            TargetSpecifier::GroupTypePlanGeneral => {
                if let Some(user) = user {
                    let exhibitor = user.get_group_read(db_conn).await?;
                    if let GroupTypeRead::TypePlan(plan) = exhibitor.r#type {
                        if let PlanTypeRead::TypeGeneral(..) = plan.r#type {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            TargetSpecifier::GroupTypePlanBooth => {
                if let Some(user) = user {
                    let exhibitor = user.get_group_read(db_conn).await?;
                    if let GroupTypeRead::TypePlan(plan) = exhibitor.r#type {
                        if let PlanTypeRead::TypeBooth(..) = plan.r#type {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            TargetSpecifier::GroupTypePlanStage => {
                if let Some(user) = user {
                    let exhibitor = user.get_group_read(db_conn).await?;
                    if let GroupTypeRead::TypePlan(plan) = exhibitor.r#type {
                        if let PlanTypeRead::TypeStage(..) = plan.r#type {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            TargetSpecifier::GroupTypePlanLabo => {
                if let Some(user) = user {
                    let exhibitor = user.get_group_read(db_conn).await?;
                    if let GroupTypeRead::TypePlan(plan) = exhibitor.r#type {
                        if let PlanTypeRead::TypeLabo(..) = plan.r#type {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            TargetSpecifier::GroupTypePress => {
                if let Some(user) = user {
                    let exhibitor = user.get_group_read(db_conn).await?;
                    if let GroupTypeRead::TypePress(..) = exhibitor.r#type {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            TargetSpecifier::GroupId(id) => {
                if let Some(user) = user {
                    let group = user.get_group_read(db_conn).await?;
                    Ok(group.id == *id)
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
        // Test GroupTypePlanGeneral
        let target = TargetSpecifier::GroupTypePlanGeneral;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"group/type/plan_general\"");

        // Test GroupTypePlanBooth
        let target = TargetSpecifier::GroupTypePlanBooth;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"group/type/plan_booth\"");

        // Test GroupTypePlanStage
        let target = TargetSpecifier::GroupTypePlanStage;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"group/type/plan_stage\"");

        // Test GroupTypePlanLabo
        let target = TargetSpecifier::GroupTypePlanLabo;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"group/type/plan_labo\"");

        // Test GroupTypePress
        let target = TargetSpecifier::GroupTypePress;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"group/type/press\"");

        // Test UserId
        let user_id = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
        let target = TargetSpecifier::UserId(user_id);
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(
            serialized,
            "\"user/id/12345678-1234-1234-1234-123456789012\""
        );

        // Test GroupId
        let group_id = "T-000";
        let target = TargetSpecifier::GroupId(group_id.to_string());
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"group/id/T-000\"");

        // Test UserNologin
        let target = TargetSpecifier::UserNologin;
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, "\"user/nologin\"");
    }

    #[test]
    fn test_deserialize() {
        // legacy specifiers
        // Test ExhibitorTypeGeneral
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"exhibitor/type/general\"").unwrap();
        assert!(matches!(
            deserialized,
            TargetSpecifier::GroupTypePlanGeneral
        ));

        // Test ExhibitorTypeBooth
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"exhibitor/type/booth\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::GroupTypePlanBooth));

        // Test ExhibitorTypeStage
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"exhibitor/type/stage\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::GroupTypePlanStage));

        // Test ExhibitorTypeLabo
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"exhibitor/type/labo\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::GroupTypePlanLabo));

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

        // Test GroupId
        let group_id = "T-000";
        let deserialized: TargetSpecifier =
            serde_json::from_str(&format!("\"group/id/{}\"", group_id)).unwrap();
        if let TargetSpecifier::GroupId(id) = deserialized {
            assert_eq!(id, group_id);
        } else {
            panic!("Expected GroupId variant");
        }

        // Test GroupTypePress
        let deserialized: TargetSpecifier = serde_json::from_str("\"group/type/press\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::GroupTypePress));
        // Test GroupTypePlanGeneral
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"group/type/plan_general\"").unwrap();
        assert!(matches!(
            deserialized,
            TargetSpecifier::GroupTypePlanGeneral
        ));
        // Test GroupTypePlanBooth
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"group/type/plan_booth\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::GroupTypePlanBooth));
        // Test GroupTypePlanStage
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"group/type/plan_stage\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::GroupTypePlanStage));
        // Test GroupTypePlanLabo
        let deserialized: TargetSpecifier =
            serde_json::from_str("\"group/type/plan_labo\"").unwrap();
        assert!(matches!(deserialized, TargetSpecifier::GroupTypePlanLabo));
    }
}
