use crate::sea_orm_entities;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityOrSelect, EntityTrait, NotSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DocumentFormat {
    FormatMarkdown { content: String },
    FormatPdf { file_key: String },
    FormatMisc { file_key: String },
}

pub enum DocumentWriteActiveModel {
    Markdown(
        sea_orm_entities::document::ActiveModel,
        sea_orm_entities::document_format_markdown::ActiveModel,
    ),
    Pdf(
        sea_orm_entities::document::ActiveModel,
        sea_orm_entities::document_format_pdf::ActiveModel,
    ),
    Misc(
        sea_orm_entities::document::ActiveModel,
        sea_orm_entities::document_format_misc::ActiveModel,
    ),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentCreate {
    pub title: String,
    pub category: Uuid,
    #[serde(flatten)]
    pub format: DocumentFormat,
    pub required_one_of_scopes: Vec<String>,
}

impl DocumentCreate {
    pub fn into_active_model(self, created_by: Uuid) -> DocumentWriteActiveModel {
        let format = match self.format {
            DocumentFormat::FormatMarkdown { .. } => {
                sea_orm_entities::sea_orm_active_enums::DocumentFormat::Markdown
            }
            DocumentFormat::FormatPdf { .. } => {
                sea_orm_entities::sea_orm_active_enums::DocumentFormat::Pdf
            }
            DocumentFormat::FormatMisc { .. } => {
                sea_orm_entities::sea_orm_active_enums::DocumentFormat::Misc
            }
        };

        let id = Uuid::new_v4();
        let generic = sea_orm_entities::document::ActiveModel {
            id: Set(id.clone()),
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: Set(created_by.clone()),
            updated_by: Set(created_by),
            title: Set(self.title),
            format: Set(format),
            category: Set(Some(self.category)),
            required_one_of_scopes: Set(self.required_one_of_scopes),
        };

        match self.format {
            DocumentFormat::FormatMarkdown { content } => DocumentWriteActiveModel::Markdown(
                generic,
                sea_orm_entities::document_format_markdown::ActiveModel {
                    id: Set(id),
                    content: Set(content),
                },
            ),
            DocumentFormat::FormatPdf { file_key } => DocumentWriteActiveModel::Pdf(
                generic,
                sea_orm_entities::document_format_pdf::ActiveModel {
                    id: Set(id),
                    file_key: Set(file_key),
                },
            ),
            DocumentFormat::FormatMisc { file_key } => DocumentWriteActiveModel::Misc(
                generic,
                sea_orm_entities::document_format_misc::ActiveModel {
                    id: Set(id),
                    file_key: Set(file_key),
                },
            ),
        }
    }

    pub async fn insert(self, created_by: Uuid, db_conn: &DbConn) -> Result<DocumentRead, DbErr> {
        Ok(match self.into_active_model(created_by) {
            DocumentWriteActiveModel::Markdown(generic, markdown) => DocumentRead::from_markdown(
                generic.insert(db_conn).await?,
                markdown.insert(db_conn).await?,
            ),
            DocumentWriteActiveModel::Pdf(generic, pdf) => {
                DocumentRead::from_pdf(generic.insert(db_conn).await?, pdf.insert(db_conn).await?)
            }
            DocumentWriteActiveModel::Misc(generic, misc) => {
                DocumentRead::from_misc(generic.insert(db_conn).await?, misc.insert(db_conn).await?)
            }
        })
    }

