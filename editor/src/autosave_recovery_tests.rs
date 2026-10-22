use super::*;
use crate::{app_state::AppState, fileops::FileOutcome};

fn copy(store: &RecoveryStore, name: &str) -> PathBuf {
    std::fs::create_dir_all(&store.untitled).unwrap();
    let path = store
        .untitled
        .join(format!("untitled-{name}.ankh.autosave"));
    let mut state = AppState::default();
    state.doc.meta.name = name.into();
    assert!(matches!(
        crate::fileops::write_to(&state, &path),
        FileOutcome::Saved(_)
    ));
    path
}

fn store() -> (tempfile::TempDir, RecoveryStore) {
    let dir = tempfile::tempdir().unwrap();
    let store = RecoveryStore::isolated(dir.path().to_path_buf());
    (dir, store)
}

#[test]
fn t837_empty_startup_does_not_create_directories() {
    let (dir, store) = store();
    let scan = store.scan(None, false, false);
    assert!(scan.copies.is_empty() && scan.warnings.is_empty());
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
}

#[test]
fn t837_crash_copy_ignore_and_restart_preserve_exact_bytes_and_assets() {
    let (_dir, store) = store();
    let path = copy(&store, "crashed");
    let before = std::fs::read(&path).unwrap();
    let offered = store.scan(None, false, false).copies.remove(0);
    let version = offered.version.clone();
    store.ignore(&offered).unwrap();
    let restarted = store.clone();
    assert!(restarted.scan(None, false, false).copies.is_empty());
    assert_eq!(restarted.scan(None, false, true).copies[0].version, version);
    assert_eq!(std::fs::read(path).unwrap(), before);
}

#[test]
fn t837_new_copy_is_not_hidden_by_an_ignored_untitled() {
    let (_dir, store) = store();
    copy(&store, "old");
    store
        .ignore(&store.scan(None, false, false).copies[0])
        .unwrap();
    let new = copy(&store, "new");
    let scan = store.scan(None, false, false);
    assert_eq!(scan.copies.len(), 1);
    assert_eq!(scan.copies[0].autosave, new);
}

#[test]
fn t837_changed_bytes_with_same_length_and_mtime_are_reoffered() {
    let (_dir, store) = store();
    let path = copy(&store, "first");
    let old = store.scan(None, false, false).copies.remove(0);
    store.ignore(&old).unwrap();
    let time = std::fs::metadata(&path).unwrap().modified().unwrap();
    let mut bytes = std::fs::read(&path).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    std::fs::write(&path, bytes).unwrap();
    std::fs::File::options()
        .write(true)
        .open(&path)
        .unwrap()
        .set_times(std::fs::FileTimes::new().set_modified(time))
        .unwrap();
    let fresh = store.scan(None, false, false).copies.remove(0);
    assert_ne!(old.version, fresh.version);
    assert!(old.ensure_unchanged().is_err());
}

#[test]
fn t837_same_bytes_new_timestamp_are_reoffered() {
    let (_dir, store) = store();
    let path = copy(&store, "same");
    let old = store.scan(None, false, false).copies.remove(0);
    store.ignore(&old).unwrap();
    let time =
        std::fs::metadata(&path).unwrap().modified().unwrap() + std::time::Duration::from_secs(60);
    std::fs::File::options()
        .write(true)
        .open(path)
        .unwrap()
        .set_times(std::fs::FileTimes::new().set_modified(time))
        .unwrap();
    assert_eq!(store.scan(None, false, false).copies.len(), 1);
}

#[test]
fn t837_identical_data_at_another_identity_is_not_hidden() {
    let (_dir, store) = store();
    let path = copy(&store, "a");
    store
        .ignore(&store.scan(None, false, false).copies[0])
        .unwrap();
    let other = store.untitled.join("untitled-b.ankh.autosave");
    std::fs::copy(path, &other).unwrap();
    assert_eq!(store.scan(None, false, false).copies[0].autosave, other);
}

#[test]
fn t837_multiple_copies_remain_independently_recoverable_in_stable_order() {
    let (_dir, store) = store();
    for name in ["a", "b", "c"] {
        copy(&store, name);
    }
    let all = store.scan(None, false, false).copies;
    let again = store.scan(None, false, false).copies;
    assert_eq!(
        all.iter().map(|r| &r.version).collect::<Vec<_>>(),
        again.iter().map(|r| &r.version).collect::<Vec<_>>()
    );
    for recovery in all {
        store.ignore(&recovery).unwrap();
    }
    assert!(store.scan(None, false, false).copies.is_empty());
    assert_eq!(store.scan(None, false, true).copies.len(), 3);
}

#[test]
fn t837_corrupt_copy_does_not_hide_valid_and_is_not_deleted() {
    let (_dir, store) = store();
    let valid = copy(&store, "valid");
    let corrupt = store.untitled.join("untitled-bad.ankh.autosave");
    std::fs::write(&corrupt, b"bad").unwrap();
    let all = store.scan(None, false, false).copies;
    assert_eq!(all.len(), 2);
    assert_eq!(all[0].autosave, valid);
    assert_eq!(std::fs::read(corrupt).unwrap(), b"bad");
}

