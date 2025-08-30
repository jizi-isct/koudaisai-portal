use crate::entities::approval_request::ReadApprovalRequest;
use crate::entities::form::FormRead;
use crate::entities::group::GroupRead;
use crate::entities::user_id::UserId;
use crate::middlewares::CurrentUser;
use crate::sea_orm_entities;
use crate::sea_orm_entities::read_notifications;
use crate::util::jwt::Claims;
use crate::util::sha::{stretch_with_salt, SHAManager};
use crate::util::{contains_uuid, format_secs_ja_full};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use reqwest::Response;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, DbErr, EntityTrait, NotSet, QueryFilter,
};
use sendgrid::v3::{Content, Email, Message, Personalization, Sender};
use sendgrid::SendgridResult;
use serde::{Deserialize, Serialize};
use tracing::log::trace;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub m_address: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRead {
    pub id: Uuid,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub name: String,
    pub m_address: String,
    pub group_id: String,
    pub password_updated_at: DateTime<chrono::Utc>,
}

impl UserCreate {
    pub async fn insert<C: ConnectionTrait>(
        self,
        db_conn: &C,
        id: Uuid,
        group_id: String,
    ) -> Result<Uuid, DbErr> {
        let active_model = sea_orm_entities::users::ActiveModel {
            id: Set(id),
            created_at: Set(Some(Utc::now().into())),
            updated_at: Set(Some(Utc::now().into())),
            name: Set(self.name),
            m_address: Set(self.m_address),
            password_hash: NotSet,
            password_salt: NotSet,
            group_id: Set(group_id),
            password_updated_at: Set(Utc::now().into()),
        };

        let user = active_model.insert(db_conn).await?;
        Ok(user.id)
    }
}

impl From<sea_orm_entities::users::Model> for UserRead {
    fn from(value: sea_orm_entities::users::Model) -> Self {
        UserRead {
            id: value.id,
            created_at: value.created_at.unwrap().to_utc(),
            updated_at: value.updated_at.unwrap().to_utc(),
            name: value.name,
            m_address: value.m_address,
            group_id: value.group_id,
            password_updated_at: value.password_updated_at.to_utc(),
        }
    }
}

impl UserRead {
    pub async fn from(value: Claims, db_conn: &DbConn) -> Result<Self> {
        match Self::find_from_id(value.sub, db_conn).await? {
            Some(value) => Ok(value),
            None => Err(anyhow::anyhow!("User not found")),
        }
    }

    pub async fn from_claims(claims: Claims, db_conn: &DbConn) -> Result<Self> {
        Ok(Self::find_from_id(claims.sub, db_conn)
            .await?
            .ok_or(anyhow!("User not found"))?)
    }

    pub async fn find_from_id(value: Uuid, db_conn: &DbConn) -> Result<Option<Self>, DbErr> {
        match sea_orm_entities::users::Entity::find_by_id(value)
            .one(db_conn)
            .await?
        {
            Some(value) => Ok(Some(value.into())),
            None => Ok(None),
        }
    }

    pub async fn find_from_m_address(value: String, db_conn: &DbConn) -> Result<Option<Self>> {
        match sea_orm_entities::users::Entity::find()
            .filter(sea_orm_entities::users::Column::MAddress.eq(value.to_string()))
            .one(db_conn)
            .await?
        {
            Some(value) => Ok(Some(value.into())),
            None => Ok(None),
        }
    }

    pub async fn get_all(db_conn: &DbConn) -> Result<Vec<Self>> {
        let users = sea_orm_entities::users::Entity::find().all(db_conn).await?;
        Ok(users.into_iter().map(Into::into).collect())
    }

    pub async fn get_group_read(&self, db_conn: &DbConn) -> Result<GroupRead> {
        match GroupRead::find_by_id(db_conn, &self.group_id).await? {
            Some(value) => Ok(value),
            None => Err(anyhow::anyhow!("Group not found")),
        }
    }

    pub async fn is_notification_read(
        &self,
        notification_id: Uuid,
        db_conn: &DbConn,
    ) -> Result<bool> {
        let result = read_notifications::Entity::find()
            .filter(read_notifications::Column::UserId.eq(self.id))
            .filter(read_notifications::Column::NotificationId.eq(notification_id))
            .one(db_conn)
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(result.is_some())
    }

    pub async fn get_forms(&self, db_conn: &DbConn) -> Result<Vec<FormRead>> {
        let forms_ = FormRead::find_all(db_conn).await?;
        let mut forms = vec![];
        for form in forms_ {
            for target in &form.targets {
                if target.does_user_match(Some(self), db_conn).await? {
                    forms.push(form);
                    break;
                }
            }
        }
        Ok(forms)
    }

