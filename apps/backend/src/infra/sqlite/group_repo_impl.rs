use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::group_repo::GroupRepo;
use crate::domain::group::{Group, GroupType};
use crate::domain::group_id::GroupId;
use crate::domain::user_id::UserId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::{dt_to_ms, ms_to_dt, to_insert_error};
use async_trait::async_trait;
use sqlx::{Sqlite, SqlitePool};
use std::str::FromStr;
use uuid::Uuid;

pub struct SqliteGroupRepo {
    pool: SqlitePool,
}

impl SqliteGroupRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

fn uid_str(uid: &UserId) -> String {
    Uuid::from(*uid).to_string()
}

fn parse_uid(s: &str) -> anyhow::Result<UserId> {
    Ok(UserId::new(Uuid::parse_str(s)?))
}

/// `GroupType`(代数的データ型)をテーブルの各列へ分解した中間表現。
struct TypeCols {
    type_: &'static str,
    r1: Option<String>,
    r2: Option<String>,
    r3: Option<String>,
    operator: Option<String>,
}

fn type_cols(group: &Group) -> TypeCols {
    match group.r#type() {
        GroupType::Press { representative } => TypeCols {
            type_: "press",
            r1: Some(uid_str(representative)),
            r2: None,
            r3: None,
            operator: None,
        },
        GroupType::GeneralProject {
            representative1,
            representative2,
            representative3,
        } => TypeCols {
            type_: "general_project",
            r1: Some(uid_str(representative1)),
            r2: Some(uid_str(representative2)),
            r3: Some(uid_str(representative3)),
            operator: None,
        },
        GroupType::BoothProject {
            representative1,
            representative2,
            representative3,
        } => TypeCols {
            type_: "booth_project",
            r1: Some(uid_str(representative1)),
            r2: Some(uid_str(representative2)),
            r3: Some(uid_str(representative3)),
            operator: None,
        },
        GroupType::StageProject {
            representative1,
            representative2,
            representative3,
        } => TypeCols {
            type_: "stage_project",
            r1: Some(uid_str(representative1)),
            r2: Some(uid_str(representative2)),
            r3: Some(uid_str(representative3)),
            operator: None,
        },
        GroupType::LabProject {
            representative,
            operator,
        } => TypeCols {
            type_: "lab_project",
            r1: Some(uid_str(representative)),
            r2: None,
            r3: None,
            operator: Some(uid_str(operator)),
        },
    }
}

/// type / representative* / operator 列群から `GroupType`(代数的データ型)を復元する。
fn group_type_from_cols(
    type_: String,
    representative1: Option<String>,
    representative2: Option<String>,
    representative3: Option<String>,
    operator: Option<String>,
) -> anyhow::Result<GroupType> {
    let req = |v: Option<String>, col: &str| -> anyhow::Result<UserId> {
        parse_uid(&v.ok_or_else(|| anyhow::anyhow!("group {} missing {}", type_, col))?)
    };

    Ok(match type_.as_str() {
        "press" => GroupType::Press {
            representative: req(representative1, "representative1")?,
        },
        "general_project" => GroupType::GeneralProject {
            representative1: req(representative1, "representative1")?,
            representative2: req(representative2, "representative2")?,
            representative3: req(representative3, "representative3")?,
        },
        "booth_project" => GroupType::BoothProject {
            representative1: req(representative1, "representative1")?,
            representative2: req(representative2, "representative2")?,
            representative3: req(representative3, "representative3")?,
        },
        "stage_project" => GroupType::StageProject {
            representative1: req(representative1, "representative1")?,
            representative2: req(representative2, "representative2")?,
            representative3: req(representative3, "representative3")?,
        },
        "lab_project" => GroupType::LabProject {
            representative: req(representative1, "representative1")?,
            operator: req(operator, "operator")?,
        },
        other => return Err(anyhow::anyhow!("unknown group type: {}", other)),
    })
}

