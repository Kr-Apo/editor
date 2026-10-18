use ankhimate_formats::{self as formats, reimport_guard};
use serde_json::json;
#[test]
fn canonical_author_fixtures_are_read_only_and_safe_to_review() {
    for file in [
        "minimal.ankh",
        "tentacle.ankh",
        "../../../demos/m0070-crapaud-pharaon/projet/Crapaud-Pharaon.ankh",
    ] {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(file);
        let before = std::fs::read(&path).unwrap();
        reimport_guard::verify_source(&path).unwrap();
        assert_eq!(std::fs::read(path).unwrap(), before);
    }
}
#[test]
fn unknown_or_nonrepresentable_author_fields_are_refused_including_nested_ones() {
    let dir = tempfile::tempdir().unwrap();
    for raw in [
        json!({"version":1,"name":"future","fps":60,"future":42}),
        json!({"version":1,"name":"future","fps":60,"bones":[{"name":"root","length":1,"future":42}]}),
        json!({"version":1,"name":"future","fps":60,"groups":[{"name":"g","future":42}]}),
    ] {
        let path = dir.path().join("future.ankh.json");
        std::fs::write(&path, raw.to_string()).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        assert!(reimport_guard::verify_source(&path).is_err());
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }
}
#[test]
fn preserved_opaque_presets_do_not_trigger_an_unknown_field_refusal() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("opaque.ankh.json");
    std::fs::write(
        &path,
        json!({"version":1,"name":"opaque","fps":60,"export_presets":[{"future":{"any":[1,2,3]}}]})
            .to_string(),
    )
    .unwrap();
    reimport_guard::verify_source(&path).unwrap();
    let p = formats::load(&path).unwrap().0;
    assert_eq!(p.export_presets[0]["future"]["any"], json!([1, 2, 3]));
}
#[test]
fn missing_invalid_and_future_sources_are_refused() {
    let dir = tempfile::tempdir().unwrap();
    assert!(reimport_guard::verify_source(&dir.path().join("absent.ankh")).is_err());
    for bytes in [
        b"not a native author project".as_slice(),
        b"{\"version\":2,\"name\":\"future\",\"fps\":60}".as_slice(),
    ] {
        let path = dir.path().join("invalid.ankh");
        std::fs::write(&path, bytes).unwrap();
        assert!(reimport_guard::verify_source(&path).is_err());
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }
}
#[test]
fn binary_and_compact_unknown_fields_are_refused_without_rewriting_source() {
    let dir = tempfile::tempdir().unwrap();
    let raw = json!({"a":1,"b":"future","c":60,"~future_root":42});
    let payload = rmp_serde::to_vec(&raw).unwrap();
    let binary = formats::container::encode(&payload).unwrap();
    for (name, bytes) in [
        ("future.ankh", binary),
        ("future.ankh.min.json", serde_json::to_vec(&raw).unwrap()),
    ] {
        let path = dir.path().join(name);
        std::fs::write(&path, &bytes).unwrap();
        let error = reimport_guard::verify_source(&path).unwrap_err();
        assert!(error.contains("future_root"), "{error}");
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }
}
