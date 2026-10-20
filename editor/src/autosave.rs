//! Crash recovery (T-701).
//!
//! An editor that loses an hour of work to a crash has taught you to save
//! compulsively, which is a tax it charges for the rest of its life. This writes
//! a spare copy on a timer so there is something to come back to.
//!
//! # What it is not
//!
//! Not a save. The autosave file sits **beside** the project under a different
//! extension and the project's own file is never touched — a background writer
//! that overwrites the thing you were editing is a way to lose two copies
//! instead of one. `current_path` does not move, the title bar does not change,
//! and the user is never told "saved" for something they did not ask for.
//!
//! # When it writes
//!
//! Only when persisted document content has changed. A timer
//! alone would rewrite an untouched project every interval, and the one thing
//! worse than no autosave is one that burns a laptop's disk while you read.
//!
//! # Recovery
//!
//! On startup, an autosave newer than its project is offered. Offered, not
//! applied: the user knows whether the last session ended badly and we do not,
//! and silently opening a different file than the one that was double-clicked
//! is the kind of helpfulness people uninstall software over.
//!
//! An unsaved document autosaves into the temp directory, since there is no
//! project directory to sit beside yet.

use crate::app_state::AppState;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
#[cfg(test)]
#[path = "autosave_clock_tests.rs"]
mod clock_tests;
static NEXT_UNTITLED: AtomicU64 = AtomicU64::new(0);

/// Extension appended to the project's own name: `walk.ankh.autosave`.
///
/// Appended rather than replacing `.ankh` so the file cannot be mistaken for a
/// project by a file dialog, and so a directory listing sorts it next to the
/// thing it belongs to.
const EXT: &str = "ankh.autosave";

/// How often to consider writing, in seconds.
///
/// Two minutes is the interval most editors settle on: short enough that what
/// you lose is an edit rather than an idea, long enough that a large rig's write
/// does not become the thing you notice about the program.
pub const DEFAULT_INTERVAL_SECS: u64 = 120;

/// The autosave path for a project, or for the unsaved document.
pub fn path_for(project: Option<&Path>) -> Option<PathBuf> {
    match project {
        Some(path) => {
            let name = path.file_name()?.to_str()?;
            Some(path.with_file_name(format!("{name}.autosave")))
        }
        // Legacy recovery location, read only by recovery discovery. New
        // untitled writes use a separate path per process and document.
        None => Some(std::env::temp_dir().join(format!("untitled.{EXT}"))),
    }
}

/// Timer and dirty-tracking for the autosave.
pub struct Autosave {
    /// Seconds between attempts. Zero disables.
    pub interval_secs: u64,
    /// Monotonic start of the pending interval, independent of rendered frames.
    started_at: Option<Instant>,
    /// Where the last write went, for the status line and for cleanup.
    last_written: Option<PathBuf>,
    unsaved_path: Option<PathBuf>,
    saved_document: Option<serde_json::Value>,
    pub last_error: Option<String>,
}

impl Default for Autosave {
    fn default() -> Self {
        Self {
            interval_secs: DEFAULT_INTERVAL_SECS,
            started_at: None,
            last_written: None,
            unsaved_path: None,
            saved_document: None,
            last_error: None,
        }
    }
}

impl Autosave {
    /// Start with `interval_secs` between attempts; 0 disables.
    pub fn new(interval_secs: u64) -> Self {
        Self {
            interval_secs,
            ..Default::default()
        }
    }

    fn advance(&mut self, now: Instant) -> Schedule {
        if self.interval_secs == 0 {
            self.started_at = None;
            return Schedule::Idle;
        }
        let started_at = self.started_at.get_or_insert(now);
        let elapsed = now.saturating_duration_since(*started_at);
        let interval = Duration::from_secs(self.interval_secs);
        if elapsed < interval {
            return Schedule::Wait(interval - elapsed);
        }
        // At most one attempt, even after a long suspension. Failed writes retry
        // after a whole interval rather than requesting immediate repaint.
        *started_at = now;
        Schedule::Due
    }

