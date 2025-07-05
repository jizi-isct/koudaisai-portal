use crate::entities::exhibitor::ExhibitorUpdate;
use crate::entities::user::UserRead;
use crate::sea_orm_entities;
use crate::util::IntoActiveValue;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, NotSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRequestType {
    TypeEditExhibitionInfo {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        exhibition_name: Option<Option<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        icon_id: Option<Option<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<Option<String>>,
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
                exhibition_name,
                icon_id,
                description,
            } => {
                let (is_exhibition_name_explicit_null, exhibition_name) = match exhibition_name {
                    Some(Some(exhibition_name)) => (Set(false), Set(Some(exhibition_name.clone()))),
                    Some(None) => (Set(true), Set(None)),
                    None => (Set(false), Set(None)),
                };
                let (is_icon_id_explicit_null, icon_id) = match icon_id {
                    Some(Some(icon_id)) => (Set(false), Set(Some(icon_id.clone()))),
                    Some(None) => (Set(true), Set(None)),
                    None => (Set(false), Set(None)),
                };
                let (is_description_explicit_null, description) = match description {
                    Some(Some(description)) => (Set(false), Set(Some(description.clone()))),
                    Some(None) => (Set(true), Set(None)),
                    None => (Set(false), Set(None)),
                };
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
                            exhibition_name,
                            icon_id,
                            description,
                            is_exhibition_name_explicit_null,
                            is_description_explicit_null,
                            is_icon_id_explicit_null,
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
                let exhibition_name = match r#type.exhibition_name {
                    Some(exhibition_name) => Some(Some(exhibition_name)),
                    None => {
                        if r#type.is_exhibition_name_explicit_null {
                            Some(None)
                        } else {
                            None
                        }
                    }
                };
                let icon_id = match r#type.icon_id {
                    Some(icon_id) => Some(Some(icon_id)),
                    None => {
                        if r#type.is_icon_id_explicit_null {
                            Some(None)
                        } else {
                            None
                        }
                    }
                };
                let description = match r#type.description {
                    Some(description) => Some(Some(description)),
                    None => {
                        if r#type.is_description_explicit_null {
                            Some(None)
                        } else {
                            None
                        }
                    }
                };
                ReadApprovalRequest {
                    id: generic.id,
                    issued_at: generic.issued_at.to_utc(),
                    issued_by: generic.issued_by,
                    r#type: ApprovalRequestType::TypeEditExhibitionInfo {
                        exhibition_name,
                        icon_id,
                        description,
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

        if let Some((generic, v)) = model {
            if let Some(v) = v {
                Ok(Some(Self::from_model(
                    ReadApprovalRequestModel::TypeEditExhibitionInfo { generic, r#type: v },
                )))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn approve(self, approved_by: Option<Uuid>, db_conn: &DbConn) -> anyhow::Result<()> {
        match self.r#type {
            ApprovalRequestType::TypeEditExhibitionInfo {
                exhibition_name,
                icon_id,
                description,
            } => {
                let user = match UserRead::find_from_id(self.issued_by, db_conn).await? {
                    Some(user) => user,
                    None => return Err(anyhow!("User not found".to_string())),
                };
                let exhibitor = user.get_exhibitor_read(db_conn).await?;
                ExhibitorUpdate {
                    exhibition_name,
                    icon_id,
                    description,
                }
                .update(exhibitor.id, db_conn)
                .await?;
            }
        }
        UpdateApprovalRequest {
            r#type: None,
            status: Some(ApprovalRequestStatus::Approved),
            approved_by: Some(approved_by),
        }
        .update(db_conn, self.id)
        .await?;
        Ok(())
    }

    pub async fn reject(self, db_conn: &DbConn, approved_by: Option<Uuid>) -> Result<(), DbErr> {
        UpdateApprovalRequest {
            r#type: None,
            status: Some(ApprovalRequestStatus::Rejected),
            approved_by: Some(approved_by),
        }
        .update(db_conn, self.id)
        .await?;
        Ok(())
    }

    pub async fn close(self, db_conn: &DbConn) -> Result<(), DbErr> {
        UpdateApprovalRequest {
            r#type: None,
            status: Some(ApprovalRequestStatus::Closed),
            approved_by: None,
        }
        .update(db_conn, self.id)
        .await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateApprovalRequest {
    #[serde(flatten, default)]
    pub r#type: Option<ApprovalRequestType>,
    #[serde(default)]
    pub status: Option<ApprovalRequestStatus>,
    #[serde(default)]
    pub approved_by: Option<Option<Uuid>>,
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
                exhibition_name,
                icon_id,
                description,
            }) => {
                let (is_exhibition_name_explicit_null, exhibition_name) = match exhibition_name {
                    Some(Some(exhibition_name)) => (Set(false), Set(Some(exhibition_name.clone()))),
                    Some(None) => (Set(true), Set(None)),
                    None => (Set(false), Set(None)),
                };
                let (is_icon_id_explicit_null, icon_id) = match icon_id {
                    Some(Some(icon_id)) => (Set(false), Set(Some(icon_id.clone()))),
                    Some(None) => (Set(true), Set(None)),
                    None => (Set(false), Set(None)),
                };
                let (is_description_explicit_null, description) = match description {
                    Some(Some(description)) => (Set(false), Set(Some(description.clone()))),
                    Some(None) => (Set(true), Set(None)),
                    None => (Set(false), Set(None)),
                };
                UpdateApprovalRequestActiveModel::TypeEditExhibitionInfo {
                    generic: sea_orm_entities::approval_request::ActiveModel {
                        id: Set(id.clone()),
                        issued_at: NotSet,
                        issued_by: NotSet,
                        r#type: self.r#type.map(|s| s.as_sea_orm()).into_active_value(),
                        status: self.status.map(|s| s.into_sea_orm()).into_active_value(),
                        approved_by: self.approved_by.into_active_value(),
                    },
                    r#type:
                        sea_orm_entities::approval_request_type_edit_exhibition_info::ActiveModel {
                            id: Set(id),
                            exhibition_name,
                            icon_id,
                            description,
                            is_exhibition_name_explicit_null,
                            is_description_explicit_null,
                            is_icon_id_explicit_null,
                        },
                }
            }
            None => UpdateApprovalRequestActiveModel::Generic {
                generic: sea_orm_entities::approval_request::ActiveModel {
                    id: Set(id),
                    issued_at: NotSet,
                    issued_by: NotSet,
                    r#type: self.r#type.map(|s| s.as_sea_orm()).into_active_value(),
                    status: self.status.map(|s| s.into_sea_orm()).into_active_value(),
                    approved_by: self.approved_by.into_active_value(),
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
