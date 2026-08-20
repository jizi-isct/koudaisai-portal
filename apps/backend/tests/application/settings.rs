use crate::application::common::{ActorSpec, build_actor};
use chrono::Utc;
use koudaisai_portal_backend::application::error::ApplicationOperationError;
use koudaisai_portal_backend::infra::memory::MemoryApplication;
use serde::Deserialize;
use std::path::Path;

fn make_app() -> MemoryApplication {
    MemoryApplication::new_memory_app(Utc::now())
}

#[derive(Deserialize)]
struct GetCase {
    actor: ActorSpec,
    expected: String,
}

pub fn test_get_all(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetCase = serde_json::from_str(&contents)?;
        let app = make_app();
        let (_, actor) = build_actor(c.actor);
        let result = app.settings().get_all(&actor).await;

        match c.expected.as_str() {
            "ok" => assert!(!result?.show_occasions_on_portal()),
            "unauthorized" => assert!(matches!(
                result,
                Err(ApplicationOperationError::Unauthorized)
            )),
            other => panic!("unknown expected: {other}"),
        }
        Ok(())
    })
}

pub fn test_get_show_occasions_on_portal(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetCase = serde_json::from_str(&contents)?;
        let app = make_app();
        let (_, actor) = build_actor(c.actor);
        let result = app.settings().get_show_occasions_on_portal(&actor).await;

        match c.expected.as_str() {
            "false" => assert!(!result?),
            "unauthorized" => assert!(matches!(
                result,
                Err(ApplicationOperationError::Unauthorized)
            )),
            other => panic!("unknown expected: {other}"),
        }
        Ok(())
    })
}

#[derive(Deserialize)]
struct ChangeCase {
    actor: ActorSpec,
    enabled: bool,
    expected: String,
}

pub fn test_change_show_occasions_on_portal(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: ChangeCase = serde_json::from_str(&contents)?;
        let app = make_app();
        let (_, actor) = build_actor(c.actor);
        let result = app
            .settings()
            .change_show_occasions_on_portal(&actor, c.enabled)
            .await;

        match c.expected.as_str() {
            "ok" => assert_eq!(result?.show_occasions_on_portal(), c.enabled),
            "unauthorized" => assert!(matches!(
                result,
                Err(ApplicationOperationError::Unauthorized)
            )),
            other => panic!("unknown expected: {other}"),
        }
        Ok(())
    })
}
