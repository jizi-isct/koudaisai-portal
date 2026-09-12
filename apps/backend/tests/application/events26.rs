use crate::application::common::{ActorSpec, build_actor};
use koudaisai_portal_backend::application::error::ApplicationOperationError;
use koudaisai_portal_backend::application::events26::Events26App;
use koudaisai_portal_backend::application::ports::events26_api::Events26Api;
use koudaisai_portal_backend::infra::memory::events26_api_impl::MemoryEvents26Api;
use serde::Deserialize;
use std::path::Path;

const PROJECT_ID: &str = "I-100";

#[derive(Deserialize)]
struct UpdateCase {
    actor: ActorSpec,
    additional_info: String,
    expected: String,
}

pub fn test_update_own_project_additional_info(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: UpdateCase = serde_json::from_str(&contents)?;
        let api = MemoryEvents26Api::new();
        let app = Events26App::new(&api);
        let (_, actor) = build_actor(c.actor);

        let result = app
            .update_own_project_additional_info(&actor, &c.additional_info)
            .await;

        match c.expected.as_str() {
            "ok" => {
                result.expect("expected update Ok");
                assert_eq!(api.additional_info(PROJECT_ID), Some(c.additional_info));
            }
            "unauthorized" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::Unauthorized)
                ));
                assert_eq!(api.additional_info(PROJECT_ID), None);
            }
            expected => panic!("unknown expected: {expected}"),
        }
        Ok(())
    })
}

#[derive(Deserialize)]
struct DeleteCase {
    actor: ActorSpec,
    #[serde(default)]
    additional_info_exists: bool,
    expected: String,
}

pub fn test_delete_own_project_additional_info(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: DeleteCase = serde_json::from_str(&contents)?;
        let api = MemoryEvents26Api::new();
        if c.additional_info_exists {
            api.update_project_additional_info(PROJECT_ID, "既存の追加情報")
                .await
                .unwrap();
        }

        let app = Events26App::new(&api);
        let (_, actor) = build_actor(c.actor);
        let result = app.delete_own_project_additional_info(&actor).await;

        match c.expected.as_str() {
            "ok" => {
                result.expect("expected delete Ok");
                assert_eq!(api.additional_info(PROJECT_ID), None);
            }
            "unauthorized" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::Unauthorized)
                ));
                assert_eq!(
                    api.additional_info(PROJECT_ID),
                    c.additional_info_exists
                        .then(|| "既存の追加情報".to_string())
                );
            }
            expected => panic!("unknown expected: {expected}"),
        }
        Ok(())
    })
}
