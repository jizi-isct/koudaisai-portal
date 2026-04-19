use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::{
    email_address::EmailAddress,
    password_credentials::PasswordCredentials,
    user::{User, UserStatus},
    user_id::UserId,
};
use serde::Deserialize;
use std::path::Path;
use uuid::Uuid;

fn make_user() -> User {
    User::register(
        UserId::new(Uuid::new_v4()),
        "Test User".to_string(),
        EmailAddress::new("test@example.com".to_string()).unwrap(),
        &FixedClock,
    )
    .unwrap()
}

// --- register ---

#[derive(Deserialize)]
struct RegisterCase {
    name: String,
    ok: bool,
}

pub fn test_register(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: RegisterCase = serde_json::from_str(&contents)?;
    let result = User::register(
        UserId::new(Uuid::new_v4()),
        c.name.clone(),
        EmailAddress::new("test@example.com".to_string()).unwrap(),
        &FixedClock,
    );
    if c.ok {
        let user = result.expect(&format!("expected Ok for {:?}", c.name));
        assert_eq!(user.name(), c.name.as_str());
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.name);
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
    let mut user = make_user();
    let result = user.rename(c.new_name.clone(), &FixedClock);
    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_name);
        assert_eq!(user.name(), c.new_name.as_str());
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_name);
    }
    Ok(())
}

// --- status_transition ---

#[derive(Deserialize)]
struct StatusTransitionCase {
    initial: String,
    operation: String,
    ok: bool,
}

pub fn test_status_transition(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: StatusTransitionCase = serde_json::from_str(&contents)?;
    let mut user = make_user();
    let creds = PasswordCredentials::new("phc".to_string(), &FixedClock).unwrap();

    match c.initial.as_str() {
        "registered" => {}
        "active"     => { user.activate(creds.clone(), &FixedClock).unwrap(); }
        "deactivated" => {
            user.activate(creds.clone(), &FixedClock).unwrap();
            user.deactivate("reason".to_string(), &FixedClock).unwrap();
        }
        s => panic!("unknown initial state: {s}"),
    }

    let result = match c.operation.as_str() {
        "activate"   => user.activate(creds.clone(), &FixedClock).map_err(|_| ()),
        "deactivate" => user.deactivate("reason".to_string(), &FixedClock).map_err(|_| ()),
        s => panic!("unknown operation: {s}"),
    };

    if c.ok {
        assert!(result.is_ok(), "expected Ok: {:?} → {:?}", c.initial, c.operation);
        match c.operation.as_str() {
            "activate"   => assert!(matches!(user.status(), UserStatus::Active { .. }), "expected Active status after activate"),
            "deactivate" => assert!(matches!(user.status(), UserStatus::Deactivated { .. }), "expected Deactivated status after deactivate"),
            _ => unreachable!(),
        }
    } else {
        assert!(result.is_err(), "expected Err: {:?} → {:?}", c.initial, c.operation);
    }
    Ok(())
}
