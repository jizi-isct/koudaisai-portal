use crate::application::ports::clock::Clock;
use crate::domain::error::FactoryError;
use crate::domain::group_id::GroupId;
use crate::domain::membership::Membership;
use crate::domain::user_id::UserId;
use chrono::{DateTime, Utc};
use std::mem::discriminant;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateRolesError {
    #[error("Invalid transition")]
    InvalidTransition,
    #[error("Invalid input")]
    InvalidInput(String),
}

/// 団体の種類と団体固有のフィールドを持つ列挙型です．
/// 各団体の種類の詳細については委員内ドキュメント(JIZI Wikiなど)を参照してください．
/// 主な固有フィールドとしてメンバーのロールがありますが，団体への所属自体は `Membership` を用いて表現します．
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupType {
    /// 学内取材団体．一人の代表者を持つ．
    Press { representative: UserId },
    /// 一般企画団体．第一責任者から第三責任者まで計３名のメンバーを持つ．
    /// 第一責任者から第三責任者には別のメンバーが割り当てられる必要がある．
    GeneralProject {
        representative1: UserId,
        representative2: UserId,
        representative3: UserId,
    },
    /// 模擬店企画団体．第一責任者から第三責任者まで計３名のメンバーを持つ．
    /// 第一責任者から第三責任者には別のメンバーが割り当てられる必要がある．
    BoothProject {
        representative1: UserId,
        representative2: UserId,
        representative3: UserId,
    },
    /// 研究室企画団体．企画責任者と企画実施担当者を持つ．
    /// 企画責任者と企画実施担当者は兼任可能(同一のユーザーIDを代入可能)．
    LabProject {
        representative: UserId,
        operator: UserId,
    },
    /// ステージ企画団体．第一責任者から第三責任者まで計３名のメンバーを持つ．
    /// 第一責任者から第三責任者には別のメンバーが割り当てられる必要がある．
    StageProject {
        representative1: UserId,
        representative2: UserId,
        representative3: UserId,
    },
}

impl GroupType {
    /// GroupTypeのフィールドが正しく設定されているかを検証します．
    ///
    /// # Errors
    /// - `FactoryError::InvalidInput` - 責任者の重複など，不正な入力が行われた場合に発生します．
    fn validate(&self) -> Result<(), FactoryError> {
        match self {
            GroupType::GeneralProject {
                representative1,
                representative2,
                representative3,
            }
            | GroupType::BoothProject {
                representative1,
                representative2,
                representative3,
            }
            | GroupType::StageProject {
                representative1,
                representative2,
                representative3,
            } => {
                if representative1 == representative2
                    || representative1 == representative3
                    || representative2 == representative3
                {
                    return Err(FactoryError::InvalidInput(
                        "Representatives must all be different".to_string(),
                    ));
                }
                Ok(())
            }
            GroupType::Press { .. } | GroupType::LabProject { .. } => Ok(()),
        }
    }
}

pub struct Group {
    id: GroupId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    r#type: GroupType,
}

impl Group {
    pub fn register<C: Clock>(
        id: GroupId,
        name: String,
        r#type: GroupType,
        clock: &C,
    ) -> Result<Self, FactoryError> {
        if name.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Name is empty".to_string()));
        }
        r#type.validate()?;

