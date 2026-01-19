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
