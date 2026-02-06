use crate::application::ports::clock::Clock;
use crate::domain::error::FactoryError;
use crate::domain::form_id::FormId;
use crate::domain::target_specifier::TargetSpecifier;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormType {
    TypeExternal { form_url: String },
}

pub struct Form {
    id: FormId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: Uuid,
    updated_by: Uuid,
    targets: Vec<TargetSpecifier>,
    form_name: String,
    summary: String,
    due_date: DateTime<Utc>,
    r#type: FormType,
}
