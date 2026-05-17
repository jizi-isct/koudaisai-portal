use koudaisai_portal_backend::domain::group_id::GroupId;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct Case {
    prefix: char,
    index: u16,
    ok: bool,
    display: Option<String>,
    index_str: Option<String>,
}

pub fn test_new(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: Case = serde_json::from_str(&contents)?;
    let result = GroupId::new(c.prefix, c.index);
    if c.ok {
        let id = result.unwrap_or_else(|_| panic!("expected Ok for {}:{}", c.prefix, c.index));
        if let Some(exp) = &c.display {
            assert_eq!(
                id.to_string(),
                *exp,
                "display mismatch for {}:{}",
                c.prefix,
                c.index
            );
        }
        if let Some(exp) = &c.index_str {
            assert_eq!(
                id.index_str(),
                *exp,
                "index_str mismatch for {}:{}",
                c.prefix,
                c.index
            );
        }
    } else {
        assert!(result.is_err(), "expected Err for {}:{}", c.prefix, c.index);
    }
    Ok(())
}
