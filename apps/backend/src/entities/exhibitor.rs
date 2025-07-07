use crate::sea_orm_entities;
use crate::util::IntoActiveValue;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExhibitionType {
    Booth,
    General,
    Stage,
    Labo,
}

impl Display for ExhibitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ExhibitionType::Booth => "booth",
            ExhibitionType::General => "general",
            ExhibitionType::Stage => "stage",
            ExhibitionType::Labo => "labo",
        })
    }
}

impl From<sea_orm_entities::sea_orm_active_enums::ExhibitionType> for ExhibitionType {
    fn from(e: sea_orm_entities::sea_orm_active_enums::ExhibitionType) -> Self {
        match e {
            sea_orm_entities::sea_orm_active_enums::ExhibitionType::Booth => ExhibitionType::Booth,
            sea_orm_entities::sea_orm_active_enums::ExhibitionType::General => {
                ExhibitionType::General
            }
            sea_orm_entities::sea_orm_active_enums::ExhibitionType::Stage => ExhibitionType::Stage,
            sea_orm_entities::sea_orm_active_enums::ExhibitionType::Labo => ExhibitionType::Labo,
        }
    }
}

pub struct ExhibitorRead {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub exhibitor_name: String,
    pub exhibition_name: Option<String>,
    pub icon_id: Option<String>,
    pub description: Option<String>,
    pub r#type: ExhibitionType,
    pub representatives: (Uuid, Uuid, Uuid),
}

impl ExhibitorRead {
    pub async fn from(model: sea_orm_entities::exhibitors_root::Model) -> Self {
        ExhibitorRead {
            id: model.id,
            created_at: model.created_at.unwrap().to_utc(),
            updated_at: model.created_at.unwrap().to_utc(),
            exhibitor_name: model.exhibitor_name,
            exhibition_name: model.exhibition_name,
            icon_id: model.icon_id,
            description: model.description,
            r#type: model.r#type.into(),
            representatives: (
                model.representative1.unwrap(),
                model.representative2.unwrap(),
                model.representative3.unwrap(),
            ),
        }
    }

    pub async fn find_from_id<T: Into<String>>(
        id: T,
        db_conn: &DbConn,
    ) -> Result<Option<Self>, DbErr> {
        match sea_orm_entities::exhibitors_root::Entity::find_by_id(id)
            .one(db_conn)
            .await?
        {
            Some(model) => Ok(Some(Self::from(model).await)),
            None => Ok(None),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ExhibitorUpdate {
    #[serde(default)]
    pub(crate) exhibition_name: Option<Option<String>>,
    #[serde(default)]
    pub(crate) icon_id: Option<Option<String>>,
    #[serde(default)]
    pub(crate) description: Option<Option<String>>,
}

impl ExhibitorUpdate {
    pub fn into_active_model(self, id: String) -> sea_orm_entities::exhibitors_root::ActiveModel {
        sea_orm_entities::exhibitors_root::ActiveModel {
            id: Set(id),
            exhibition_name: self.exhibition_name.into_active_value(),
            icon_id: self.icon_id.into_active_value(),
            description: self.description.into_active_value(),
            ..Default::default()
        }
    }

    pub async fn update(
        self,
        id: String,
        db_conn: &DbConn,
    ) -> Result<sea_orm_entities::exhibitors_root::Model, DbErr> {
        let active_model = self.into_active_model(id);
        let updated_exhibitor = active_model.update(db_conn).await?;
        Ok(updated_exhibitor)
    }
}
