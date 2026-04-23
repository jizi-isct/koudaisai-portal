use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::target_specifier::TargetSpecifier;
use crate::domain::user_id::UserId;

use crate::{application::ports::clock::Clock, domain::error::FactoryError};

pub struct Document {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: UserId,
    updated_by: UserId,
    title: String,
    category: Option<Uuid>,
    targets: Vec<TargetSpecifier>,
    format: DocumentFormat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentFormat {
    Markdown { content: String },
    Pdf { file_key: String, file_name: String },
    Misc { file_key: String, file_name: String },
}

impl Document {
    /// 新規登録用コンストラクタ
    /// タイトル、カテゴリID(optional)、フォーマット、対象、作成者のUserIdを受け取り、現在の日時を作成日時と更新日時に設定。
    pub fn register<C: Clock>(
        title: String,
        category: Option<Uuid>,
        format: DocumentFormat,
        targets: Vec<TargetSpecifier>,
        created_by: UserId,
        clock: &C,
    ) -> Result<Self, FactoryError> {
        if title.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Title is empty".to_string()));
        }

        if targets.is_empty() {
            return Err(FactoryError::InvalidInput(
                "Targets cannot be empty".to_string(),
            ));
        }

        Self::validate_format(&format)?;

        let now = clock.now();

        Ok(Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            created_by: created_by,
            updated_by: created_by,
            format: format,
            title: title,
            category: category,
            targets: targets,
        })
    }

    /// 復元用コンストラクタ
    /// ID、作成日時、更新日時、作成者、更新者、タイトル、カテゴリ(optional)、フォーマット、対象を受け取り、それらをそのままフィールドに設定。
    pub fn restore(
        id: Uuid,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        created_by: UserId,
        updated_by: UserId,
        title: String,
        category: Option<Uuid>,
        format: DocumentFormat,
        targets: Vec<TargetSpecifier>,
    ) -> Result<Self, FactoryError> {
        if title.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Title is empty".to_string()));
        }

        if targets.is_empty() {
            return Err(FactoryError::InvalidInput(
                "Targets cannot be empty".to_string(),
            ));
        }

        Self::validate_format(&format)?; // 参照渡し

        Ok(Self {
            id,
            created_at,
            updated_at,
            created_by,
            updated_by,
            title,
            category,
            format,
            targets,
        })
    }

    fn validate_format(format: &DocumentFormat) -> Result<(), FactoryError> {
        match format {
            DocumentFormat::Markdown { content } => {
                if content.trim().is_empty() {
                    return Err(FactoryError::InvalidInput(
                        "Markdown content is empty".to_string(),
                    ));
                }
            }
            DocumentFormat::Pdf {
                file_key,
                file_name,
            }
            | DocumentFormat::Misc {
                file_key,
                file_name,
            } => {
                if file_key.trim().is_empty() {
                    return Err(FactoryError::InvalidInput("File key is empty".to_string()));
                }
                if file_name.trim().is_empty() {
                    return Err(FactoryError::InvalidInput("File name is empty".to_string()));
                }
            }
        }

        Ok(())
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn created_by(&self) -> UserId {
        self.created_by
    }

    pub fn updated_by(&self) -> UserId {
        self.updated_by
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn format(&self) -> &DocumentFormat {
        &self.format
    }

    pub fn category(&self) -> Option<Uuid> {
        self.category
    }

    pub fn targets(&self) -> &[TargetSpecifier] {
        &self.targets
    }

    pub fn change_title<C: Clock>(
        &mut self,
        new_title: String,
        updated_by: UserId,
        clock: &C,
    ) -> Result<(), FactoryError> {
        if new_title.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Title is empty".to_string()));
        }
        self.title = new_title;
        self.updated_by = updated_by;
        self.updated_at = clock.now();
        Ok(())
    }

    pub fn change_category<C: Clock>(
        &mut self,
        new_category: Option<Uuid>,
        updated_by: UserId,
        clock: &C,
    ) -> Result<(), FactoryError> {
        self.category = new_category;
        self.updated_by = updated_by;
        self.updated_at = clock.now();

        Ok(())
    }

    pub fn change_format<C: Clock>(
        &mut self,
        new_format: DocumentFormat,
        updated_by: UserId,
        clock: &C,
    ) -> Result<(), FactoryError> {
        Self::validate_format(&new_format)?; // 参照渡し
        self.format = new_format;
        self.updated_by = updated_by;
        self.updated_at = clock.now();
        Ok(())
    }

    pub fn change_targets<C: Clock>(
        &mut self,
        new_targets: Vec<TargetSpecifier>,
        updated_by: UserId,
        clock: &C,
    ) -> Result<(), FactoryError> {
        if new_targets.is_empty() {
            return Err(FactoryError::InvalidInput(
                "Targets cannot be empty".to_string(),
            ));
        }
        self.targets = new_targets;
        self.updated_by = updated_by;
        self.updated_at = clock.now();
        Ok(())
    }
}
