use crate::entities::target_specifier::TargetSpecifier;
use crate::sea_orm_entities::{
    notification, notification_type_approval_request, notification_type_markdown,
    sea_orm_active_enums,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    TypeMarkdown { title: String, content: String },
    TypeApprovalRequest { approval_request_id: Uuid },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationCreate {
    pub target: Vec<TargetSpecifier>,
    #[serde(flatten)]
    pub notification_type: NotificationType,
}

pub enum NotificationCreateActiveModel {
    TypeMarkdown(
        notification::ActiveModel,
        notification_type_markdown::ActiveModel,
    ),
    TypeApprovalRequest(
        notification::ActiveModel,
        notification_type_approval_request::ActiveModel,
    ),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationRead {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub target: Vec<TargetSpecifier>,
    #[serde(flatten)]
    pub notification_type: NotificationType,
}

pub enum NotificationReadModel {
    TypeMarkdown(notification::Model, notification_type_markdown::Model),
    TypeApprovalRequest(
        notification::Model,
        notification_type_approval_request::Model,
    ),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationUpdate {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub target: Option<Vec<TargetSpecifier>>,
    #[serde(flatten, default)]
    pub notification_type: Option<NotificationType>,
}

pub enum NotificationUpdateActiveModel {
    TypeMarkdown(
        notification::ActiveModel,
        notification_type_markdown::ActiveModel,
    ),
    TypeApprovalRequest(
        notification::ActiveModel,
        notification_type_approval_request::ActiveModel,
    ),
    NoTypeSpecified(notification::ActiveModel),
}

impl NotificationCreate {
    pub fn into_active_model(self, created_by: Option<Uuid>) -> NotificationCreateActiveModel {
        match self.notification_type {
            NotificationType::TypeMarkdown { title, content } => {
                let id = Uuid::new_v4();
                let notification_model = notification::ActiveModel {
                    id: Set(id.clone()),
                    created_at: Set(Utc::now().into()),
                    updated_at: Set(Utc::now().into()),
                    created_by: Set(created_by),
                    updated_by: Set(created_by),
                    target: Set(self.target.iter().map(|t| t.into()).collect()),
                    r#type: Set(sea_orm_active_enums::NotificationType::Markdown),
                };
                let markdown_model = notification_type_markdown::ActiveModel {
                    id: Set(id),
                    title: Set(title),
                    content: Set(content),
                };
                NotificationCreateActiveModel::TypeMarkdown(notification_model, markdown_model)
            }
            NotificationType::TypeApprovalRequest {
                approval_request_id,
            } => {
                let id = Uuid::new_v4();
                let notification_model = notification::ActiveModel {
                    id: Set(id.clone()),
                    created_at: Set(Utc::now().into()),
                    updated_at: Set(Utc::now().into()),
                    created_by: Set(created_by),
                    updated_by: Set(created_by),
                    target: Set(self.target.iter().map(|t| t.into()).collect()),
                    r#type: Set(sea_orm_active_enums::NotificationType::ApprovalRequest),
                };
                let approval_request_model = notification_type_approval_request::ActiveModel {
                    id: Set(id),
                    approval_request_id: Set(approval_request_id),
                };
                NotificationCreateActiveModel::TypeApprovalRequest(
                    notification_model,
                    approval_request_model,
                )
            }
        }
    }

    pub async fn insert(self, db: &DbConn, created_by: Option<Uuid>) -> Result<()> {
        let active_model = self.into_active_model(created_by);
        match active_model {
            NotificationCreateActiveModel::TypeMarkdown(notification_model, markdown_model) => {
                notification_model.insert(db).await?;
                markdown_model.insert(db).await?;
                Ok(())
            }
            NotificationCreateActiveModel::TypeApprovalRequest(
                notification_model,
                approval_request_model,
            ) => {
                notification_model.insert(db).await?;
                approval_request_model.insert(db).await?;
                Ok(())
            }
        }
    }
}

impl NotificationRead {
    pub async fn from_model(model: NotificationReadModel) -> anyhow::Result<Self> {
        match model {
            NotificationReadModel::TypeMarkdown(notification, markdown) => Ok(NotificationRead {
                id: notification.id,
                created_at: notification.created_at.into(),
                updated_at: notification.updated_at.into(),
                created_by: notification.created_by.unwrap_or(Uuid::new_v4()),
                updated_by: notification.updated_by.unwrap_or(Uuid::new_v4()),
                target: notification
                    .target
                    .iter()
                    .map(|t| TargetSpecifier::from_string(t))
                    .collect(),
                notification_type: NotificationType::TypeMarkdown {
                    title: markdown.title,
                    content: markdown.content,
                },
            }),
            NotificationReadModel::TypeApprovalRequest(notification, approval_request) => {
                Ok(NotificationRead {
                    id: notification.id,
                    created_at: notification.created_at.into(),
                    updated_at: notification.updated_at.into(),
                    created_by: notification.created_by.unwrap_or(Uuid::new_v4()),
                    updated_by: notification.updated_by.unwrap_or(Uuid::new_v4()),
                    target: notification
                        .target
                        .iter()
                        .map(|t| TargetSpecifier::from_string(t))
                        .collect(),
                    notification_type: NotificationType::TypeApprovalRequest {
                        approval_request_id: approval_request.approval_request_id,
                    },
                })
            }
        }
    }
    pub async fn find_by_id(db: &DbConn, id: Uuid) -> Result<Option<NotificationRead>> {
        let notification = notification::Entity::find_by_id(id).one(db).await?;
        if let Some(notification) = notification {
            match notification.r#type {
                sea_orm_active_enums::NotificationType::Markdown => {
                    let markdown = notification_type_markdown::Entity::find_by_id(id)
                        .one(db)
                        .await?
                        .ok_or(anyhow!("No Markdown content found."))?;
                    Ok(Some(
                        NotificationRead::from_model(NotificationReadModel::TypeMarkdown(
                            notification,
                            markdown,
                        ))
                        .await?,
                    ))
                }
                sea_orm_active_enums::NotificationType::ApprovalRequest => {
                    let approval_request =
                        notification_type_approval_request::Entity::find_by_id(id)
                            .one(db)
                            .await?
                            .ok_or(anyhow!("No ApprovalRequest content found."))?;
                    Ok(Some(
                        NotificationRead::from_model(NotificationReadModel::TypeApprovalRequest(
                            notification,
                            approval_request,
                        ))
                        .await?,
                    ))
                }
            }
        } else {
            Ok(None)
        }
    }

    pub async fn get_all(db: &DbConn) -> Result<Vec<NotificationRead>> {
        let notifications = notification::Entity::find().all(db).await?;
        let mut results = Vec::new();
        for notification in notifications {
            match notification.r#type {
                sea_orm_active_enums::NotificationType::Markdown => {
                    let markdown = notification_type_markdown::Entity::find_by_id(notification.id)
                        .one(db)
                        .await?
                        .ok_or(anyhow!("No Markdown content found."))?;
                    results.push(
                        NotificationRead::from_model(NotificationReadModel::TypeMarkdown(
                            notification,
                            markdown,
                        ))
                        .await?,
                    );
                }
                sea_orm_active_enums::NotificationType::ApprovalRequest => {
                    let approval_request =
                        notification_type_approval_request::Entity::find_by_id(notification.id)
                            .one(db)
                            .await?
                            .ok_or(anyhow!("No ApprovalRequest content found."))?;
                    results.push(
                        NotificationRead::from_model(NotificationReadModel::TypeApprovalRequest(
                            notification,
                            approval_request,
                        ))
                        .await?,
                    );
                }
            }
        }
        Ok(results)
    }
}

impl NotificationUpdate {
    pub async fn into_active_model(
        self,
        id: Uuid,
        updated_by: Option<Uuid>,
    ) -> Result<NotificationUpdateActiveModel> {
        let target = match self.target {
            Some(targets) => Set(targets.iter().map(|t| t.into()).collect::<Vec<String>>()),
            None => Default::default(),
        };
        let notification_model = notification::ActiveModel {
            id: Set(id),
            created_at: Default::default(),
            updated_at: Set(Utc::now().into()),
            created_by: Default::default(),
            updated_by: Set(updated_by),
            target,
            r#type: Default::default(),
        };

        match self.notification_type {
            Some(NotificationType::TypeMarkdown { title, content }) => {
                let markdown_model = notification_type_markdown::ActiveModel {
                    id: Set(id),
                    title: Set(title),
                    content: Set(content),
                };
                Ok(NotificationUpdateActiveModel::TypeMarkdown(
                    notification_model,
                    markdown_model,
                ))
            }
            Some(NotificationType::TypeApprovalRequest {
                approval_request_id,
            }) => {
                let approval_request_model = notification_type_approval_request::ActiveModel {
                    id: Set(id),
                    approval_request_id: Set(approval_request_id),
                };
                Ok(NotificationUpdateActiveModel::TypeApprovalRequest(
                    notification_model,
                    approval_request_model,
                ))
            }
            None => Ok(NotificationUpdateActiveModel::NoTypeSpecified(
                notification_model,
            )),
        }
    }

    pub async fn update(self, db: &DbConn, id: Uuid, updated_by: Option<Uuid>) -> Result<()> {
        let active_model = self.into_active_model(id, updated_by).await?;
        match active_model {
            NotificationUpdateActiveModel::TypeMarkdown(notification_model, markdown_model) => {
                notification_model.update(db).await?;
                markdown_model.update(db).await?;
                Ok(())
            }
            NotificationUpdateActiveModel::TypeApprovalRequest(
                notification_model,
                approval_request_model,
            ) => {
                notification_model.update(db).await?;
                approval_request_model.update(db).await?;
                Ok(())
            }
            NotificationUpdateActiveModel::NoTypeSpecified(notification_model) => {
                notification_model.update(db).await?;
                Ok(())
            }
        }
    }
}

pub async fn delete_notification(db: &DbConn, id: Uuid) -> Result<()> {
    notification::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}