    /// Caller supplies real monotonic time (or controlled time in tests) and
    /// arms egui's delayed repaint only for `Wait`. Clean content and pending
    /// recovery offers have no wakeup. Later edits start a fresh interval.
    pub fn schedule(&mut self, now: Instant, state: &AppState, eligible: bool) -> Schedule {
        if !eligible || self.interval_secs == 0 {
            self.started_at = None;
            return Schedule::Idle;
        }
        let current = snapshot(state);
        if current.is_some() && current == self.saved_document {
            self.started_at = None;
            return Schedule::Idle;
        }
        self.advance(now)
    }

    /// Write the autosave beside `project`, or to the temp directory.
    ///
    /// Failure is logged and exposed in `last_error` for the status line.
    pub fn write(&mut self, state: &AppState, project: Option<&Path>) -> Option<PathBuf> {
        let path = match project {
            Some(path) => path_for(Some(path))?,
            None => {
                if self.unsaved_path.is_none() {
                    let root = std::env::temp_dir().join("ankhimate-recovery");
                    if let Err(e) = std::fs::create_dir_all(&root) {
                        self.last_error = Some(e.to_string());
                        return None;
                    }
                    loop {
                        let path = root.join(format!(
                            "untitled-{}-{}.ankh.autosave",
                            std::process::id(),
                            NEXT_UNTITLED.fetch_add(1, Ordering::Relaxed)
                        ));
                        if !path.exists() {
                            self.unsaved_path = Some(path);
                            break;
                        }
                    }
                }
                self.unsaved_path.clone().unwrap()
            }
        };
        match ankhimate_formats::save(&path, &state.doc.as_project_ref(), &[]) {
            Ok(()) => {
                self.last_written = Some(path.clone());
                self.saved_document = snapshot(state);
                self.last_error = None;
                Some(path)
            }
            Err(e) => {
                log::warn!("autosave to {} failed: {e}", path.display());
                self.last_error = Some(e.to_string());
                None
            }
        }
    }

    /// Forget the current document's write history.
    ///
    /// Reset the deadline and saved-content snapshot on a new/opened document.
    /// Neither belongs to the next document, even if its revision is the same.
    pub fn reset(&mut self) {
        self.started_at = None;
        self.last_written = None;
        self.unsaved_path = None;
        self.saved_document = None;
        self.last_error = None;
    }

    /// Forget tracking without deleting a recovery file. A Save As or a second
    /// process must not erase a previous session's only copy.
    ///
    #[cfg(test)]
    pub fn discard(&mut self, _project: Option<&Path>) {
        self.reset();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Schedule {
    Idle,
    Wait(Duration),
    Due,
}

/// An autosave worth offering at startup.
#[derive(Debug, Clone)]
pub struct Recovery {
    /// The autosave file.
    pub autosave: PathBuf,
    /// The project it belongs to, if it had one.
    pub project: Option<PathBuf>,
    /// A staged project payload keeps the original project's asset namespace.
    pub asset_origin: Option<PathBuf>,
}

/// Is there an autosave newer than the project it shadows?
///
/// Newer, not merely present: an autosave older than the last real save has
/// already been superseded, and offering it would invite the user to restore
/// stale work over good work.
///
/// A project whose own mtime cannot be read is treated as *not* superseding the
/// autosave — the recovery is offered rather than hidden. Erring the other way
/// silently discards the only copy of a crashed session.
#[cfg(test)]
pub fn check(project: Option<&Path>) -> Option<Recovery> {
    check_after_open(project, true)
}

pub fn check_after_open(project: Option<&Path>, primary_valid: bool) -> Option<Recovery> {
    let primary_time = project
        .filter(|_| primary_valid)
        .and_then(|p| std::fs::metadata(p).ok()?.modified().ok());
    let mut candidates = Vec::new();
    if let Some(path) = path_for(project) {
        candidates.push((path, None));
    }
    if let Some(project) = project {
        candidates.extend(
            ankhimate_formats::container::recoverable_writes(project)
                .into_iter()
                .map(|p| (p, Some(project.to_path_buf()))),
        );
    } else if let Ok(entries) = std::fs::read_dir(std::env::temp_dir().join("ankhimate-recovery")) {
        candidates.extend(
            entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.file_name().is_some_and(|n| {
                        n.to_string_lossy().starts_with("untitled-")
                            && n.to_string_lossy().ends_with(".ankh.autosave")
                    })
                })
                .map(|p| (p, None)),
        );
    }
    candidates
        .into_iter()
        .filter_map(|(path, origin)| {
            let metadata = std::fs::symlink_metadata(&path).ok()?;
            if !metadata.is_file() || metadata.file_type().is_symlink() {
                return None;
            }
            #[cfg(windows)]
            {
                use std::os::windows::fs::MetadataExt;
                if metadata.file_attributes() & 0x400 != 0 {
                    return None;
                }
            }
            let modified = metadata.modified().ok()?;
            if primary_time.is_some_and(|t| t >= modified) {
                return None;
            }
            // A newer corrupt sidecar must not hide a valid interrupted-save
            // payload. Keep an invalid ordinary sidecar as a fallback so Recover
            // can report its error and Ignore still preserves the copy.
            let valid =
                ankhimate_formats::load_with_assets(&path, origin.as_deref().unwrap_or(&path))
                    .is_ok();
            if origin.is_some() && !valid {
                return None;
            }
            Some((
                valid,
                modified,
                Recovery {
                    autosave: path,
                    project: project.map(Path::to_path_buf),
                    asset_origin: origin,
                },
            ))
        })
        .max_by_key(|(valid, time, _)| (*valid, *time))
        .map(|(_, _, recovery)| recovery)
}