async fn exec_insert<'c, E>(executor: E, group: &Group) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let c = type_cols(group);
    let id = group.id().to_string();
    let created_at = dt_to_ms(group.created_at());
    let updated_at = dt_to_ms(group.updated_at());
    let name = group.name().to_string();
    sqlx::query!(
        "INSERT INTO groups \
         (id, created_at, updated_at, name, type, representative1, representative2, representative3, operator) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        id,
        created_at,
        updated_at,
        name,
        c.type_,
        c.r1,
        c.r2,
        c.r3,
        c.operator,
    )
    .execute(executor)
    .await?;
    Ok(())
}

async fn exec_update<'c, E>(executor: E, group: &Group) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let c = type_cols(group);
    let id = group.id().to_string();
    let created_at = dt_to_ms(group.created_at());
    let updated_at = dt_to_ms(group.updated_at());
    let name = group.name().to_string();
    let res = sqlx::query!(
        "UPDATE groups SET \
         created_at = ?, updated_at = ?, name = ?, type = ?, \
         representative1 = ?, representative2 = ?, representative3 = ?, operator = ? \
         WHERE id = ?",
        created_at,
        updated_at,
        name,
        c.type_,
        c.r1,
        c.r2,
        c.r3,
        c.operator,
        id,
    )
    .execute(executor)
    .await?;
    Ok(res.rows_affected())
}

async fn exec_delete<'c, E>(executor: E, id: GroupId) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = id.to_string();
    let res = sqlx::query!("DELETE FROM groups WHERE id = ?", id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

#[async_trait]
impl GroupRepo<SqliteTransaction> for SqliteGroupRepo {
    async fn find_by_id(&self, id: GroupId) -> Result<Option<Group>, FindError> {
        let id = id.to_string();
        let row = sqlx::query!(
            r#"SELECT id, created_at, updated_at, name, type,
                      representative1, representative2, representative3, operator
               FROM groups WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| -> anyhow::Result<Group> {
            Group::restore(
                GroupId::from_str(&r.id).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                ms_to_dt(r.created_at)?,
                ms_to_dt(r.updated_at)?,
                r.name,
                group_type_from_cols(
                    r.r#type,
                    r.representative1,
                    r.representative2,
                    r.representative3,
                    r.operator,
                )?,
            )
            .map_err(|e| anyhow::anyhow!(e.to_string()))
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_all(&self) -> Result<Vec<Group>, FindError> {
        let rows = sqlx::query!(
            r#"SELECT id, created_at, updated_at, name, type,
                      representative1, representative2, representative3, operator
               FROM groups"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<Group> {
                Group::restore(
                    GroupId::from_str(&r.id).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                    ms_to_dt(r.created_at)?,
                    ms_to_dt(r.updated_at)?,
                    r.name,
                    group_type_from_cols(
                        r.r#type,
                        r.representative1,
                        r.representative2,
                        r.representative3,
                        r.operator,
                    )?,
                )
                .map_err(|e| anyhow::anyhow!(e.to_string()))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert(&self, group: Group) -> Result<(), InsertError> {
        exec_insert(&self.pool, &group).await.map_err(to_insert_error)
    }

    async fn insert_in(&self, tx: &mut SqliteTransaction, group: Group) -> Result<(), InsertError> {
        let conn = tx.conn().map_err(InsertError::InternalError)?;
        exec_insert(conn, &group).await.map_err(to_insert_error)
    }

    async fn update(&self, group: Group) -> Result<(), UpdateError> {
        let affected = exec_update(&self.pool, &group)
            .await
            .map_err(|e| UpdateError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(UpdateError::NotFound);
        }
        Ok(())
    }

    async fn update_in(&self, tx: &mut SqliteTransaction, group: Group) -> Result<(), UpdateError> {
        let conn = tx.conn().map_err(UpdateError::InternalError)?;
        let affected = exec_update(conn, &group)
            .await
            .map_err(|e| UpdateError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(UpdateError::NotFound);
        }
        Ok(())
    }

    async fn delete(&self, id: GroupId) -> Result<(), DeleteError> {
        let affected = exec_delete(&self.pool, id)
            .await
            .map_err(|e| DeleteError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }

    async fn delete_in(&self, tx: &mut SqliteTransaction, id: GroupId) -> Result<(), DeleteError> {
        let conn = tx.conn().map_err(DeleteError::InternalError)?;
        let affected = exec_delete(conn, id)
            .await
            .map_err(|e| DeleteError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }
}
