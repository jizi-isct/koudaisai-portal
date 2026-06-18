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

#[derive(Deserialize, ToSchema)]
pub struct FormCreate {
    #[schema(value_type = Vec<String>)]
    targets: Vec<TargetSpecifier>,
    name: String,
    summary: String,
    due_date: DateTime<Utc>,
    #[serde(flatten)]
    r#type: FormType,
}

#[derive(Deserialize, ToSchema)]
pub struct FormUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<String>>)]
    targets: Option<Vec<TargetSpecifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    due_date: Option<DateTime<Utc>>,
}
