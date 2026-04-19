use koudaisai_portal_backend::domain::target_specifier::TargetSpecifier;
use serde::Deserialize;
use std::path::Path;
use std::str::FromStr;

#[derive(Deserialize)]
struct Case {
    input: String,
    ok: bool,
}

pub fn test_from_str(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: Case = serde_json::from_str(&contents)?;
    let result = TargetSpecifier::from_str(&c.input);
    if c.ok {
        let ts = result.unwrap_or_else(|_| panic!("expected Ok for {:?}", c.input));
        // roundtrip: シリアライズして元の文字列に戻ることを確認
        let back: String = (&ts).into();
        assert_eq!(back, c.input, "roundtrip failed for {:?}", c.input);
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.input);
    }
    Ok(())
}
