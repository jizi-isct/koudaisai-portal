use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::{
    approval_request::{ApprovalRequest, ApprovalRequestType},
    approval_request_id::ApprovalRequestId,
    user_id::UserId,
};
use serde::Deserialize;
use std::path::Path;
use uuid::Uuid;

fn make_pending() -> ApprovalRequest {
    ApprovalRequest::create(
        ApprovalRequestId::generate(),
        UserId::new(Uuid::new_v4()),
        ApprovalRequestType::EditExhibitionInfo { description: None, icon_key: None },
        "test reason".to_string(),
        &FixedClock,
    )
    .unwrap()
}

// --- create ---

#[derive(Deserialize)]
struct CreateCase {
    reason: String,
    ok: bool,
}

pub fn test_create(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: CreateCase = serde_json::from_str(&contents)?;
    let result = ApprovalRequest::create(
        ApprovalRequestId::generate(),
        UserId::new(Uuid::new_v4()),
        ApprovalRequestType::EditExhibitionInfo { description: None, icon_key: None },
        c.reason.clone(),
        &FixedClock,
    );
    if c.ok {
        assert!(result.is_ok(), "expected Ok for reason {:?}", c.reason);
    } else {
        assert!(result.is_err(), "expected Err for reason {:?}", c.reason);
    }
    Ok(())
}

// --- transition ---

#[derive(Deserialize)]
struct TransitionCase {
    initial: String,
    operation: String,
    ok: bool,
}

pub fn test_transition(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: TransitionCase = serde_json::from_str(&contents)?;
    let mut req = make_pending();
    let user = UserId::new(Uuid::new_v4());

    match c.initial.as_str() {
        "pending"  => {}
        "approved" => { req.approve(user, None, &FixedClock).unwrap(); }
        "rejected" => { req.reject(user, None, &FixedClock).unwrap(); }
        "closed"   => { req.close(&FixedClock).unwrap(); }
        s => panic!("unknown initial state: {s}"),
    }

    let result = match c.operation.as_str() {
        "approve" => req.approve(user, None, &FixedClock).map_err(|_| ()),
        "reject"  => req.reject(user, None, &FixedClock).map_err(|_| ()),
        "close"   => req.close(&FixedClock).map_err(|_| ()),
        s => panic!("unknown operation: {s}"),
    };

    if c.ok {
        assert!(result.is_ok(), "expected Ok: {:?} → {:?}", c.initial, c.operation);
    } else {
        assert!(result.is_err(), "expected Err: {:?} → {:?}", c.initial, c.operation);
    }
    Ok(())
}
