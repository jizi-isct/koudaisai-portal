use crate::sea_orm_entities;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityOrSelect, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DocumentFormat {
    FormatMarkdown { content: String },
    FormatPdf { file_url: String },
}

pub enum DocumentActiveModel {
    Markdown(
        sea_orm_entities::document::ActiveModel,
        sea_orm_entities::document_format_markdown::ActiveModel,
    ),
    Pdf(
        sea_orm_entities::document::ActiveModel,
        sea_orm_entities::document_format_pdf::ActiveModel,
    ),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentWrite {
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub title: String,
    pub category: Option<Uuid>,
    #[serde(flatten)]
    pub format: DocumentFormat,
    pub required_one_of_scopes: Vec<String>,
}

impl Into<DocumentActiveModel> for DocumentWrite {
    fn into(self) -> DocumentActiveModel {
        let format = match self.format {
            DocumentFormat::FormatMarkdown { .. } => {
                sea_orm_entities::sea_orm_active_enums::DocumentFormat::Markdown
            }
            DocumentFormat::FormatPdf { .. } => {
                sea_orm_entities::sea_orm_active_enums::DocumentFormat::Pdf
            }
        };

        let id = Uuid::new_v4();
        let generic = sea_orm_entities::document::ActiveModel {
            id: Set(id.clone()),
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: Set(self.created_by),
            updated_by: Set(self.updated_by),
            title: Set(self.title),
            format: Set(format),
            category: Set(self.category),
            required_one_of_scopes: Set(self.required_one_of_scopes),
        };

        match self.format {
            DocumentFormat::FormatMarkdown { content } => DocumentActiveModel::Markdown(
                generic,
                sea_orm_entities::document_format_markdown::ActiveModel {
                    id: Set(id),
                    content: Set(content),
                },
            ),
            DocumentFormat::FormatPdf { file_url } => DocumentActiveModel::Pdf(
                generic,
                sea_orm_entities::document_format_pdf::ActiveModel {
                    id: Set(id),
                    file_url: Set(file_url),
                },
            ),
        }
    }
}

impl DocumentWrite {
    pub async fn insert(self, db_conn: &DbConn) -> Result<DocumentRead, DbErr> {
        Ok(match self.into() {
            DocumentActiveModel::Markdown(generic, markdown) => DocumentRead::from_markdown(
                generic.insert(db_conn).await?,
                markdown.insert(db_conn).await?,
            ),
            DocumentActiveModel::Pdf(generic, pdf) => {
                DocumentRead::from_pdf(generic.insert(db_conn).await?, pdf.insert(db_conn).await?)
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentRead {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub title: String,
    pub category: Uuid,
    #[serde(flatten)]
    pub format: DocumentFormat,
    pub required_one_of_scopes: Vec<String>,
}

impl DocumentRead {
    pub fn from_markdown(
        generic: sea_orm_entities::document::Model,
        markdown: sea_orm_entities::document_format_markdown::Model,
    ) -> Self {
        DocumentRead {
            id: generic.id,
            created_at: generic.created_at.unwrap().to_utc(),
            updated_at: generic.updated_at.unwrap().to_utc(),
            created_by: generic.created_by,
            updated_by: generic.updated_by,
            title: generic.title,
            category: generic.category.unwrap(),
            format: DocumentFormat::FormatMarkdown {
                content: markdown.content,
            },
            required_one_of_scopes: generic.required_one_of_scopes,
        }
    }

    pub fn from_pdf(
        generic: sea_orm_entities::document::Model,
        pdf: sea_orm_entities::document_format_pdf::Model,
    ) -> DocumentRead {
        DocumentRead {
            id: generic.id,
            created_at: generic.created_at.unwrap().to_utc(),
            updated_at: generic.updated_at.unwrap().to_utc(),
            created_by: generic.created_by,
            updated_by: generic.updated_by,
            title: generic.title,
            category: generic.category.unwrap(),
            format: DocumentFormat::FormatPdf {
                file_url: pdf.file_url,
            },
            required_one_of_scopes: generic.required_one_of_scopes,
        }
    }

    pub async fn from(value: sea_orm_entities::document::Model, db_conn: &DbConn) -> Result<Self> {
        match value.format {
            sea_orm_entities::sea_orm_active_enums::DocumentFormat::Markdown => {
                //dbから読み込み
                let markdown =
                    sea_orm_entities::document_format_markdown::Entity::find_by_id(value.id)
                        .one(db_conn)
                        .await?
                        .ok_or(anyhow::anyhow!("Document not found"))?;

                Ok(Self::from_markdown(value, markdown))
            }
            sea_orm_entities::sea_orm_active_enums::DocumentFormat::Pdf => {
                let pdf = sea_orm_entities::document_format_pdf::Entity::find_by_id(value.id)
                    .one(db_conn)
                    .await?
                    .ok_or(anyhow::anyhow!("Document not found"))?;

                Ok(Self::from_pdf(value, pdf))
            }
        }
    }

    pub async fn find_from_id(id: Uuid, db_conn: &DbConn) -> Result<Self> {
        let document = sea_orm_entities::document::Entity::find_by_id(id)
            .one(db_conn)
            .await?
            .ok_or(anyhow::anyhow!("Document not found"))?;

        Self::from(document, db_conn).await
    }

    pub async fn get_all(db_conn: &DbConn) -> Result<Vec<Self>> {
        let models = sea_orm_entities::document::Entity
            .select()
            .all(db_conn)
            .await?;

        let mut documents = Vec::new();
        for model in models {
            documents.push(DocumentRead::from(model, db_conn).await?)
        }
        Ok(documents)
    }

    pub async fn find_from_required_one_of_scopes<T: Into<String>>(
        scope: T,
        db_conn: &DbConn,
    ) -> Result<Vec<Self>> {
        let models = sea_orm_entities::document::Entity
            .select()
            .filter(sea_orm_entities::document::Column::RequiredOneOfScopes.contains(scope))
            .all(db_conn)
            .await?;

        let mut documents = Vec::new();
        for model in models {
            documents.push(DocumentRead::from(model, db_conn).await?)
        }
        Ok(documents)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentCategoryWrite {
    pub title: String,
}

impl Into<sea_orm_entities::document_category::ActiveModel> for DocumentCategoryWrite {
    fn into(self) -> sea_orm_entities::document_category::ActiveModel {
        let id = Uuid::new_v4();
        sea_orm_entities::document_category::ActiveModel {
            id: Set(id),
            created_at: Default::default(),
            updated_at: Default::default(),
            title: Set(self.title),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentCategoryRead {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
}

impl From<sea_orm_entities::document_category::Model> for DocumentCategoryRead {
    fn from(value: sea_orm_entities::document_category::Model) -> Self {
        DocumentCategoryRead {
            id: value.id,
            created_at: value.created_at.unwrap().to_utc(),
            updated_at: value.updated_at.unwrap().to_utc(),
            title: value.title,
        }
    }
}
