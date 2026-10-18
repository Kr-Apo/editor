//! M001b: disk failures must not turn a valid export into a partial one.
use ankhimate_export::run::{self, Plan};
use ankhimate_export::template::RenderedFile;

fn text(path: &str, contents: &str) -> RenderedFile {
    RenderedFile {
        path: path.into(),
        contents: contents.into(),
    }
}

#[test]
fn an_empty_output_directory_is_refused() {
    assert!(run::write(&Plan::default(), std::path::Path::new("")).is_err());
}

#[test]
fn malformed_json_in_a_later_output_preserves_all_previous_files() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("first.txt"), "old first").unwrap();
    std::fs::write(dir.path().join("skeleton.json"), "{\"valid\":true}").unwrap();
    let plan = Plan {
        files: vec![
            text("first.txt", "new first"),
            text("skeleton.json", "{\"name\":\"a\"b\"}"),
        ],
        binaries: vec![],
    };
    let err = run::write(&plan, dir.path()).expect_err("malformed JSON must not be published");
    assert!(err.to_string().contains("invalid JSON"), "{err}");
    assert_eq!(
        std::fs::read(dir.path().join("first.txt")).unwrap(),
        b"old first"
    );
    assert_eq!(
        std::fs::read(dir.path().join("skeleton.json")).unwrap(),
        b"{\"valid\":true}"
    );
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 2);
}

#[test]
fn malformed_binary_json_is_rejected_before_creating_output_directory() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("output");
    let plan = Plan {
        files: vec![],
        binaries: vec![("data.JSON".into(), vec![0xff, 0])],
    };
    assert!(run::write(&plan, &root).is_err());
    assert!(!root.exists());
}

#[test]
fn an_invalid_later_destination_preserves_every_previous_file() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("skeleton.json"), "old skeleton").unwrap();
    std::fs::create_dir(dir.path().join("atlas.png")).unwrap();
    let plan = Plan {
        files: vec![text("skeleton.json", "{\"new\":true}")],
        binaries: vec![("atlas.png".into(), vec![1, 2, 3])],
    };
    assert!(run::write(&plan, dir.path()).is_err());
    assert_eq!(
        std::fs::read(dir.path().join("skeleton.json")).unwrap(),
        b"old skeleton"
    );
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 2);
}

#[test]
fn direct_plans_cannot_escape_the_output_directory() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("output");
    let plan = Plan {
        files: vec![text("../outside.txt", "lost")],
        binaries: vec![],
    };
    assert!(run::write(&plan, &root).is_err());
    assert!(!dir.path().join("outside.txt").exists());
    assert!(!root.exists());
}

#[test]
fn text_and_binary_collisions_are_rejected_before_writing() {
    let dir = tempfile::tempdir().unwrap();
    let plan = Plan {
        files: vec![text("atlas.png", "text")],
        binaries: vec![("atlas.png".into(), vec![1, 2, 3])],
    };
    assert!(run::write(&plan, dir.path()).is_err());
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
}

#[test]
fn file_directory_and_normalized_collisions_are_rejected_before_writing() {
    for paths in [
        ["a", "a/b.txt"],
        ["a/b.txt", "a\\b.txt"],
        ["a.txt", "A.TXT"],
    ] {
        let dir = tempfile::tempdir().unwrap();
        let plan = Plan {
            files: paths.iter().map(|p| text(p, "new")).collect(),
            binaries: vec![],
        };
        assert!(run::write(&plan, dir.path()).is_err(), "accepted {paths:?}");
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    }
}

#[test]
fn nonportable_windows_paths_are_rejected_before_creating_directories() {
    for path in [
        "file:stream",
        "CON.json",
        "CON .json",
        "aux/x.png",
        "x./y.txt",
        "a /b.txt",
        "quote\".txt",
        "a\u{0}b",
    ] {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("output");
        let plan = Plan {
            files: vec![text(path, "new")],
            binaries: vec![],
        };
        assert!(run::write(&plan, &root).is_err(), "accepted {path:?}");
        assert!(!root.exists(), "created directories for {path:?}");
    }
}

#[test]
fn successful_staged_export_replaces_text_and_binary_and_keeps_orphans() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("skeleton.json"), "old").unwrap();
    std::fs::write(dir.path().join("orphan.txt"), "mine").unwrap();
    let plan = Plan {
        files: vec![text("skeleton.json", "\"new\"")],
        binaries: vec![("images/atlas.png".into(), vec![1, 2, 3])],
    };
    run::write(&plan, dir.path()).unwrap();
    assert_eq!(
        std::fs::read(dir.path().join("skeleton.json")).unwrap(),
        b"\"new\""
    );
    assert_eq!(
        std::fs::read(dir.path().join("images/atlas.png")).unwrap(),
        [1, 2, 3]
    );
    assert_eq!(
        std::fs::read(dir.path().join("orphan.txt")).unwrap(),
        b"mine"
    );
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 3);
}