        Ok(Self {
            id,
            created_at: clock.now(),
            updated_at: clock.now(),
            name,
            r#type,
        })
    }

    pub fn restore(
        id: GroupId,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        name: String,
        r#type: GroupType,
    ) -> Result<Self, FactoryError> {
        if name.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Name is empty".to_string()));
        }

        Ok(Self {
            id,
            created_at,
            updated_at,
            name,
            r#type,
        })
    }

    pub fn id(&self) -> GroupId {
        self.id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rename<C: Clock>(&mut self, name: String, clock: &C) -> Result<(), FactoryError> {
        if name.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Name is empty".to_string()));
        }
        self.name = name;
        self.updated_at = clock.now();
        Ok(())
    }

    pub fn r#type(&self) -> &GroupType {
        &self.r#type
    }

    /// ユーザーの役職を変更します
    ///
    /// # Params
    /// - `r#type` - 新たな役職を含んだGroupType
    /// - `clock` - 時計ポート
    /// - `membership` - 役職を変更するユーザーのMembership．ユーザーがグループに所属していることの証明として使用されます．
    ///
    /// # Errors
    /// - `UpdateRolesError::InvalidTransition` - 異なるGroupTypeへ遷移しようとした場合に発生します．
    /// - `UpdateRolesError::InvalidInput` - 不正な入力が行われた場合に発生します．
    pub fn update_roles<C: Clock>(
        &mut self,
        r#type: GroupType,
        clock: &C,
        membership: &Membership,
    ) -> Result<(), UpdateRolesError> {
        if discriminant(&r#type) != discriminant(&self.r#type) {
            return Err(UpdateRolesError::InvalidTransition);
        }

        if membership.group_id() != self.id {
            return Err(UpdateRolesError::InvalidInput(format!(
                "Membership group id {} does not match group id {}",
                membership.group_id(),
                self.id
            )));
        }

        r#type
            .validate()
            .map_err(|e| UpdateRolesError::InvalidInput(e.to_string()))?;

        self.r#type = r#type;
        self.updated_at = clock.now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::group_id::GroupId;
    use crate::domain::membership::Membership;
    use crate::domain::user_id::UserId;
    use uuid::Uuid;

    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    // --- GroupType バリデーション ---

    #[test]
    fn test_general_project_all_unique_reps_succeeds() {
        let clock = MockClock { now: Utc::now() };
        let group_id = GroupId::new('A', 1).unwrap();
        let group_type = GroupType::GeneralProject {
            representative1: UserId::new(Uuid::new_v4()),
            representative2: UserId::new(Uuid::new_v4()),
            representative3: UserId::new(Uuid::new_v4()),
        };
        let result = Group::register(group_id, "G".to_string(), group_type, &clock);
        assert!(result.is_ok());
    }

    #[test]
    fn test_general_project_duplicate_reps_fails() {
        let clock = MockClock { now: Utc::now() };
        let group_id = GroupId::new('A', 1).unwrap();
        let shared = UserId::new(Uuid::new_v4());
        let group_type = GroupType::GeneralProject {
            representative1: shared,
            representative2: shared,
            representative3: UserId::new(Uuid::new_v4()),
        };
        let result = Group::register(group_id, "G".to_string(), group_type, &clock);
        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
    }

    #[test]
    fn test_booth_project_duplicate_reps_fails() {
        let clock = MockClock { now: Utc::now() };
        let group_id = GroupId::new('A', 1).unwrap();
        let shared = UserId::new(Uuid::new_v4());
        let group_type = GroupType::BoothProject {
            representative1: shared,
            representative2: shared,
            representative3: UserId::new(Uuid::new_v4()),
        };
        let result = Group::register(group_id, "G".to_string(), group_type, &clock);
        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
    }

    #[test]
    fn test_stage_project_duplicate_reps_fails() {
        let clock = MockClock { now: Utc::now() };
        let group_id = GroupId::new('A', 1).unwrap();
        let shared = UserId::new(Uuid::new_v4());
        let group_type = GroupType::StageProject {
            representative1: shared,
            representative2: shared,
            representative3: UserId::new(Uuid::new_v4()),
        };
        let result = Group::register(group_id, "G".to_string(), group_type, &clock);
        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
    }

    #[test]
    fn test_lab_project_same_representative_and_operator_succeeds() {
        let clock = MockClock { now: Utc::now() };
        let group_id = GroupId::new('A', 1).unwrap();
        let shared = UserId::new(Uuid::new_v4());
        let group_type = GroupType::LabProject {
            representative: shared,
            operator: shared,
        };
        let result = Group::register(group_id, "G".to_string(), group_type, &clock);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lab_project_different_representative_and_operator_succeeds() {
        let clock = MockClock { now: Utc::now() };
        let group_id = GroupId::new('A', 1).unwrap();
        let group_type = GroupType::LabProject {
            representative: UserId::new(Uuid::new_v4()),
            operator: UserId::new(Uuid::new_v4()),
        };
        let result = Group::register(group_id, "G".to_string(), group_type, &clock);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_roles_validates_duplicate_reps() {
        let clock = MockClock { now: Utc::now() };
        let group_id = GroupId::new('A', 1).unwrap();
        let group_type = GroupType::GeneralProject {
            representative1: UserId::new(Uuid::new_v4()),
            representative2: UserId::new(Uuid::new_v4()),
            representative3: UserId::new(Uuid::new_v4()),
        };
        let mut group = Group::register(group_id, "G".to_string(), group_type, &clock).unwrap();

        let shared = UserId::new(Uuid::new_v4());
        let invalid_type = GroupType::GeneralProject {
            representative1: shared,
            representative2: shared,
            representative3: UserId::new(Uuid::new_v4()),
        };
        let membership = Membership::new(group_id, UserId::new(Uuid::new_v4()), &clock);
        let result = group.update_roles(invalid_type, &clock, &membership);
        assert!(matches!(result, Err(UpdateRolesError::InvalidInput(_))));
    }

    #[test]
    fn test_restore_success() {
        let group_id = GroupId::new('A', 1).unwrap();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let name = "Restored Group".to_string();
        let user_id = UserId::new(Uuid::new_v4());
        let group_type = GroupType::Press {
            representative: user_id,
        };

        let group = Group::restore(
            group_id,
            created_at,
            updated_at,
            name.clone(),
            group_type.clone(),
        )
        .unwrap();

        assert_eq!(group.id(), group_id);
        assert_eq!(group.name(), name);
        assert_eq!(group.r#type(), &group_type);
        assert_eq!(group.created_at(), created_at);
        assert_eq!(group.updated_at(), updated_at);
    }
}
