use crate::entities::exhibitor::ExhibitorRead;
use crate::sea_orm_entities;
use crate::util::jwt::Claims;
use anyhow::{anyhow, Result};
use chrono::DateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRead {
    pub id: Uuid,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub first_name: String,
    pub last_name: String,
    pub m_address: String,
    pub exhibition_id: String,
}

impl From<sea_orm_entities::users::Model> for UserRead {
    fn from(value: sea_orm_entities::users::Model) -> Self {
        UserRead {
            id: value.id,
            created_at: value.created_at.unwrap().to_utc(),
            updated_at: value.updated_at.unwrap().to_utc(),
            first_name: value.first_name,
            last_name: value.last_name,
            m_address: value.m_address,
            exhibition_id: value.exhibition_id,
        }
    }
}

impl UserRead {
    pub async fn from(value: Claims, db_conn: &DbConn) -> Result<Self> {
        match Self::find_from_id(value.sub, db_conn).await? {
            Some(value) => Ok(value),
            None => Err(anyhow::anyhow!("User not found")),
        }
    }

    pub async fn from_claims(claims: Claims, db_conn: &DbConn) -> Result<Self> {
        Ok(Self::find_from_id(claims.sub, db_conn)
            .await?
            .ok_or(anyhow!("User not found"))?)
    }

    pub async fn find_from_id(value: Uuid, db_conn: &DbConn) -> Result<Option<Self>> {
        match sea_orm_entities::users::Entity::find_by_id(value)
            .one(db_conn)
            .await?
        {
            Some(value) => Ok(Some(value.into())),
            None => Ok(None),
        }
    }

    pub async fn get_all(db_conn: &DbConn) -> Result<Vec<Self>> {
        let users = sea_orm_entities::users::Entity::find().all(db_conn).await?;
        Ok(users.into_iter().map(Into::into).collect())
    }

    pub async fn get_exhibitor_read(&self, db_conn: &DbConn) -> Result<ExhibitorRead> {
        match ExhibitorRead::find_from_id(&self.exhibition_id, db_conn).await? {
            Some(value) => Ok(value),
            None => Err(anyhow::anyhow!("Exhibitor not found")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserUpdate {
    pub first_name: String,
    pub last_name: String,
    pub m_address: String,
}

impl Into<sea_orm_entities::users::ActiveModel> for UserUpdate {
    fn into(self) -> sea_orm_entities::users::ActiveModel {
        sea_orm_entities::users::ActiveModel {
            id: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            first_name: Set(self.first_name),
            last_name: Set(self.last_name),
            m_address: Set(self.m_address),
            password_hash: Default::default(),
            password_salt: Default::default(),
            exhibition_id: Default::default(),
        }
    }
}

impl UserUpdate {
    pub async fn update(
        self,
        user_id: Uuid,
        db_conn: &DbConn,
    ) -> Result<sea_orm_entities::users::Model> {
        let mut active_model: sea_orm_entities::users::ActiveModel = self.into();
        active_model.id = Set(user_id);

        let updated_user = active_model.update(db_conn).await?;
        Ok(updated_user)
    }
}