    pub async fn update(
        self,
        id: Uuid,
        created_by: Uuid,
        db_conn: &DbConn,
    ) -> Result<DocumentRead, DbErr> {
        Ok(match self.into_active_model(created_by) {
            DocumentWriteActiveModel::Markdown(mut generic, mut markdown) => {
                generic.id = Set(id);
                markdown.id = Set(id);
                DocumentRead::from_markdown(
                    generic.update(db_conn).await?,
                    markdown.update(db_conn).await?,
                )
            }
            DocumentWriteActiveModel::Pdf(mut generic, mut pdf) => {
                generic.id = Set(id);
                pdf.id = Set(id);
                DocumentRead::from_pdf(generic.update(db_conn).await?, pdf.update(db_conn).await?)
            }
            DocumentWriteActiveModel::Misc(mut generic, mut misc) => {
                generic.id = Set(id);
                misc.id = Set(id);
                DocumentRead::from_misc(generic.update(db_conn).await?, misc.update(db_conn).await?)
            }
        })
    }
}
pub enum DocumentUpdateActiveModel {
    Markdown(
        sea_orm_entities::document::ActiveModel,
        sea_orm_entities::document_format_markdown::ActiveModel,
    ),
    Pdf(
        sea_orm_entities::document::ActiveModel,
        sea_orm_entities::document_format_pdf::ActiveModel,
    ),
    Misc(
        sea_orm_entities::document::ActiveModel,
        sea_orm_entities::document_format_misc::ActiveModel,
    ),
    Generic(sea_orm_entities::document::ActiveModel),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentUpdate {
    pub updated_by: Uuid,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub category: Option<Option<Uuid>>,
    #[serde(flatten)]
    #[serde(default)]
    pub format: Option<DocumentFormat>,
    #[serde(default)]
    pub required_one_of_scopes: Option<Vec<String>>,
}

impl DocumentUpdate {
    fn into_active_model(self, id: Uuid) -> DocumentUpdateActiveModel {
        let updated_by = Set(self.updated_by);
        let title = match self.title {
            None => NotSet,
            Some(title) => Set(title),
        };
        let format = match self.format {
            None => NotSet,
            Some(DocumentFormat::FormatMarkdown { .. }) => {
                Set(sea_orm_entities::sea_orm_active_enums::DocumentFormat::Markdown)
            }
            Some(DocumentFormat::FormatPdf { .. }) => {
                Set(sea_orm_entities::sea_orm_active_enums::DocumentFormat::Pdf)
            }
            Some(DocumentFormat::FormatMisc { .. }) => {
                Set(sea_orm_entities::sea_orm_active_enums::DocumentFormat::Misc)
            }
        };
        let category = match self.category {
            None => NotSet,
            Some(category) => Set(category),
        };
        let required_one_of_scopes = match self.required_one_of_scopes {
            None => NotSet,
            Some(required_one_of_scopes) => Set(required_one_of_scopes),
        };

        let generic = sea_orm_entities::document::ActiveModel {
            id: Set(id.clone()),
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: Default::default(),
            updated_by,
            title,
            format,
            category,
            required_one_of_scopes,
        };

        match self.format {
            Some(DocumentFormat::FormatMarkdown { content }) => {
                DocumentUpdateActiveModel::Markdown(
                    generic,
                    sea_orm_entities::document_format_markdown::ActiveModel {
                        id: Set(id),
                        content: Set(content),
                    },
                )
            }
            Some(DocumentFormat::FormatPdf { file_key }) => DocumentUpdateActiveModel::Pdf(
                generic,
                sea_orm_entities::document_format_pdf::ActiveModel {
                    id: Set(id),
                    file_key: Set(file_key),
                },
            ),
            Some(DocumentFormat::FormatMisc { file_key }) => DocumentUpdateActiveModel::Misc(
                generic,
                sea_orm_entities::document_format_misc::ActiveModel {
                    id: Set(id),
                    file_key: Set(file_key),
                },
            ),
            None => DocumentUpdateActiveModel::Generic(generic),
        }
    }

    pub async fn update(self, id: Uuid, db_conn: &DbConn) -> Result<DocumentRead> {
        Ok(match self.into_active_model(id) {
            DocumentUpdateActiveModel::Markdown(mut generic, mut markdown) => {
                DocumentRead::from_markdown(
                    generic.update(db_conn).await?,
                    markdown.update(db_conn).await?,
                )
            }
            DocumentUpdateActiveModel::Pdf(mut generic, mut pdf) => {
                DocumentRead::from_pdf(generic.update(db_conn).await?, pdf.update(db_conn).await?)
            }
            DocumentUpdateActiveModel::Misc(mut generic, mut misc) => {
                DocumentRead::from_misc(generic.update(db_conn).await?, misc.update(db_conn).await?)
            }
            DocumentUpdateActiveModel::Generic(generic) => {
                DocumentRead::from(generic.update(db_conn).await?, db_conn).await?
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
                file_key: pdf.file_key,
            },
            required_one_of_scopes: generic.required_one_of_scopes,
        }
    }

    pub fn from_misc(
        generic: sea_orm_entities::document::Model,
        misc: sea_orm_entities::document_format_misc::Model,
    ) -> DocumentRead {
        DocumentRead {
            id: generic.id,
            created_at: generic.created_at.unwrap().to_utc(),
            updated_at: generic.updated_at.unwrap().to_utc(),
            created_by: generic.created_by,
            updated_by: generic.updated_by,
            title: generic.title,
            category: generic.category.unwrap(),
            format: DocumentFormat::FormatMisc {
                file_key: misc.file_key,
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
            sea_orm_entities::sea_orm_active_enums::DocumentFormat::Misc => {
                let misc = sea_orm_entities::document_format_misc::Entity::find_by_id(value.id)
                    .one(db_conn)
                    .await?
                    .ok_or(anyhow::anyhow!("Document not found"))?;

                Ok(Self::from_misc(value, misc))
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

    pub async fn find_from_required_one_of_scopes(
        scope: &String,
        db_conn: &DbConn,
    ) -> Result<Vec<Self>> {
        let all_documents = Self::get_all(db_conn).await?;
        let mut documents = vec![];

        for document in all_documents {
            if document.required_one_of_scopes.contains(scope) {
                documents.push(document);
            }
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

pub async fn delete_document(id: Uuid, db_conn: &DbConn) -> Result<u64, DbErr> {
    let result = sea_orm_entities::document::Entity::delete_by_id(id)
        .exec(db_conn)
        .await?;

    Ok(result.rows_affected)
}