    pub async fn get_approval_requests(
        &self,
        db_conn: &DbConn,
    ) -> Result<Vec<ReadApprovalRequest>> {
        let group = self.get_group_read(db_conn).await?;
        let requests = ReadApprovalRequest::get_all(db_conn).await?;
        let mut user_requests = vec![];
        for request in requests {
            if group.contains_user_in_representatives(request.issued_by) {
                user_requests.push(request);
            }
        }
        Ok(user_requests)
    }

    /// Sends an email to the user using Sendgrid
    pub async fn send_email_plain_text(
        &self,
        sender: &Sender,
        from: Email,
        subject: &str,
        body: &str,
    ) -> SendgridResult<Response> {
        let message = Message::new(from)
            .add_personalization(Personalization::new(Email::new(self.m_address.clone())))
            .set_subject(subject)
            .add_content(
                Content::new()
                    .set_content_type("text/plain")
                    .set_value(body),
            );

        tracing::info!("Sending email to: {}", self.m_address);

        let result = sender.send(&message).await;

        trace!("Email sent to {}: {:?}", self.m_address, result);

        result
    }

    pub async fn send_password_reset_email(
        &self,
        sender: &Sender,
        from: Email,
        reset_token: String,
        template_subject: &str,
        template_content: &str,
        expire_time: i64,
    ) -> SendgridResult<Response> {
        self.send_email_plain_text(
            sender,
            from,
            template_subject,
            &template_content
                .replace("{{reset_token}}", &reset_token)
                .replace("{{username}}", format!("{}", self.name).as_str())
                .replace("{{expires_at}}", &*format_secs_ja_full(expire_time)),
        )
        .await
    }

    pub async fn change_m_address(
        &self,
        db_conn: &DbConn,
        new_m_address: String,
        salt: String,
        stretch_cost: i32,
    ) -> Result<String> {
        let user_model = sea_orm_entities::users::Entity::find_by_id(self.id)
            .one(db_conn)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let mut active_model: sea_orm_entities::users::ActiveModel = user_model.into();
        active_model.m_address = Set(new_m_address.clone());
        active_model.updated_at = Set(Some(chrono::Utc::now().into()));
        active_model.password_hash = Set(None);
        active_model.password_updated_at = Set(chrono::Utc::now().into());

        let result = active_model.update(db_conn).await?;
        trace!("Updated user: {:?}", result);

        Ok(stretch_with_salt(
            new_m_address.as_str(),
            salt.as_str(),
            2_i32.pow(stretch_cost as u32),
        ))
    }

    pub async fn change_password(
        &self,
        db_conn: &DbConn,
        password: String,
        sha_manager: &SHAManager,
    ) -> Result<()> {
        let user_model = sea_orm_entities::users::Entity::find_by_id(self.id)
            .one(db_conn)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;
        let salt = &*user_model.password_salt.clone();

        let mut active_model: sea_orm_entities::users::ActiveModel = user_model.into();

        active_model.password_hash = Set(Some(sha_manager.stretch_with_salt(&*password, salt)));
        active_model.password_updated_at = Set(chrono::Utc::now().into());

        active_model.update(db_conn).await?;
        Ok(())
    }

    pub async fn from_user_id(
        user_id: UserId,
        current_user: UserRead,
        db_conn: &DbConn,
    ) -> Result<Self> {
        match user_id {
            UserId::Uuid(uuid) => Ok(Self::find_from_id(uuid, db_conn)
                .await?
                .ok_or(anyhow!("User not found"))?),
            UserId::Me => Ok(current_user),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserUpdate {
    pub name: String,
    pub m_address: String,
}

impl Into<sea_orm_entities::users::ActiveModel> for UserUpdate {
    fn into(self) -> sea_orm_entities::users::ActiveModel {
        sea_orm_entities::users::ActiveModel {
            id: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            name: Set(self.name),
            m_address: Set(self.m_address),
            password_hash: Default::default(),
            password_salt: Default::default(),
            group_id: Default::default(),
            password_updated_at: Default::default(),
        }
    }
}

impl UserUpdate {
    pub async fn update(
        self,
        user_id: Uuid,
        db_conn: &DbConn,
    ) -> Result<sea_orm_entities::users::Model> {
        let mut active_model: sea_orm_entities::users::ActiveModel = self.into();
        active_model.id = Set(user_id);

        let updated_user = active_model.update(db_conn).await?;
        Ok(updated_user)
    }
}
