use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::document_category::DocumentCategory;
use serde::Deserialize;
use std::path::Path;

fn make_category() -> DocumentCategory {
    DocumentCategory::register("Test Category".to_string(), Some("📃".to_string()), &FixedClock)
        .unwrap()
}

// --- register ---

#[derive(Deserialize)]
struct RegisterCase {
    title: String,
    emoji: Option<String>,
    ok: bool,
}

pub fn test_register(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: RegisterCase = serde_json::from_str(&contents)?;
    let result = DocumentCategory::register(c.title.clone(), c.emoji.clone(), &FixedClock);
    if c.ok {
        assert!(result.is_ok(), "expected Ok for title={:?} emoji={:?}", c.title, c.emoji);
    } else {
        assert!(result.is_err(), "expected Err for title={:?} emoji={:?}", c.title, c.emoji);
    }
    Ok(())
}

// --- change_title ---

#[derive(Deserialize)]
struct ChangeTitleCase {
    new_title: String,
    ok: bool,
}

pub fn test_change_title(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: ChangeTitleCase = serde_json::from_str(&contents)?;
    let mut cat = make_category();
    let result = cat.change_title(c.new_title.clone(), &FixedClock);
    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_title);
        assert_eq!(cat.title(), c.new_title.as_str());
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_title);
    }
    Ok(())
}

// --- change_emoji ---

#[derive(Deserialize)]
struct ChangeEmojiCase {
    new_emoji: Option<String>,
    ok: bool,
}

pub fn test_change_emoji(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: ChangeEmojiCase = serde_json::from_str(&contents)?;
    let mut cat = make_category();
    let result = cat.change_emoji(c.new_emoji.clone(), &FixedClock);
    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_emoji);
        assert_eq!(cat.emoji(), c.new_emoji.as_deref());
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_emoji);
    }
    Ok(())
}
