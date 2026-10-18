# Project save safety and recovery — M001c

## Document lifecycle

New, Open (including recent files), Import and root-window Close use the same
confirmation controller in `editor/src/app.rs`. The custom title button, Quit
menu and native root-window close event reach that controller. A clean document
continues directly; persisted changes require **Save / Discard / Cancel**.

- Save continues only after `FileOutcome::Saved`. An error or cancelled native
  save picker leaves the current document and confirmation intact.
- Discard permits the requested operation. Open/import still keep the old
  document until the background reader successfully completes.
- Cancel dismisses the confirmation and suspends its request. It preserves the
  document, path, selection, undo/redo history and existing reader. An ordinary
  later Save does not execute that suspended request. A new explicit lifecycle
  request supersedes it. Repeated requests cannot replace an active confirmation.

The save point is a snapshot of persisted Ankh v1 content, not the monotonic
revision/cache counter. Undo followed by redo to the saved content becomes clean;
selection and other session-only changes do not become document edits. Imported
and recovered documents require a real Save. Save As changes the active path and
save point only on complete success.

Background readers own no application state. Their result can be adopted only
by the current receiver and only if both revision and persisted content still
match their starting document. A newer reader replaces the receiver; New, Close,
recovery and successful Save As invalidate obsolete readers. Confirmation and
recovery prompts suspend handoff. Worker threads may finish reading after their
receiver is dropped, but cannot replace the document.

## Writing a project

`fileops::write_to` calls `formats::save`; `container::atomic_write` delegates to
the private `formats::safe_write` module. The historic function name does **not**
imply a fully atomic or durable project transaction.

1. Serialize the complete project in memory. Validate the primary path and its
   existing parent before preparing assets. Refuse directory, linked/reparse and
   read-only write targets. Validate portable Windows components (device names,
   trailing dots/spaces, control characters, invalid characters and ADS colon),
   and refuse Windows device namespaces. Normal drive/extended-drive and UNC
   prefixes are permitted; actual share behavior still depends on the server.
2. Prepare external assets first. Existing files must contain the exact expected
   bytes; identical read-only assets can be reused. Different existing bytes are
   an error and are preserved. New assets use the same safe writer. Asset naming,
   project version and the external asset namespace are unchanged.
3. Exclusively create a sibling `.filename.ankh-write-lock`, then an exclusive
   `.ankh-save-PID-counter` staging directory. Old `.tmp`, `.bak-PID` files and
   other stages are never reused or overwritten.
4. Write and `sync_all` a recovery `manifest.json` and the complete `new` payload;
   close both file handles before renaming. The manifest identifies the original
   absolute project path and recovery-manifest version 1; this is **not** a new
   project/export format version.
5. Move the old destination to `previous` inside that owned stage, then publish
   `new` under the destination name. On an ordinary publication error, restore
   `previous` if the destination is still absent. If restoration fails or a new
   destination appeared, retain `previous`, `new` and the manifest and report
   **ROLLBACK INCOMPLETE** with the recovery directory. Do not overwrite the
   newly occupied destination during that check.
6. After success remove only known owned staging files and the empty stage and
   release the lock. Cleanup failure is an error explicitly stating that the
   new project was published; the UI remains dirty and a pending destructive
   action stays aborted. There is no recursive deletion of recovery directories.

## Autosave and recovery

Named projects keep the existing `<project filename>.autosave` location, with
their own external asset namespace. Untitled documents now write under
`temp/ankhimate-recovery/untitled-PID-counter.ankh.autosave`, independently for
each process/document. The old fixed `temp/untitled.ankh.autosave` is still
discoverable but is never used for new writes. Save, Save As, New and Ignore do
not delete older recovery copies. Dirty persisted content and the interval gate
autosave; an unanswered recovery offer blocks it. Failures appear in the status
line and logs, without claiming that the primary document was saved.

