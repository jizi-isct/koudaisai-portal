use koudaisai_portal_backend::domain::settings::Settings;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct DefaultCase {
    show_occasions_on_portal: bool,
}

pub fn test_default(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: DefaultCase = serde_json::from_str(&contents)?;
    assert_eq!(Settings::default(), Settings::new());
    assert_eq!(
        Settings::default().show_occasions_on_portal(),
        c.show_occasions_on_portal
    );
    Ok(())
}
