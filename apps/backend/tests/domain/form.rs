use crate::domain::common::FixedClock;
use chrono::Utc;
use koudaisai_portal_backend::domain::{
    form::{Form, FormType},
    target_specifier::TargetSpecifier,
};
use serde::Deserialize;
use std::path::Path;
use uuid::Uuid;

fn make_form() -> Form {
    Form::register(
        Uuid::new_v4(),
        vec![TargetSpecifier::GroupTypePress],
        "Test Form".to_string(),
        "Test Summary".to_string(),
        Utc::now() + chrono::Duration::days(7),
        FormType::TypeExternal {
            form_url: "https://example.com/form".to_string(),
        },
        &FixedClock,
    )
    .unwrap()
}

// --- register ---

#[derive(Deserialize)]
struct RegisterCase {
    name: String,
    summary: String,
    ok: bool,
}

pub fn test_register(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: RegisterCase = serde_json::from_str(&contents)?;
    let result = Form::register(
        Uuid::new_v4(),
        vec![TargetSpecifier::GroupTypePress],
        c.name.clone(),
        c.summary.clone(),
        Utc::now() + chrono::Duration::days(7),
        FormType::TypeExternal {
            form_url: "https://example.com/form".to_string(),
        },
        &FixedClock,
    );
    if c.ok {
        let form = result.unwrap_or_else(|_| panic!("expected Ok for name={:?} summary={:?}",
            c.name, c.summary));
        assert_eq!(form.name(), c.name.as_str());
        assert_eq!(form.summary(), c.summary.as_str());
    } else {
        assert!(
            result.is_err(),
            "expected Err for name={:?} summary={:?}",
            c.name,
            c.summary
        );
    }
    Ok(())
}

// --- rename ---

#[derive(Deserialize)]
struct RenameCase {
    new_name: String,
    ok: bool,
}

pub fn test_rename(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: RenameCase = serde_json::from_str(&contents)?;
    let mut form = make_form();
    let result = form.rename(c.new_name.clone(), Uuid::new_v4(), &FixedClock);
    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_name);
        assert_eq!(form.name(), c.new_name.as_str());
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_name);
    }
    Ok(())
}

// --- change_summary ---

#[derive(Deserialize)]
struct ChangeSummaryCase {
    new_summary: String,
    ok: bool,
}

pub fn test_change_summary(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: ChangeSummaryCase = serde_json::from_str(&contents)?;
    let mut form = make_form();
    let result = form.change_summary(c.new_summary.clone(), Uuid::new_v4(), &FixedClock);
    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_summary);
        assert_eq!(form.summary(), c.new_summary.as_str());
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_summary);
    }
    Ok(())
}