fn snapshot(state: &AppState) -> Option<serde_json::Value> {
    serde_json::to_value(ankhimate_formats::convert::to_schema(
        &state.doc.as_project_ref(),
    ))
    .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_autosave_sits_beside_the_project_without_replacing_it() {
        let project = Path::new("/rigs/walk.ankh");
        let auto = path_for(Some(project)).unwrap();

        assert_eq!(auto.file_name().unwrap(), "walk.ankh.autosave");
        assert_eq!(auto.parent(), project.parent());
        assert_ne!(auto, project, "must never target the project itself");
    }

    #[test]
    fn a_save_cannot_delete_a_recovery_copy_not_written_by_this_session() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("other.ankh");
        let path = path_for(Some(&project)).unwrap();
        std::fs::write(&path, b"other session recovery").unwrap();
        Autosave::default().discard(Some(&project));
        assert_eq!(std::fs::read(path).unwrap(), b"other session recovery");
    }

    #[test]
    fn unsaved_documents_get_independent_recovery_paths_and_keep_primary_untouched() {
        let mut state = AppState::default();
        state.doc.meta.name = "work".into();
        let mut a = Autosave::default();
        let mut b = Autosave::default();
        let first = a.write(&state, None).unwrap();
        let second = b.write(&state, None).unwrap();
        assert_ne!(first, second);
        assert_ne!(
            first,
            path_for(None).unwrap(),
            "legacy fixed copy must not be overwritten"
        );
        assert_eq!(ankhimate_formats::load(&first).unwrap().0.name, "work");
        a.reset();
        let third = a.write(&state, None).unwrap();
        assert_ne!(
            first, third,
            "New must not replace the previous untitled document's copy"
        );
        for path in [first, second, third] {
            std::fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn autosave_tracks_persisted_content_even_without_a_revision_increment() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("project.ankh");
        let mut state = AppState::default();
        let mut autosave = Autosave::new(10);
        autosave.write(&state, Some(&project)).unwrap();
        let now = Instant::now();
        assert_eq!(autosave.schedule(now, &state, true), Schedule::Idle);
        state.doc.meta.name = "metadata edit".into();
        assert_eq!(
            autosave.schedule(now, &state, true),
            Schedule::Wait(Duration::from_secs(10))
        );
        assert_eq!(
            autosave.schedule(now + Duration::from_secs(10), &state, true),
            Schedule::Due
        );
        let copy = autosave.write(&state, Some(&project)).unwrap();
        assert_eq!(
            ankhimate_formats::load(&copy).unwrap().0.name,
            "metadata edit"
        );
        assert!(!project.exists());
    }

    #[test]
    fn an_unsaved_document_autosaves_to_the_temp_directory() {
        let auto = path_for(None).unwrap();
        assert_eq!(auto.parent().unwrap(), std::env::temp_dir());
    }

    #[test]
    fn nothing_is_due_before_the_interval_elapses() {
        let mut autosave = Autosave::new(10);
        let now = Instant::now();
        assert_eq!(
            autosave.advance(now),
            Schedule::Wait(Duration::from_secs(10))
        );
        assert_eq!(
            autosave.advance(now + Duration::from_secs(9)),
            Schedule::Wait(Duration::from_secs(1))
        );
        assert_eq!(
            autosave.advance(now + Duration::from_millis(10500)),
            Schedule::Due
        );
    }

    #[test]
    fn an_untouched_document_is_not_rewritten() {
        // Content saved in the recovery copy must not be rewritten periodically.
        let mut autosave = Autosave::new(10);
        let now = Instant::now();
        let mut state = AppState::default();
        autosave.saved_document = snapshot(&state);
        assert_eq!(autosave.schedule(now, &state, true), Schedule::Idle);
        state.doc.meta.name = "edited".into();
        assert!(matches!(
            autosave.schedule(now, &state, true),
            Schedule::Wait(_)
        ));
        assert_eq!(
            autosave.schedule(now + Duration::from_secs(10), &state, true),
            Schedule::Due
        );
    }

    #[test]
    fn an_edit_before_the_first_write_still_saves() {
        // Never written content must be scheduled, even at revision zero.
        let mut autosave = Autosave::new(10);
        let now = Instant::now();
        let state = AppState::default();
        autosave.schedule(now, &state, true);
        assert_eq!(
            autosave.schedule(now + Duration::from_secs(10), &state, true),
            Schedule::Due
        );
    }

    #[test]
    fn a_zero_interval_disables_it() {
        let mut autosave = Autosave::new(0);
        assert_eq!(autosave.advance(Instant::now()), Schedule::Idle);
    }

    #[test]
    fn resetting_forgets_the_previous_documents_saved_content() {
        let mut autosave = Autosave::new(10);
        let now = Instant::now();
        let state = AppState::default();
        autosave.saved_document = snapshot(&state);
        assert_eq!(autosave.schedule(now, &state, true), Schedule::Idle);
        autosave.reset();
        assert_eq!(
            autosave.schedule(now, &state, true),
            Schedule::Wait(Duration::from_secs(10))
        );
        assert_eq!(
            autosave.schedule(now + Duration::from_secs(10), &state, true),
            Schedule::Due
        );
    }

    #[test]
    fn a_missing_autosave_offers_no_recovery() {
        let dir = std::env::temp_dir().join("ankh_autosave_test_missing");
        let _ = std::fs::create_dir_all(&dir);
        let project = dir.join("nothing.ankh");
        assert!(check(Some(&project)).is_none());
    }

    #[test]
    fn an_autosave_older_than_the_project_is_not_offered() {
        // It has already been superseded by a real save; offering it invites the
        // user to restore stale work over good work.
        let dir = std::env::temp_dir().join("ankh_autosave_test_stale");
        std::fs::create_dir_all(&dir).unwrap();
        let project = dir.join("rig.ankh");
        let auto = dir.join("rig.ankh.autosave");

        std::fs::write(&auto, b"old").unwrap();
        // Sleep rather than set mtimes by hand: filesystem timestamp resolution
        // is coarse enough on Windows that two writes in the same instant
        // compare equal, which would make this pass for the wrong reason.
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(&project, b"newer").unwrap();

        assert!(check(Some(&project)).is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_autosave_newer_than_the_project_is_offered() {
        let dir = std::env::temp_dir().join("ankh_autosave_test_fresh");
        std::fs::create_dir_all(&dir).unwrap();
        let project = dir.join("rig.ankh");
        let auto = dir.join("rig.ankh.autosave");

        std::fs::write(&project, b"saved").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(&auto, b"crashed after this").unwrap();

        let recovery = check(Some(&project)).expect("newer autosave is offered");
        assert_eq!(recovery.autosave, auto);
        assert_eq!(recovery.project.as_deref(), Some(project.as_path()));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_autosave_for_a_project_that_no_longer_exists_is_still_offered() {
        // The project file being gone is exactly the case where the autosave is
        // the only copy left. Hiding it here would discard the crashed session.
        let dir = std::env::temp_dir().join("ankh_autosave_test_orphan");
        std::fs::create_dir_all(&dir).unwrap();
        let project = dir.join("vanished.ankh");
        let auto = dir.join("vanished.ankh.autosave");
        std::fs::write(&auto, b"all that is left").unwrap();

        assert!(check(Some(&project)).is_some());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
