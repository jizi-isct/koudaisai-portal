use crate::entities::user::UserRead;
use crate::routes::AppState;
use crate::sea_orm_entities;
use chrono::{DateTime, Utc};
use http::Method;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, IntoActiveValue, NotSet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRequestType {
    TypeEditExhibitionInfo {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        plan_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        is_child_friendly: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        icon_key: Option<String>,
    },
}

impl ApprovalRequestType {
    pub fn as_sea_orm(&self) -> sea_orm_entities::sea_orm_active_enums::ApprovalRequestType {
        match self {
            ApprovalRequestType::TypeEditExhibitionInfo { .. } => {
                sea_orm_entities::sea_orm_active_enums::ApprovalRequestType::TypeEditExhibitionInfo
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRequestStatus {
    Pending,
    Approved,
    Rejected,
    Closed,
}

impl ApprovalRequestStatus {
    pub fn into_sea_orm(self) -> sea_orm_entities::sea_orm_active_enums::ApprovalRequestStatus {
        match self {
            ApprovalRequestStatus::Pending => {
                sea_orm_entities::sea_orm_active_enums::ApprovalRequestStatus::Pending
            }
            ApprovalRequestStatus::Approved => {
                sea_orm_entities::sea_orm_active_enums::ApprovalRequestStatus::Approved
            }
            ApprovalRequestStatus::Rejected => {
                sea_orm_entities::sea_orm_active_enums::ApprovalRequestStatus::Rejected
            }
            ApprovalRequestStatus::Closed => {
                sea_orm_entities::sea_orm_active_enums::ApprovalRequestStatus::Closed
            }
        }
    }

    pub fn from_sea_orm(
        status: sea_orm_entities::sea_orm_active_enums::ApprovalRequestStatus,
    ) -> Self {
        match status {
            sea_orm_entities::sea_orm_active_enums::ApprovalRequestStatus::Pending => {
                ApprovalRequestStatus::Pending
            }
            sea_orm_entities::sea_orm_active_enums::ApprovalRequestStatus::Approved => {
                ApprovalRequestStatus::Approved
            }
            sea_orm_entities::sea_orm_active_enums::ApprovalRequestStatus::Rejected => {
                ApprovalRequestStatus::Rejected
            }
            sea_orm_entities::sea_orm_active_enums::ApprovalRequestStatus::Closed => {
                ApprovalRequestStatus::Closed
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateApprovalRequest {
    #[serde(flatten)]
    pub r#type: ApprovalRequestType,
}

pub enum CreateApprovalRequestActiveModel {
    TypeEditExhibitionInfo {
        generic: sea_orm_entities::approval_request::ActiveModel,
        r#type: sea_orm_entities::approval_request_type_edit_exhibition_info::ActiveModel,
    },
}

impl CreateApprovalRequest {
    pub fn into_active_model(self, issued_by: Uuid) -> CreateApprovalRequestActiveModel {
        let id = Uuid::new_v4();
        match &self.r#type {
            ApprovalRequestType::TypeEditExhibitionInfo {
                plan_name,
                description,
                is_child_friendly,
                icon_key,
            } => {
                // Map plan_name to exhibition_name in database
                CreateApprovalRequestActiveModel::TypeEditExhibitionInfo {
                    generic: sea_orm_entities::approval_request::ActiveModel {
                        id: Set(id.clone()),
                        issued_at: Set(Utc::now().into()),
                        issued_by: Set(issued_by),
                        r#type: Set(self.r#type.as_sea_orm()),
                        status: Set(ApprovalRequestStatus::Pending.into_sea_orm()),
                        approved_by: Set(None),
                    },
                    r#type:
                        sea_orm_entities::approval_request_type_edit_exhibition_info::ActiveModel {
                            id: Set(id),
                            exhibition_name: plan_name.clone().into_active_value(),
                            icon_id: icon_key.clone().into_active_value(),
                            description: description.clone().into_active_value(),
                            is_child_friendly: is_child_friendly.into_active_value(),
                        },
                }
            }
        }
    }

    pub async fn insert(self, db_conn: &DbConn, issued_by: Uuid) -> Result<(), DbErr> {
        match self.into_active_model(issued_by) {
            CreateApprovalRequestActiveModel::TypeEditExhibitionInfo { generic, r#type } => {
                generic.insert(db_conn).await?;
                r#type.insert(db_conn).await?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadApprovalRequest {
    pub id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub issued_by: Uuid,
    #[serde(flatten)]
    pub r#type: ApprovalRequestType,
    pub status: ApprovalRequestStatus,
    pub approved_by: Option<Uuid>,
}

pub enum ReadApprovalRequestModel {
    TypeEditExhibitionInfo {
        generic: sea_orm_entities::approval_request::Model,
        r#type: sea_orm_entities::approval_request_type_edit_exhibition_info::Model,
    },
}

impl ReadApprovalRequest {
    pub fn from_model(model: ReadApprovalRequestModel) -> Self {
        match model {
            ReadApprovalRequestModel::TypeEditExhibitionInfo { generic, r#type } => {
                // Map exhibition_name to plan_name
                let plan_name = r#type.exhibition_name.clone();
                // Map icon_id to icon_key
                let icon_key = r#type.icon_id.clone();
                let is_child_friendly = r#type.is_child_friendly.clone();
                let description = r#type.description.clone();

                ReadApprovalRequest {
                    id: generic.id,
                    issued_at: generic.issued_at.to_utc(),
                    issued_by: generic.issued_by,
                    r#type: ApprovalRequestType::TypeEditExhibitionInfo {
                        plan_name,
                        description,
                        is_child_friendly,
                        icon_key,
                    },
                    status: ApprovalRequestStatus::from_sea_orm(generic.status),
                    approved_by: generic.approved_by,
                }
            }
        }
    }

    pub async fn get_all(db_conn: &DbConn) -> Result<Vec<Self>, DbErr> {
        let models = sea_orm_entities::approval_request::Entity::find()
            .find_also_related(sea_orm_entities::approval_request_type_edit_exhibition_info::Entity)
            .all(db_conn)
            .await?;

        let mut result = Vec::new();
        for (generic, v) in models {
            if let Some(v) = v {
                // If the related type is found, we can construct the ReadApprovalRequest
                result.push(Self::from_model(
                    ReadApprovalRequestModel::TypeEditExhibitionInfo { generic, r#type: v },
                ));
                continue;
            }
        }

        Ok(result)
    }

    pub async fn find_from_id(id: Uuid, db_conn: &DbConn) -> Result<Option<Self>, DbErr> {
        let model = sea_orm_entities::approval_request::Entity::find_by_id(id)
            .find_also_related(sea_orm_entities::approval_request_type_edit_exhibition_info::Entity)
            .one(db_conn)
            .await?;

        if let Some((generic, Some(v))) = model {
            Ok(Some(Self::from_model(
                ReadApprovalRequestModel::TypeEditExhibitionInfo { generic, r#type: v },
            )))
        } else {
            Ok(None)
        }
    }

    /// Approves a request by updating relevant details and resolving the approval process.
    ///
    /// # Parameters
    /// - `self`: Instance of the approval request to be processed.
    /// - `approved_by`: An optional UUID representing the user who approved the request.
    /// - `state`: Shared application state.
    /// - `token`: Bearer authentication token to authorize API requests.
    pub async fn approve(
        self,
        approved_by: Option<Uuid>,
        state: Arc<AppState>,
        token: &str,
    ) -> anyhow::Result<()> {
        match self.r#type {
            ApprovalRequestType::TypeEditExhibitionInfo {
                plan_name,
                description,
                is_child_friendly,
                icon_key,
            } => {
                // 団体情報
                let issuer = UserRead::find_from_id(self.issued_by, &state.db_conn).await?;
                if issuer.is_none() {
                    return Err(anyhow::anyhow!("Issuer not found"));
                }
                let issuer = issuer.unwrap();

                // 企画情報の更新(アイコン以外)
                let mut body = HashMap::new();
                if let Some(plan_name) = plan_name {
                    body.insert("plan_name", plan_name);
                }
                if let Some(description) = description {
                    body.insert("description", description);
                }
                if let Some(is_child_friendly) = is_child_friendly {
                    body.insert("is_child_friendly", is_child_friendly.to_string());
                }
                let response = state
                    .http_client
                    .request(
                        Method::PATCH,
                        format!("https://api2025.jizi.jp/v1/plans/{}", issuer.group_id),
                    )
                    .json(&body)
                    .bearer_auth(token.clone())
                    .send()
                    .await?;
                if !response.status().is_success() {
                    return Err(anyhow::anyhow!(
                        "Failed to update plan: {}",
                        response.text().await.unwrap_or("Error".into())
                    ));
                }

                // アイコンの更新
                if icon_key.is_none() {
                    return Ok(());
                }
                let icon_key = icon_key.unwrap();
                let object = state
                    .s3_client
                    .get_object()
                    .bucket(&state.s3_bucket)
                    .key(&icon_key)
                    .send()
                    .await?;
                let content_type = object.content_type;
                if content_type.is_none() {
                    return Err(anyhow::anyhow!("Content type not found"));
                }
                let content_type = content_type.unwrap();
                let body = object.body.collect().await?.into_bytes();
                let response = state
                    .http_client
                    .request(
                        Method::PUT,
                        format!("https://api2025.jizi.jp/v1/plans/{}/icon", issuer.group_id),
                    )
                    .body(body)
                    .header("Content-Type", content_type)
                    .bearer_auth(token)
                    .send()
                    .await?;
                if !response.status().is_success() {
                    return Err(anyhow::anyhow!(
                        "Failed to upload icon: {}",
                        response.text().await.unwrap_or("Error".into())
                    ));
                }
            }
        }
        // 承認申請のステータスを更新
        let model = sea_orm_entities::approval_request::ActiveModel {
            id: Set(self.id),
            status: Set(ApprovalRequestStatus::Approved.into_sea_orm()),
            approved_by: Set(approved_by),
            ..Default::default()
        };
        model.update(&state.db_conn).await?;
        Ok(())
    }

    pub async fn reject(self, db_conn: &DbConn, approved_by: Option<Uuid>) -> Result<(), DbErr> {
        let model = sea_orm_entities::approval_request::ActiveModel {
            id: Set(self.id),
            status: Set(ApprovalRequestStatus::Rejected.into_sea_orm()),
            approved_by: Set(approved_by),
            ..Default::default()
        };
        model.update(db_conn).await?;
        Ok(())
    }

    pub async fn close(self, db_conn: &DbConn) -> Result<(), DbErr> {
        // Since API schema doesn't have Closed status, we'll use Rejected for close operations
        let model = sea_orm_entities::approval_request::ActiveModel {
            id: Set(self.id),
            status: Set(ApprovalRequestStatus::Rejected.into_sea_orm()),
            ..Default::default()
        };
        model.update(db_conn).await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateApprovalRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ApprovalRequestStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<Option<Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ApprovalRequestType>,
}

pub enum UpdateApprovalRequestActiveModel {
    TypeEditExhibitionInfo {
        generic: sea_orm_entities::approval_request::ActiveModel,
        r#type: sea_orm_entities::approval_request_type_edit_exhibition_info::ActiveModel,
    },
    Generic {
        generic: sea_orm_entities::approval_request::ActiveModel,
    },
}

impl UpdateApprovalRequest {
    pub fn into_active_model(self, id: Uuid) -> UpdateApprovalRequestActiveModel {
        match &self.r#type {
            Some(ApprovalRequestType::TypeEditExhibitionInfo {
                plan_name,
                description,
                is_child_friendly,
                icon_key,
            }) => UpdateApprovalRequestActiveModel::TypeEditExhibitionInfo {
                generic: sea_orm_entities::approval_request::ActiveModel {
                    id: Set(id.clone()),
                    status: self.status.map(|s| Set(s.into_sea_orm())).unwrap_or(NotSet),
                    approved_by: self.approved_by.map(|ab| Set(ab)).unwrap_or(NotSet),
                    ..Default::default()
                },
                r#type: sea_orm_entities::approval_request_type_edit_exhibition_info::ActiveModel {
                    id: Set(id),
                    exhibition_name: plan_name.clone().into_active_value(),
                    icon_id: icon_key.clone().into_active_value(),
                    is_child_friendly: is_child_friendly.into_active_value(),
                    description: description.clone().into_active_value(),
                },
            },
            None => UpdateApprovalRequestActiveModel::Generic {
                generic: sea_orm_entities::approval_request::ActiveModel {
                    id: Set(id),
                    status: self.status.map(|s| Set(s.into_sea_orm())).unwrap_or(NotSet),
                    approved_by: self.approved_by.map(|ab| Set(ab)).unwrap_or(NotSet),
                    ..Default::default()
                },
            },
        }
    }

    pub async fn update(self, db_conn: &DbConn, id: Uuid) -> Result<(), DbErr> {
        match self.into_active_model(id) {
            UpdateApprovalRequestActiveModel::TypeEditExhibitionInfo { generic, r#type } => {
                generic.update(db_conn).await?;
                r#type.update(db_conn).await?;
            }
            UpdateApprovalRequestActiveModel::Generic { generic } => {
                generic.update(db_conn).await?;
            }
        }
        Ok(())
    }
}

pub async fn delete_by_id(id: Uuid, db_conn: &DbConn) -> Result<(), DbErr> {
    sea_orm_entities::approval_request::Entity::delete_by_id(id)
        .exec(db_conn)
        .await?;
    Ok(())
}
