//! SQLite インフラ層で共有する変換・エラーマッピングヘルパー。

use crate::application::error::InsertError;
use crate::domain::target_specifier::TargetSpecifier;
use chrono::{DateTime, Utc};
use std::str::FromStr;

/// `DateTime<Utc>` を UNIX エポックからのミリ秒(i64)へ。
pub(crate) fn dt_to_ms(dt: DateTime<Utc>) -> i64 {
    dt.timestamp_millis()
}

/// UNIX エポックからのミリ秒(i64)を `DateTime<Utc>` へ。
pub(crate) fn ms_to_dt(ms: i64) -> anyhow::Result<DateTime<Utc>> {
    DateTime::from_timestamp_millis(ms)
        .ok_or_else(|| anyhow::anyhow!("timestamp out of range: {} ms", ms))
}

/// `Vec<TargetSpecifier>` を JSON 配列文字列へ。
pub(crate) fn targets_to_json(targets: &[TargetSpecifier]) -> String {
    let v: Vec<String> = targets.iter().map(|t| t.into()).collect();
    serde_json::to_string(&v).unwrap_or_else(|_| "[]".to_string())
}

/// JSON 配列文字列を `Vec<TargetSpecifier>` へ。
pub(crate) fn targets_from_json(s: &str) -> anyhow::Result<Vec<TargetSpecifier>> {
    let raw: Vec<String> = serde_json::from_str(s)?;
    let mut out = Vec::with_capacity(raw.len());
    for item in raw {
        out.push(TargetSpecifier::from_str(&item).map_err(|e| anyhow::anyhow!(e.to_string()))?);
    }
    Ok(out)
}

/// sqlx のエラーを `InsertError` へ。UNIQUE/PK 制約違反は `Conflict` に振り分ける。
pub(crate) fn to_insert_error(e: sqlx::Error) -> InsertError {
    if let sqlx::Error::Database(db) = &e {
        if db.is_unique_violation() {
            return InsertError::Conflict;
        }
    }
    InsertError::InternalError(e.into())
}
