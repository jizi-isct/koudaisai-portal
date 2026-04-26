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

#[test]
fn test_does_actor_match() {
    let user_id = UserId::new(Uuid::new_v4());
    let group_id = GroupId::from_str("M-001").unwrap();
    let memberships = vec![Membership::new(group_id, user_id, &FixedClock)];

    let ctx_general = ActorContext::User {
        user_id,
        memberships: memberships.clone(),
        group_type: GroupType::GeneralProject {
            representative1: user_id,
            representative2: user_id,
            representative3: user_id,
        },
    };

    let ctx_nologin = ActorContext::NoLogin;

    // GroupTypeProjectGeneral
    assert!(TargetSpecifier::GroupTypeProjectGeneral.does_actor_match(&ctx_general));
    assert!(!TargetSpecifier::GroupTypeProjectGeneral.does_actor_match(&ctx_nologin));

    // GroupId
    assert!(TargetSpecifier::GroupId(group_id).does_actor_match(&ctx_general));
    assert!(
        !TargetSpecifier::GroupId(GroupId::from_str("M-002").unwrap())
            .does_actor_match(&ctx_general)
    );

    // UserId
    assert!(TargetSpecifier::UserId(user_id).does_actor_match(&ctx_general));
    assert!(!TargetSpecifier::UserId(UserId::new(Uuid::new_v4())).does_actor_match(&ctx_general));

    // UserNologin
    // 未ログインユーザーはもちろんマッチする
    assert!(TargetSpecifier::UserNologin.does_actor_match(&ctx_nologin));
    // ログイン済みユーザーも、一般公開ドキュメントを見られるように設計変更したためマッチする
    assert!(TargetSpecifier::UserNologin.does_actor_match(&ctx_general));
}

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
