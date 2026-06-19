use crate::domain::form::{Form, FormType as DomainFormType};
use crate::domain::target_specifier::TargetSpecifier;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormType {
    External { form_url: String },
}

impl From<&DomainFormType> for FormType {
    fn from(t: &DomainFormType) -> Self {
        match t {
            DomainFormType::TypeExternal { form_url } => FormType::External {
                form_url: form_url.clone(),
            },
        }
    }
}

impl From<&FormType> for DomainFormType {
    fn from(t: &FormType) -> Self {
        match t {
            FormType::External { form_url } => DomainFormType::TypeExternal {
                form_url: form_url.clone(),
            },
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct FormRead {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: Option<Uuid>,
    #[schema(value_type = Vec<String>)]
    targets: Vec<TargetSpecifier>,
    name: String,
    summary: String,
    due_date: DateTime<Utc>,
    #[serde(flatten)]
    r#type: FormType,
}

impl From<&Form> for FormRead {
    fn from(f: &Form) -> Self {
        FormRead {
            id: f.id().as_uuid(),
            created_at: f.created_at(),
            updated_at: f.updated_at(),
            created_by: Some(f.created_by()),
            targets: f.targets().clone(),
            name: f.name().to_string(),
            summary: f.summary().to_string(),
            due_date: f.due_date(),
            r#type: f.r#type().into(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct FormCreate {
    #[schema(value_type = Vec<String>)]
    pub targets: Vec<TargetSpecifier>,
    pub name: String,
    pub summary: String,
    pub due_date: DateTime<Utc>,
    #[serde(flatten)]
    pub r#type: FormType,
}

#[derive(Deserialize, ToSchema)]
pub struct FormUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<String>>)]
    pub targets: Option<Vec<TargetSpecifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DateTime<Utc>>,
}