#[test]
fn t837_newer_valid_primary_supersedes_but_invalid_primary_does_not() {
    let (dir, store) = store();
    let primary = dir.path().join("named.ankh");
    let sidecar = path_for(Some(&primary)).unwrap();
    std::fs::write(&sidecar, b"preserve").unwrap();
    std::fs::write(&primary, b"primary").unwrap();
    std::fs::File::options()
        .write(true)
        .open(&primary)
        .unwrap()
        .set_times(
            std::fs::FileTimes::new()
                .set_modified(std::time::SystemTime::now() + std::time::Duration::from_secs(60)),
        )
        .unwrap();
    assert!(store.scan(Some(&primary), true, false).copies.is_empty());
    assert_eq!(store.scan(Some(&primary), false, false).copies.len(), 1);
    assert!(sidecar.exists());
}

#[test]
fn t837_deleted_between_scan_and_action_never_creates_a_receipt() {
    let (_dir, store) = store();
    let path = copy(&store, "gone");
    let offered = store.scan(None, false, false).copies.remove(0);
    std::fs::remove_file(path).unwrap();
    assert!(store.ignore(&offered).is_err());
    assert!(!store.decisions.exists());
    assert!(offered.ensure_unchanged().is_err());
}

#[test]
fn t837_changed_between_scan_and_ignore_never_suppresses_new_version() {
    let (_dir, store) = store();
    let path = copy(&store, "changed");
    let offered = store.scan(None, false, false).copies.remove(0);
    std::fs::write(path, b"new work").unwrap();
    assert!(store.ignore(&offered).is_err());
    assert_eq!(store.scan(None, false, false).copies.len(), 1);
}

#[test]
fn t837_blocked_decision_directory_preserves_copy_and_reports_discovery_errors() {
    let (_dir, store) = store();
    let path = copy(&store, "important");
    let before = std::fs::read(&path).unwrap();
    let offered = store.scan(None, false, false).copies.remove(0);
    std::fs::write(&store.decisions, b"occupied").unwrap();
    assert!(store.ignore(&offered).is_err());
    assert_eq!(std::fs::read(path).unwrap(), before);
    let scan = store.scan(None, false, false);
    assert_eq!(scan.copies.len(), 1);
    assert!(!scan.warnings.is_empty());
}

#[test]
fn t837_simultaneous_instances_ignore_without_overwriting_receipts() {
    let (_dir, store) = store();
    copy(&store, "shared");
    let offered = store.scan(None, false, false).copies.remove(0);
    let second = store.clone();
    store.ignore(&offered).unwrap();
    let before = std::fs::read(store.decisions.join(&offered.version)).unwrap();
    second.ignore(&offered).unwrap();
    assert_eq!(
        std::fs::read(store.decisions.join(&offered.version)).unwrap(),
        before
    );
    assert!(second.scan(None, false, false).copies.is_empty());
}

#[test]
fn t837_partial_receipt_never_hides_or_gets_overwritten() {
    let (_dir, store) = store();
    copy(&store, "partial");
    let offered = store.scan(None, false, false).copies.remove(0);
    std::fs::create_dir_all(&store.decisions).unwrap();
    let marker = store.decisions.join(&offered.version);
    std::fs::write(&marker, b"interrupted").unwrap();
    assert_eq!(store.scan(None, false, false).copies.len(), 1);
    assert!(store.ignore(&offered).is_err());
    assert_eq!(std::fs::read(marker).unwrap(), b"interrupted");
}

#[test]
fn t837_legacy_unsaved_copy_is_covered_by_the_same_contract() {
    let (_dir, store) = store();
    std::fs::write(&store.legacy, b"legacy work").unwrap();
    let offered = store.scan(None, false, false).copies.remove(0);
    store.ignore(&offered).unwrap();
    assert!(store.scan(None, false, false).copies.is_empty());
    assert_eq!(std::fs::read(&store.legacy).unwrap(), b"legacy work");
}

#[cfg(windows)]
#[test]
fn t837_windows_locked_receipt_does_not_hide_the_copy_or_overwrite_lock() {
    use std::os::windows::fs::OpenOptionsExt;
    let (_dir, store) = store();
    copy(&store, "locked");
    let offered = store.scan(None, false, false).copies.remove(0);
    store.ignore(&offered).unwrap();
    let held = std::fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(store.decisions.join(&offered.version))
        .unwrap();
    let scan = store.scan(None, false, false);
    assert_eq!(scan.copies.len(), 1);
    assert!(!scan.warnings.is_empty());
    assert!(store.ignore(&offered).is_err());
    drop(held);
    assert!(store.scan(None, false, false).copies.is_empty());
}

#[cfg(windows)]
#[test]
fn t837_windows_locked_copy_is_preserved_and_discovery_reports_the_failure() {
    use std::os::windows::fs::OpenOptionsExt;
    let (_dir, store) = store();
    let path = copy(&store, "held");
    let before = std::fs::read(&path).unwrap();
    let held = std::fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(&path)
        .unwrap();
    let scan = store.scan(None, false, false);
    assert!(scan.copies.is_empty());
    assert!(!scan.warnings.is_empty());
    drop(held);
    assert_eq!(std::fs::read(&path).unwrap(), before);
    assert_eq!(store.scan(None, false, false).copies.len(), 1);
}
