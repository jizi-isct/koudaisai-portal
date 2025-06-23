use crate::entities::target_specifier::TargetSpecifier;
use crate::sea_orm_entities;
use crate::util::IntoActiveValue;
use chrono::{DateTime, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DbConn, EntityTrait, NotSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod builtin;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FormType {
    TypeExternal { form_url: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormCreate {
    pub targets: Vec<TargetSpecifier>,
    pub form_name: String,
    #[serde(flatten)]
    pub form_type: FormType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormRead {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub targets: Vec<TargetSpecifier>,
    pub form_name: String,
    #[serde(flatten)]
    pub form_type: FormType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormUpdate {
    #[serde(default)]
    pub targets: Option<Vec<TargetSpecifier>>,
    #[serde(default)]
    pub form_name: Option<String>,
    #[serde(flatten, default)]
    pub form_type: Option<FormType>,
}

enum ActiveModelFormCreate {
    External(
        sea_orm_entities::form::ActiveModel,
        sea_orm_entities::form_type_external::ActiveModel,
    ),
}

enum ActiveModelFormUpdate {
    External(
        sea_orm_entities::form::ActiveModel,
        sea_orm_entities::form_type_external::ActiveModel,
    ),
    Generic(sea_orm_entities::form::ActiveModel),
}

enum ModelFormRead {
    External(
        sea_orm_entities::form::Model,
        sea_orm_entities::form_type_external::Model,
    ),
}

impl FormCreate {
    pub fn into_active_model(self, created_by: Option<Uuid>) -> ActiveModelFormCreate {
        let id = Set(Uuid::new_v4());
        let form = sea_orm_entities::form::ActiveModel {
            id,
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            created_by: Set(created_by),
            updated_by: Set(created_by),
            targets: Set(self.targets.iter().map(|t| t.into()).collect()),
            form_name: Set(self.form_name),
            ..Default::default()
        };

        match self.form_type {
            FormType::TypeExternal { form_url } => ActiveModelFormCreate::External(
                form,
                sea_orm_entities::form_type_external::ActiveModel {
                    form_url: Set(form_url),
                    ..Default::default()
                },
            ),
        }
    }

    pub async fn insert(self, db_conn: &DbConn, created_by: Option<Uuid>) -> anyhow::Result<()> {
        match self.into_active_model(created_by) {
            ActiveModelFormCreate::External(form, external) => {
                sea_orm_entities::form::Entity::insert(form)
                    .exec(db_conn)
                    .await?;
                sea_orm_entities::form_type_external::Entity::insert(external)
                    .exec(db_conn)
                    .await?;
            }
        }

        Ok(())
    }
}

impl FormUpdate {
    pub fn into_active_model(self, id: Uuid, updated_by: Option<Uuid>) -> ActiveModelFormUpdate {
        let mut targets = NotSet;
        if let Some(targets_) = self.targets {
            targets = Set(targets_.iter().map(|t| t.into()).collect());
        }
        let form = sea_orm_entities::form::ActiveModel {
            id: Set(id),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_by: Set(updated_by),
            targets,
            form_name: self.form_name.into_active_value(),
            ..Default::default()
        };

        match self.form_type {
            Some(FormType::TypeExternal { form_url }) => ActiveModelFormUpdate::External(
                form,
                sea_orm_entities::form_type_external::ActiveModel {
                    form_url: Set(form_url),
                    ..Default::default()
                },
            ),
            None => ActiveModelFormUpdate::Generic(form),
        }
    }

    pub async fn update(
        self,
        db_conn: &DbConn,
        id: Uuid,
        updated_by: Option<Uuid>,
    ) -> anyhow::Result<()> {
        match self.into_active_model(id, updated_by) {
            ActiveModelFormUpdate::External(form, external) => {
                sea_orm_entities::form::Entity::update(form)
                    .exec(db_conn)
                    .await?;
                sea_orm_entities::form_type_external::Entity::update(external)
                    .exec(db_conn)
                    .await?;
            }
            ActiveModelFormUpdate::Generic(form) => {
                sea_orm_entities::form::Entity::update(form)
                    .exec(db_conn)
                    .await?;
            }
        }

        Ok(())
    }
}

impl FormRead {
    pub async fn from_model(model: ModelFormRead) -> anyhow::Result<Self> {
        match model {
            ModelFormRead::External(form, external) => Ok(Self {
                id: form.id,
                created_at: DateTime::from(form.created_at),
                updated_at: DateTime::from(form.updated_at),
                created_by: form.created_by,
                updated_by: form.updated_by,
                targets: form
                    .targets
                    .iter()
                    .map(|t| TargetSpecifier::from_string(t))
                    .collect(),
                form_name: form.form_name,
                form_type: FormType::TypeExternal {
                    form_url: external.form_url,
                },
            }),
        }
    }

    pub async fn find_by_id(id: Uuid, db_conn: &DbConn) -> anyhow::Result<Option<Self>> {
        let form = sea_orm_entities::form::Entity::find_by_id(id)
            .one(db_conn)
            .await?
            .ok_or_else(|| anyhow::anyhow!("form not found"))?;

        let external = sea_orm_entities::form_type_external::Entity::find()
            .filter(sea_orm_entities::form_type_external::Column::FormId.eq(id))
            .one(db_conn)
            .await?
            .ok_or_else(|| anyhow::anyhow!("external form type not found"))?;

        Ok(Some(
            Self::from_model(ModelFormRead::External(form, external)).await?,
        ))
    }

    pub async fn find_all(db_conn: &DbConn) -> anyhow::Result<Vec<Self>> {
        let forms = sea_orm_entities::form::Entity::find().all(db_conn).await?;

        let mut result = Vec::new();
        for form in forms {
            let external = sea_orm_entities::form_type_external::Entity::find()
                .filter(sea_orm_entities::form_type_external::Column::FormId.eq(form.id))
                .one(db_conn)
                .await?
                .ok_or_else(|| anyhow::anyhow!("external form type not found"))?;

            result.push(Self::from_model(ModelFormRead::External(form, external)).await?);
        }

        Ok(result)
    }
}

pub async fn delete_form_by_id(id: Uuid, db_conn: &DbConn) -> anyhow::Result<u64> {
    let form = sea_orm_entities::form::Entity::delete_by_id(id)
        .exec(db_conn)
        .await?;

    Ok(form.rows_affected)
}
