use koudaisai_portal_backend::domain::email_address::EmailAddress;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct Case {
    input: String,
    ok: bool,
}

pub fn test_new(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: Case = serde_json::from_str(&contents)?;
    let result = EmailAddress::new(c.input.clone());
    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.input);
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.input);
    }
    Ok(())
}
