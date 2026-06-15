use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::form_repo::FormRepo;
use crate::domain::form::{Form, FormType};
use crate::domain::form_id::FormId;
use crate::infra::sqlite::util::{
    dt_to_ms, ms_to_dt, targets_from_json, targets_to_json, to_insert_error,
};
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteFormRepo {
    pool: SqlitePool,
}

impl SqliteFormRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

/// `FormType` を type/form_url 列へ分解。
struct TypeCols {
    type_: &'static str,
    form_url: Option<String>,
}

fn type_cols(form: &Form) -> TypeCols {
    match form.r#type() {
        FormType::TypeExternal { form_url } => TypeCols {
            type_: "external",
            form_url: Some(form_url.clone()),
        },
    }
}

/// type / form_url 列から `FormType` を復元する。
fn form_type_from_cols(type_: String, form_url: Option<String>) -> anyhow::Result<FormType> {
    Ok(match type_.as_str() {
        "external" => FormType::TypeExternal {
            form_url: form_url.ok_or_else(|| anyhow::anyhow!("external form missing form_url"))?,
        },
        other => return Err(anyhow::anyhow!("unknown form type: {}", other)),
    })
}

#[async_trait]
impl FormRepo for SqliteFormRepo {
    async fn find_by_id(&self, id: FormId) -> Result<Option<Form>, FindError> {
        let id = id.to_string();
        let row = sqlx::query!(
            r#"SELECT id, created_at, updated_at, created_by, updated_by, targets,
                      name, summary, due_date, type, form_url
               FROM forms WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| -> anyhow::Result<Form> {
            Ok(Form::restore(
                FormId::new(Uuid::parse_str(&r.id)?),
                ms_to_dt(r.created_at)?,
                ms_to_dt(r.updated_at)?,
                Uuid::parse_str(&r.created_by)?,
                Uuid::parse_str(&r.updated_by)?,
                targets_from_json(&r.targets)?,
                r.name,
                r.summary,
                ms_to_dt(r.due_date)?,
                form_type_from_cols(r.r#type, r.form_url)?,
            ))
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_all(&self) -> Result<Vec<Form>, FindError> {
        let rows = sqlx::query!(
            r#"SELECT id, created_at, updated_at, created_by, updated_by, targets,
                      name, summary, due_date, type, form_url
               FROM forms"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<Form> {
                Ok(Form::restore(
                    FormId::new(Uuid::parse_str(&r.id)?),
                    ms_to_dt(r.created_at)?,
                    ms_to_dt(r.updated_at)?,
                    Uuid::parse_str(&r.created_by)?,
                    Uuid::parse_str(&r.updated_by)?,
                    targets_from_json(&r.targets)?,
                    r.name,
                    r.summary,
                    ms_to_dt(r.due_date)?,
                    form_type_from_cols(r.r#type, r.form_url)?,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert(&self, form: Form) -> Result<(), InsertError> {
        let t = type_cols(&form);
        let id = form.id().to_string();
        let created_at = dt_to_ms(form.created_at());
        let updated_at = dt_to_ms(form.updated_at());
        let created_by = form.created_by().to_string();
        let updated_by = form.updated_by().to_string();
        let targets = targets_to_json(form.targets());
        let name = form.name().to_string();
        let summary = form.summary().to_string();
        let due_date = dt_to_ms(form.due_date());
        sqlx::query!(
            "INSERT INTO forms \
             (id, created_at, updated_at, created_by, updated_by, targets, name, summary, due_date, type, form_url) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            id,
            created_at,
            updated_at,
            created_by,
            updated_by,
            targets,
            name,
            summary,
            due_date,
            t.type_,
            t.form_url,
        )
        .execute(&self.pool)
        .await
        .map_err(to_insert_error)?;
        Ok(())
    }

    async fn update(&self, form: Form) -> Result<(), UpdateError> {
        let t = type_cols(&form);
        let id = form.id().to_string();
        let created_at = dt_to_ms(form.created_at());
        let updated_at = dt_to_ms(form.updated_at());
        let created_by = form.created_by().to_string();
        let updated_by = form.updated_by().to_string();
        let targets = targets_to_json(form.targets());
        let name = form.name().to_string();
        let summary = form.summary().to_string();
        let due_date = dt_to_ms(form.due_date());
        let res = sqlx::query!(
            "UPDATE forms SET \
             created_at = ?, updated_at = ?, created_by = ?, updated_by = ?, targets = ?, \
             name = ?, summary = ?, due_date = ?, type = ?, form_url = ? \
             WHERE id = ?",
            created_at,
            updated_at,
            created_by,
            updated_by,
            targets,
            name,
            summary,
            due_date,
            t.type_,
            t.form_url,
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| UpdateError::InternalError(e.into()))?;
        if res.rows_affected() == 0 {
            return Err(UpdateError::NotFound);
        }
        Ok(())
    }

    async fn delete(&self, id: FormId) -> Result<(), DeleteError> {
        let id = id.to_string();
        let res = sqlx::query!("DELETE FROM forms WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DeleteError::InternalError(e.into()))?;
        if res.rows_affected() == 0 {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }
}