M001c.1 measures pending intervals with `std::time::Instant`, independently of
the frame/animation clock. The final update after edits or file actions requests
egui's delayed repaint for the remaining interval. An idle reactive window thus
wakes for the attempt without playback or input. Clean content, disabled
autosave, pending recovery and content identical to the last successful copy
need no autosave wakeup. A later edit starts a new interval; subsequent edits
during a pending interval do not postpone it. Manual Save resets the timer.
Failure retains the existing document and schedules one retry after another
whole interval, with no immediate repaint loop or catch-up burst. Changing the
configured delay applies to the elapsed pending interval; disabling/re-enabling
starts fresh. Hand-edited intervals exceeding one day use bounded daily wakeups
until due, to avoid platform wait overflow without changing the interval.

This is a scheduling guarantee while the event loop runs, not a real-time or
durability guarantee: blocked UI, OS scheduling, suspension and synchronous
serialization can delay completion. An old backend wakeup after Save may still
occur once, but cannot write clean content or rearm the autosave timer. The writer,
backup/rollback and recovery guarantees below are unchanged.

Startup and native Open look for recovery beside the **requested** project, even
when it is missing or corrupt. A valid primary supersedes copies at or below its
mtime; a corrupt primary does not suppress an older recovery. Untitled discovery
also scans the new recovery directory. Interrupted-write stages must match the
manifest destination and contain a decodable payload; truncated candidates are
skipped. Staged payloads load assets relative to the original project, not their
temporary filename. Discovery refuses linked/reparse candidate files/stages.

The newest eligible copy is offered through the existing modal shell, including
on the startup page. Recover first loads it successfully, then adopts it and
marks it dirty; it never rewrites the primary file. Missing assets remain visible
as unresolved references. A stale offer cannot replace changed content. Failure
keeps the existing document/selection/history and offers retry or Ignore. Ignore,
Escape and closing the prompt decline recovery without deleting the copy.

## Actual guarantees and limitations

Tested ordinary preparation/flush/backup/publication errors keep or restore the
previous primary bytes. Real Windows sharing violations and read-only targets
fail without replacing valid files. Failed restoration retains a discoverable
backup. These are error-handling guarantees under a stable filesystem and
cooperating writers, not proof of power-loss durability.

- There is a rename interval where the primary path is absent. Process death in
  that interval can leave the old valid payload in `previous`; discovery offers
  it, provided the stage/manifest survives. No directories or rename metadata are
  fsynced, and storage caches, network filesystems and power loss can lose even
  flushed data/metadata. No full atomicity or durability is claimed.
- External assets and project data are not one multi-file transaction. A failed
  save may leave new unreferenced content-addressed assets. Existing assets and
  recovery files are not garbage-collected. A successful save removes its owned
  old primary backup after publishing; it is not permanent version history.
- Locks serialize cooperating publication attempts for one filename. They are
  not an OS lock on the whole project/asset set; two independent editor sessions
  can still save different revisions in succession. External noncooperating
  writers can race checks and renames; this is not a hostile-filesystem boundary.
  Symlinks, junctions and cloud placeholder reparse paths are intentionally
  refused for writing. The original ACL/file metadata is not cloned onto the
  newly staged file; inherited directory permissions apply.
- An interrupted process can leave a stale lock. It is not automatically deleted:
  inspect the primary/stage and verify no writer is active before manually
  removing that exact lock. Recover can read retained stages without removing it.
  Legacy `.bak-PID`/`.tmp` backups are preserved but require manual examination.
- Recovery presents one newest candidate; other retained copies need manual
  opening/examination. Ignore is per offer/session, not a persisted dismissal.
  Named autosaves still share one location across sessions. Timestamps can be
  coarse or changed externally. There is no recovery browser or automatic purge.
- Save/autosave and persisted-content snapshots are synchronous. Large projects
  may pause the UI; content comparison/hashing can also cost time per frame.
  Background readers are invalidated by dropping receivers, not forcibly stopped.

Project schema Ankh v1, runtime v2, runtime v1 reader compatibility and all export
presets remain unchanged. Portable path restrictions are the deliberate behavior
change: invalid/linked paths must be replaced with an ordinary valid location.

Automated tests use production controller/writer seams, CPU egui input and real
Windows file handles. M001c.1 also validates idle autosave in the real reactive
Windows desktop at 30 seconds, later edits, manual Save and playback; see its
milestone report. No real power-cut test, network-share fault test or ACL-denial
environment is claimed; permission-denied preparation and rename/restore/cleanup
faults are injected.
