use ankhimate_export::{batch, run::Plan, template::RenderedFile};
use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};
fn plan() -> Plan {
    Plan {
        files: vec![
            RenderedFile {
                path: "batch.json".into(),
                contents: "{}".into(),
            },
            RenderedFile {
                path: "0001-Clip/data.json".into(),
                contents: "{}".into(),
            },
        ],
        binaries: vec![],
    }
}
fn prepare(root: &Path) -> Result<(), String> {
    fs::create_dir(root.join("0001-Clip")).map_err(|e| e.to_string())?;
    fs::write(root.join("0001-Clip/data.json"), b"{}").map_err(|e| e.to_string())?;
    fs::write(root.join("batch.json"), b"{}").map_err(|e| e.to_string())
}
#[test]
fn batch_directory_is_published_only_after_its_complete_plan_is_ready() {
    let temp = tempfile::tempdir().unwrap();
    let dst = temp.path().join("Égypte 蛙");
    batch::write_new_directory(
        &plan(),
        &dst,
        |stage| {
            assert!(!dst.exists());
            assert_eq!(
                stage.parent(),
                fs::canonicalize(temp.path()).ok().as_deref()
            );
            prepare(stage)
        },
        || false,
    )
    .unwrap();
    assert!(dst.join("batch.json").is_file());
    assert!(dst.join("0001-Clip/data.json").is_file());
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 1);
}
#[test]
fn batch_refuses_existing_and_newly_appeared_destinations_without_touching_them() {
    let temp = tempfile::tempdir().unwrap();
    let dst = temp.path().join("batch");
    batch::write_new_directory(
        &plan(),
        &dst,
        |stage| {
            prepare(stage)?;
            fs::create_dir(&dst).unwrap();
            fs::write(dst.join("previous.txt"), b"keep").unwrap();
            Ok(())
        },
        || false,
    )
    .unwrap_err();
    assert_eq!(fs::read(dst.join("previous.txt")).unwrap(), b"keep");
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 1);
    assert!(
        batch::write_new_directory(
            &plan(),
            &dst,
            |_| panic!("must refuse before prepare"),
            || false
        )
        .is_err()
    );
    let empty = temp.path().join("empty");
    fs::create_dir(&empty).unwrap();
    assert!(batch::inspect(&plan(), &empty).is_err());
    assert_eq!(fs::read_dir(&empty).unwrap().count(), 0);
    let file = temp.path().join("file");
    fs::write(&file, b"old").unwrap();
    assert!(batch::inspect(&plan(), &file).is_err());
    assert_eq!(fs::read(file).unwrap(), b"old");
}
#[test]
fn batch_cancellation_and_prepare_failure_clean_only_the_owned_temporary() {
    for failure in [0, 1, 2] {
        let temp = tempfile::tempdir().unwrap();
        let prior = temp.path().join(".ankhimate-batch-prior");
        fs::create_dir(&prior).unwrap();
        fs::write(prior.join("backup"), b"keep").unwrap();
        let dst = temp.path().join("batch");
        let cancel = AtomicBool::new(failure == 0);
        assert!(
            batch::write_new_directory(
                &plan(),
                &dst,
                |stage| {
                    prepare(stage)?;
                    if failure == 1 {
                        return Err("second clip failed".into());
                    }
                    cancel.store(true, Ordering::Relaxed);
                    Ok(())
                },
                || cancel.load(Ordering::Relaxed)
            )
            .is_err()
        );
        assert!(!dst.exists());
        assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 1);
        assert_eq!(fs::read(prior.join("backup")).unwrap(), b"keep");
    }
}
#[test]
fn incomplete_extra_or_empty_output_sets_are_not_published() {
    for kind in 0..3 {
        let temp = tempfile::tempdir().unwrap();
        let dst = temp.path().join("batch");
        assert!(
            batch::write_new_directory(
                &plan(),
                &dst,
                |stage| {
                    prepare(stage)?;
                    match kind {
                        0 => fs::remove_file(stage.join("batch.json")).unwrap(),
                        1 => fs::write(stage.join("unexpected.txt"), b"x").unwrap(),
                        _ => fs::write(stage.join("batch.json"), b"").unwrap(),
                    };
                    Ok(())
                },
                || false
            )
            .is_err()
        );
        assert!(!dst.exists());
        assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 0);
    }
}
#[test]
fn residual_child_backups_are_retained_and_reported() {
    let temp = tempfile::tempdir().unwrap();
    let dst = temp.path().join("batch");
    let mut saved = None;
    let result = batch::write_new_directory(
        &plan(),
        &dst,
        |stage| {
            prepare(stage)?;
            let backups = stage.join("0001-Clip/.ankhimate-export-recovery");
            fs::create_dir(&backups).unwrap();
            fs::write(backups.join("old-0"), b"last valid bytes").unwrap();
            saved = Some(backups);
            Err("rollback incomplete".into())
        },
        || false,
    );
    let message = result.unwrap_err().to_string();
    assert!(
        message.contains("retained") && message.contains("backups"),
        "{message}"
    );
    assert!(!dst.exists());
    assert_eq!(
        fs::read(saved.unwrap().join("old-0")).unwrap(),
        b"last valid bytes"
    );
}
#[test]
fn invalid_paths_and_windows_folder_names_are_rejected_before_preparation() {
    for name in [
        "../outside",
        "C:\\escape",
        "CON",
        "COM¹",
        "NUL.txt",
        "bad.",
        "bad ",
        "a/b",
        "a\\b",
        ".ankhimate-export-x",
    ] {
        assert!(batch::folder_name(name).is_err(), "{name}");
    }
    let temp = tempfile::tempdir().unwrap();
    let dst = temp.path().join("batch");
    let mut p = plan();
    p.files[1].path = "../escape.json".into();
    assert!(batch::write_new_directory(&p, &dst, |_| panic!("unsafe plan"), || false).is_err());
    assert!(!dst.exists());
    let mut p = plan();
    p.files[1].path = "BATCH.JSON".into();
    assert!(batch::inspect(&p, &dst).is_err());
}
#[test]
fn destination_appearing_after_the_final_check_is_never_replaced() {
    use std::cell::Cell;
    let temp = tempfile::tempdir().unwrap();
    let dst = temp.path().join("target");
    let ready = Cell::new(false);
    let calls = Cell::new(0);
    let result = batch::write_new_directory(
        &plan(),
        &dst,
        |stage| {
            prepare(stage)?;
            ready.set(true);
            Ok(())
        },
        || {
            if ready.get() {
                calls.set(calls.get() + 1);
                if calls.get() == 2 {
                    fs::create_dir(&dst).unwrap();
                }
            }
            false
        },
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("publication failed")
    );
    assert!(dst.is_dir());
    assert_eq!(fs::read_dir(&dst).unwrap().count(), 0);
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 1);
}
#[cfg(windows)]
#[test]
fn windows_locked_directory_failure_retains_inspectable_staging() {
    use std::{cell::RefCell, os::windows::fs::OpenOptionsExt};
    let temp = tempfile::tempdir().unwrap();
    let dst = temp.path().join("batch");
    let lock = RefCell::new(None);
    let mut saved = None;
    let result = batch::write_new_directory(
        &plan(),
        &dst,
        |stage| {
            prepare(stage)?;
            let file = fs::OpenOptions::new()
                .read(true)
                .custom_flags(0x02000000)
                .share_mode(3)
                .open(stage)
                .map_err(|e| e.to_string())?;
            *lock.borrow_mut() = Some(file);
            saved = Some(stage.to_path_buf());
            Ok(())
        },
        || false,
    );
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("publication failed") && error.contains("retained"),
        "{error}"
    );
    drop(lock);
    assert!(!dst.exists());
    assert!(saved.unwrap().is_dir());
}
