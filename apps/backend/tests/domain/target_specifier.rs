use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::actor_ctx::ActorContext;
use koudaisai_portal_backend::domain::group::GroupType;
use koudaisai_portal_backend::domain::group_id::GroupId;
use koudaisai_portal_backend::domain::membership::Membership;
use koudaisai_portal_backend::domain::target_specifier::TargetSpecifier;
use koudaisai_portal_backend::domain::user_id::UserId;
use serde::Deserialize;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Deserialize)]
struct FromStrCase {
    input: String,
    ok: bool,
    expected: Option<String>,
}

pub fn test_from_str(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: FromStrCase = serde_json::from_str(&contents)?;
    let result = TargetSpecifier::from_str(&c.input);
    if c.ok {
        let ts = result.expect("expected Ok");
        if let Some(expected_s) = c.expected {
            let actual_s: String = (&ts).into();
            assert_eq!(actual_s, expected_s);
        }
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.input);
    }
    Ok(())
}
