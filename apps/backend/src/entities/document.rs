use crate::entities::target_specifier::TargetSpecifier;
use crate::sea_orm_entities;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityOrSelect, EntityTrait, NotSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DocumentFormat {
    FormatMarkdown { content: String },
    FormatPdf { file_key: String, file_name: String },
    FormatMisc { file_key: String, file_name: String },
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
    pub category: Option<Uuid>,
    #[serde(flatten)]
    pub format: DocumentFormat,
    pub targets: Vec<TargetSpecifier>,
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
            category: Set(self.category),
            targets: Set(self.targets.iter().map(|t| t.into()).collect()),
        };

        match self.format {
            DocumentFormat::FormatMarkdown { content } => DocumentWriteActiveModel::Markdown(
                generic,
                sea_orm_entities::document_format_markdown::ActiveModel {
                    id: Set(id),
                    content: Set(content),
                },
            ),
            DocumentFormat::FormatPdf {
                file_key,
                file_name,
            } => DocumentWriteActiveModel::Pdf(
                generic,
                sea_orm_entities::document_format_pdf::ActiveModel {
                    id: Set(id),
                    file_key: Set(file_key),
                    file_name: Set(file_name),
                },
            ),
            DocumentFormat::FormatMisc {
                file_key,
                file_name,
            } => DocumentWriteActiveModel::Misc(
                generic,
                sea_orm_entities::document_format_misc::ActiveModel {
                    id: Set(id),
                    file_key: Set(file_key),
                    file_name: Set(file_name),
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
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub category: Option<Option<Uuid>>,
    #[serde(flatten)]
    #[serde(default)]
    pub format: Option<DocumentFormat>,
    #[serde(default)]
    pub targets: Option<Vec<TargetSpecifier>>,
}

impl DocumentUpdate {
    /// Converts the DocumentUpdate into an ActiveModel for updating the document.
    /// # Arguments
    /// * `id` - The ID of the document to update.
    /// * `updated_by` - The ID of the user who is updating the document.
    fn into_active_model(self, id: Uuid, updated_by: Uuid) -> DocumentUpdateActiveModel {
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
        let targets = match self.targets {
            None => NotSet,
            Some(targets) => Set(targets.iter().map(|t| t.into()).collect()),
        };

        let generic = sea_orm_entities::document::ActiveModel {
            id: Set(id.clone()),
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: Default::default(),
            updated_by: Set(updated_by),
            title,
            format,
            category,
            targets,
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
            Some(DocumentFormat::FormatPdf {
                file_key,
                file_name,
            }) => DocumentUpdateActiveModel::Pdf(
                generic,
                sea_orm_entities::document_format_pdf::ActiveModel {
                    id: Set(id),
                    file_key: Set(file_key),
                    file_name: Set(file_name),
                },
            ),
            Some(DocumentFormat::FormatMisc {
                file_key,
                file_name,
            }) => DocumentUpdateActiveModel::Misc(
                generic,
                sea_orm_entities::document_format_misc::ActiveModel {
                    id: Set(id),
                    file_key: Set(file_key),
                    file_name: Set(file_name),
                },
            ),
            None => DocumentUpdateActiveModel::Generic(generic),
        }
    }

    /// Updates the document in the database.
    /// # Arguments
    /// * `id` - The ID of the document to update.
    /// * `updated_by` - The ID of the user who is updating the document.
    /// * `db_conn` - The database connection to use for the update.
    pub async fn update(
        self,
        id: Uuid,
        updated_by: Uuid,
        db_conn: &DbConn,
    ) -> Result<DocumentRead> {
        Ok(match self.into_active_model(id, updated_by) {
            DocumentUpdateActiveModel::Markdown(generic, markdown) => DocumentRead::from_markdown(
                generic.update(db_conn).await?,
                markdown.update(db_conn).await?,
            ),
            DocumentUpdateActiveModel::Pdf(generic, pdf) => {
                DocumentRead::from_pdf(generic.update(db_conn).await?, pdf.update(db_conn).await?)
            }
            DocumentUpdateActiveModel::Misc(generic, misc) => {
                DocumentRead::from_misc(generic.update(db_conn).await?, misc.update(db_conn).await?)
            }
            DocumentUpdateActiveModel::Generic(generic) => {
                DocumentRead::from(generic.update(db_conn).await?, db_conn).await?
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentRead {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub title: String,
    pub category: Option<Uuid>,
    #[serde(flatten)]
    pub format: DocumentFormat,
    pub targets: Vec<TargetSpecifier>,
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
            category: generic.category,
            format: DocumentFormat::FormatMarkdown {
                content: markdown.content,
            },
            targets: generic
                .targets
                .iter()
                .map(|t| TargetSpecifier::from_string(t))
                .collect(),
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
            category: generic.category,
            format: DocumentFormat::FormatPdf {
                file_key: pdf.file_key,
                file_name: pdf.file_name,
            },
            targets: generic
                .targets
                .iter()
                .map(|t| TargetSpecifier::from_string(t))
                .collect(),
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
            category: generic.category,
            format: DocumentFormat::FormatMisc {
                file_key: misc.file_key,
                file_name: misc.file_name,
            },
            targets: generic
                .targets
                .iter()
                .map(|t| TargetSpecifier::from_string(t))
                .collect(),
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
}

pub async fn delete_document(id: Uuid, db_conn: &DbConn) -> Result<u64, DbErr> {
    let result = sea_orm_entities::document::Entity::delete_by_id(id)
        .exec(db_conn)
        .await?;

    Ok(result.rows_affected)
}
