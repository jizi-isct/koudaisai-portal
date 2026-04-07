use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::password_credentials::PasswordCredentials;
use serde::Deserialize;
use std::path::Path;

// --- new ---

#[derive(Deserialize)]
struct NewCase {
    phc: String,
}

pub fn test_new(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: NewCase = serde_json::from_str(&contents)?;
    let creds = PasswordCredentials::new(c.phc.clone(), &FixedClock)
        .unwrap_or_else(|_| panic!("expected Ok for phc {:?}", c.phc));
    assert_eq!(creds.phc(), c.phc, "phc mismatch");
    Ok(())
}
