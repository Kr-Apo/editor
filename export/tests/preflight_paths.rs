use ankhimate_export::{
    run::{self, Plan},
    template::RenderedFile,
};
fn plan(path: &str, contents: &str) -> Plan {
    Plan {
        files: vec![RenderedFile {
            path: path.into(),
            contents: contents.into(),
        }],
        binaries: vec![],
    }
}
#[test]
fn inspection_reuses_portable_path_and_json_checks_without_creating_a_directory() {
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("uncreated");
    for (path, contents) in [
        ("../escape.json", "{}"),
        ("CON.json", "{}"),
        ("bad.json", "{bad"),
    ] {
        assert!(run::inspect(&plan(path, contents), &output).is_err());
        assert!(!output.exists());
    }
    assert!(run::inspect(&plan("nested/valid.json", "{}"), &output).is_ok());
    assert!(!output.exists());
}
#[test]
fn preflight_does_not_reserve_paths_or_bypass_later_publisher_checks() {
    let temp = tempfile::tempdir().unwrap();
    let plan = plan("data.json", "{}");
    run::inspect(&plan, temp.path()).unwrap();
    std::fs::write(
        temp.path().join(".ankhimate-export.lock"),
        b"existing owner",
    )
    .unwrap();
    assert!(run::inspect(&plan, temp.path()).is_err());
    assert!(run::write(&plan, temp.path()).is_err());
    assert!(!temp.path().join("data.json").exists());
    assert_eq!(
        std::fs::read(temp.path().join(".ankhimate-export.lock")).unwrap(),
        b"existing owner"
    );
}

#[test]
fn reviewed_replacements_are_checked_again_by_the_publisher() {
    let temp = tempfile::tempdir().unwrap();
    let plan = plan("data.json", "{}");
    std::fs::write(temp.path().join("data.json"), b"previous valid export").unwrap();
    assert!(run::write_reviewed(&plan, temp.path(), &[]).is_err());
    assert_eq!(
        std::fs::read(temp.path().join("data.json")).unwrap(),
        b"previous valid export"
    );
    assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 1);
    run::write_reviewed(&plan, temp.path(), &["data.json".into()]).unwrap();
    assert_eq!(std::fs::read(temp.path().join("data.json")).unwrap(), b"{}");
}

#[test]
fn survey_uses_the_publishers_normalized_names_instead_of_false_orphans() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir(temp.path().join("nested")).unwrap();
    std::fs::write(temp.path().join("nested/data.json"), b"old").unwrap();
    std::fs::write(temp.path().join("orphan.txt"), b"keep").unwrap();
    let plan = plan("nested\\data.json", "{}");
    let survey = plan.survey(temp.path());
    assert_eq!(survey.replaced, vec!["nested/data.json"]);
    assert_eq!(survey.orphans, vec!["orphan.txt"]);
    run::write_reviewed(&plan, temp.path(), &survey.replaced).unwrap();
    assert_eq!(
        std::fs::read(temp.path().join("nested/data.json")).unwrap(),
        b"{}"
    );
}

#[test]
#[cfg(windows)]
fn windows_case_alias_replacements_are_not_also_announced_as_preserved_orphans() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("DATA.JSON"), b"{}").unwrap();
    let plan = plan("data.json", "{}");
    let survey = plan.survey(temp.path());
    assert_eq!(survey.replaced, vec!["data.json"]);
    assert!(
        survey.orphans.is_empty(),
        "a replaced Windows case alias cannot also be promised as a preserved orphan"
    );
}
