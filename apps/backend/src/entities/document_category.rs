use crate::sea_orm_entities;
use crate::sea_orm_entities::document_category::Entity;
use crate::sea_orm_entities::document_category::Model;
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityOrSelect, EntityTrait, NotSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentCategoryWrite {
    pub title: Option<String>,
    pub emoji: Option<Option<String>>,
}

impl DocumentCategoryWrite {
    pub fn into_active_model(self, id: Uuid) -> sea_orm_entities::document_category::ActiveModel {
        let title = match self.title {
            Some(title) => Set(title),
            None => NotSet,
        };
        let emoji = match self.emoji {
            Some(emoji) => Set(emoji),
            None => NotSet,
        };
        sea_orm_entities::document_category::ActiveModel {
            id: Set(id),
            created_at: Default::default(),
            updated_at: Default::default(),
            title,
            emoji,
        }
    }

    pub async fn insert(self, id: Uuid, db_conn: &DbConn) -> Result<DocumentCategoryRead, DbErr> {
        let result = self.into_active_model(id).insert(db_conn).await?;
        Ok(result.into())
    }

    pub async fn update(
        self,
        id: Uuid,
        db_conn: &DbConn,
    ) -> Result<Option<DocumentCategoryRead>, DbErr> {
        match self.into_active_model(id).update(db_conn).await {
            Ok(model) => Ok(Some(model.into())),
            Err(DbErr::RecordNotUpdated) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentCategoryRead {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub emoji: Option<String>,
}

impl From<Model> for DocumentCategoryRead {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at.unwrap().to_utc(),
            updated_at: value.updated_at.unwrap().to_utc(),
            title: value.title,
            emoji: value.emoji,
        }
    }
}

impl DocumentCategoryRead {
    pub async fn get_all(db_conn: &DbConn) -> Result<Vec<DocumentCategoryRead>, DbErr> {
        let models = Entity.select().all(db_conn).await?;
        let mut result = Vec::new();
        for model in models {
            result.push(model.into());
        }
        Ok(result)
    }

    pub async fn find_by_id(
        id: Uuid,
        db_conn: &DbConn,
    ) -> Result<Option<DocumentCategoryRead>, DbErr> {
        let result = Entity::find_by_id(id).one(db_conn).await?;
        Ok(result.map(|model| model.into()))
    }
}

pub async fn delete_document_category(id: Uuid, db_conn: &DbConn) -> Result<u64, DbErr> {
    Ok(Entity::delete_by_id(id).exec(db_conn).await?.rows_affected)
}
