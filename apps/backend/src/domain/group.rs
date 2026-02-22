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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupType {
    Press {
        representative: UserId,
    },
    GeneralProject {
        representative1: UserId,
        representative2: UserId,
        representative3: UserId,
    },
    BoothProject {
        representative1: UserId,
        representative2: UserId,
        representative3: UserId,
    },
    LabProject {
        representative: UserId,
    },
    StageProject {
        representative1: UserId,
        representative2: UserId,
        representative3: UserId,
    },
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

    fn setup_group(clock: &MockClock) -> Group {
        let group_id = GroupId::new('A', 1).unwrap();
        let name = "Test Group".to_string();
        let user_id = UserId::new(Uuid::new_v4());
        let group_type = GroupType::Press {
            representative: user_id,
        };
        Group::register(group_id, name, group_type, clock).unwrap()
    }

    #[test]
    fn test_register_success() {
        let clock = MockClock { now: Utc::now() };
        let group_id = GroupId::new('A', 1).unwrap();
        let name = "Test Group".to_string();
        let user_id = UserId::new(Uuid::new_v4());
        let group_type = GroupType::Press {
            representative: user_id,
        };

        let group = Group::register(group_id, name.clone(), group_type.clone(), &clock).unwrap();

        assert_eq!(group.id(), group_id);
        assert_eq!(group.name(), name);
        assert_eq!(group.r#type(), &group_type);
        assert_eq!(group.created_at(), clock.now);
        assert_eq!(group.updated_at(), clock.now);
    }

    #[test]
    fn test_register_empty_name() {
        let clock = MockClock { now: Utc::now() };
        let group_id = GroupId::new('A', 1).unwrap();
        let name = "  ".to_string();
        let user_id = UserId::new(Uuid::new_v4());
        let group_type = GroupType::Press {
            representative: user_id,
        };

        let result = Group::register(group_id, name, group_type, &clock);
        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
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

    #[test]
    fn test_rename_success() {
        let initial_time = Utc::now();
        let clock = MockClock { now: initial_time };
        let mut group = setup_group(&clock);

        let new_name = "New Group Name".to_string();
        let update_time = initial_time + chrono::Duration::seconds(10);
        let clock_updated = MockClock { now: update_time };

        let result = group.rename(new_name.clone(), &clock_updated);
        assert!(result.is_ok());
        assert_eq!(group.name(), new_name);
        assert_eq!(group.updated_at(), update_time);
        assert_eq!(group.created_at(), initial_time);
    }

    #[test]
    fn test_rename_empty_name() {
        let clock = MockClock { now: Utc::now() };
        let mut group = setup_group(&clock);

        let result = group.rename("  ".to_string(), &clock);
        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
    }

    #[test]
    fn test_update_roles_success() {
        let initial_time = Utc::now();
        let clock = MockClock { now: initial_time };
        let mut group = setup_group(&clock);

        let new_representative = UserId::new(Uuid::new_v4());
        let new_type = GroupType::Press {
            representative: new_representative,
        };
        let membership = Membership::new(group.id(), UserId::new(Uuid::new_v4()), &clock);

        let update_time = initial_time + chrono::Duration::seconds(20);
        let clock_updated = MockClock { now: update_time };

        let result = group.update_roles(new_type.clone(), &clock_updated, &membership);
        assert!(result.is_ok());
        assert_eq!(group.r#type(), &new_type);
        assert_eq!(group.updated_at(), update_time);
    }

    #[test]
    fn test_update_roles_invalid_transition() {
        let clock = MockClock { now: Utc::now() };
        let mut group = setup_group(&clock);

        let new_type = GroupType::GeneralProject {
            representative1: UserId::new(Uuid::new_v4()),
            representative2: UserId::new(Uuid::new_v4()),
            representative3: UserId::new(Uuid::new_v4()),
        };
        let membership = Membership::new(group.id(), UserId::new(Uuid::new_v4()), &clock);

        let result = group.update_roles(new_type, &clock, &membership);
        assert!(matches!(result, Err(UpdateRolesError::InvalidTransition)));
    }

    #[test]
    fn test_update_roles_invalid_membership_group_id() {
        let clock = MockClock { now: Utc::now() };
        let mut group = setup_group(&clock);

        let new_type = group.r#type().clone();
        let other_group_id = GroupId::new('B', 2).unwrap();
        let membership = Membership::new(other_group_id, UserId::new(Uuid::new_v4()), &clock);

        let result = group.update_roles(new_type, &clock, &membership);
        assert!(matches!(result, Err(UpdateRolesError::InvalidInput(_))));
    }
}