#[test]
fn a_read_only_destination_preserves_previous_outputs() {
    let dir = tempfile::tempdir().unwrap();
    let blocked = dir.path().join("blocked.txt");
    std::fs::write(dir.path().join("first.txt"), "old first").unwrap();
    std::fs::write(&blocked, "old blocked").unwrap();
    let original = std::fs::metadata(&blocked).unwrap().permissions();
    let mut readonly = original.clone();
    readonly.set_readonly(true);
    std::fs::set_permissions(&blocked, readonly).unwrap();
    let plan = Plan {
        files: vec![text("first.txt", "new"), text("blocked.txt", "new")],
        binaries: vec![],
    };
    let result = run::write(&plan, dir.path());
    std::fs::set_permissions(&blocked, original).unwrap();
    assert!(result.is_err());
    assert_eq!(
        std::fs::read(dir.path().join("first.txt")).unwrap(),
        b"old first"
    );
    assert_eq!(std::fs::read(&blocked).unwrap(), b"old blocked");
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 2);
}

#[test]
fn atlas_and_copied_image_paths_are_validated_during_planning() {
    use ankhimate_core::assets::{AssetDb, ImageAsset};
    use ankhimate_export::{
        preset::{Cadence, Template},
        presets,
    };
    let project: ankhimate_formats::schema::Project =
        serde_json::from_str(include_str!("fixtures/runtime_axes.ankh.json")).unwrap();
    let mut preset = presets::default_preset();
    preset.atlas.enabled = false;
    preset.copy_images = true;
    let mut assets = AssetDb::new();
    assets.add(ImageAsset::new("../../escape", vec![1], 1, 1));
    assert!(run::plan(&project, &assets, &preset).is_err());
    let mut assets = AssetDb::new();
    assets.add(ImageAsset::new("head", vec![1], 1, 1));
    preset.templates.push(Template {
        name: "collision".into(),
        output_path: "images/head.png".into(),
        body: "text".into(),
        per: Cadence::Once,
    });
    assert!(run::plan(&project, &assets, &preset).is_err());
    preset = presets::default_preset();
    preset.templates[0].output_path = "atlas.png".into();
    // A genuine PNG gives the atlas a page, hence a text/image collision.
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(1, 1, image::Rgba([255, 255, 255, 255]))
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    let mut assets = AssetDb::new();
    assets.add(ImageAsset::new("head", bytes.into_inner(), 1, 1));
    assert!(run::plan(&project, &assets, &preset).is_err());
    preset.templates[0].output_path = "skeleton.json".into();
    preset.atlas.stem = "../outside".into();
    assert!(run::plan(&project, &assets, &preset).is_err());
}

#[cfg(windows)]
#[test]
fn windows_junctions_cannot_redirect_writes_or_orphan_surveys() {
    use std::os::windows::process::CommandExt;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("output");
    let outside = dir.path().join("outside");
    let junction = root.join("link");
    std::fs::create_dir(&root).unwrap();
    std::fs::create_dir(&outside).unwrap();
    std::fs::write(root.join("first.txt"), "old first").unwrap();
    std::fs::write(outside.join("old.txt"), "outside data").unwrap();
    let status = std::process::Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", "$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:ANKH_TEST_LINK -Target $env:ANKH_TEST_TARGET | Out-Null"])
        .env("ANKH_TEST_LINK", &junction).env("ANKH_TEST_TARGET", &outside)
        .creation_flags(0x08000000).status().unwrap();
    assert!(
        status.success(),
        "the junction fixture must be created, not skipped"
    );
    for path in ["link/new.txt", "link/old.txt"] {
        let plan = Plan {
            files: vec![text("first.txt", "new"), text(path, "bad")],
            binaries: vec![],
        };
        assert!(run::write(&plan, &root).is_err());
        assert!(
            !plan
                .survey(&root)
                .orphans
                .iter()
                .any(|p| p.starts_with("link/"))
        );
    }
    let plan = Plan {
        files: vec![text("new.txt", "bad")],
        binaries: vec![],
    };
    assert!(run::write(&plan, &junction).is_err());
    assert!(
        plan.survey(&junction).orphans.is_empty(),
        "survey must not follow a linked output root"
    );
    assert_eq!(std::fs::read(root.join("first.txt")).unwrap(), b"old first");
    assert_eq!(
        std::fs::read(outside.join("old.txt")).unwrap(),
        b"outside data"
    );
    assert_eq!(std::fs::read_dir(&outside).unwrap().count(), 1);
    std::fs::remove_dir(junction).unwrap();
}

#[cfg(unix)]
#[test]
fn symlinks_cannot_redirect_writes_or_orphan_surveys() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("output");
    let outside = dir.path().join("outside");
    std::fs::create_dir(&root).unwrap();
    std::fs::create_dir(&outside).unwrap();
    std::os::unix::fs::symlink(&outside, root.join("link")).unwrap();
    let plan = Plan {
        files: vec![text("link/new.txt", "bad")],
        binaries: vec![],
    };
    assert!(run::write(&plan, &root).is_err());
    assert!(plan.survey(&root).orphans.contains(&"link".into()));
    assert!(!outside.join("new.txt").exists());
}
