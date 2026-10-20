# Document diagnostics (T-818 local candidate)

**LOCAL_CANDIDATE · PENDING_MANUAL**. Internal version 0.2.0-alpha.1 is unchanged.

The bottom **Diagnostics** badge gives exact counts of Error, Warning and Info.
Click it to open or close **Document diagnostics**. Closing the window retains
the report in this session. No window opens automatically during an edit.
The list is sorted by severity, code, location and message, with repeated
identical findings removed. Text labels accompany colors.

Errors describe author-document integrity. Warnings describe permitted but
potentially problematic production data. Info includes unused images and empty
skins/static clips; it does not mean corruption. No rule repairs, normalizes,
deletes, publishes, adds keys or changes the Save policy.

## Rules and scope

The existing export preflight supplies reference, image decoding/dimensions,
mesh topology, weight validity, numeric/curve/time and deform shape checks.
A document-only mode suppresses consumer-fidelity messages and allows intentionally
rigid vertices with empty influence groups. Existing export targets retain their
original policy. Native checks add weight sums (tolerance 0.001), derived bind
fallbacks, evaluation-order integrity, skin fallback availability, mesh edges,
inline FFD, finite path/clipping points, IK structure, duplicate track addresses,
duplicate draw order and unused images.

Positive-duration static clips are valid (Info). Zero-duration data clips warn.
Out-of-duration keys are retained. Repeated animated draw offsets warn because
evaluation uses the first entry; equal destination positions are not corruption.
Missing derived binds warn about native setup fallback; inspection never rebinds.
Optional image source paths are not probed: embedded library bytes are authoritative.
Alternate skins may intentionally inherit default entries. Geometry and FFD are
not certified for an arbitrary runtime by this author-document report.

## Refresh and navigation

Opening/replacing a project resets the derived cache. The first scan is immediate.
Native edits (including approved AI edits), Undo and Redo move the existing
revision. Rapid revisions coalesce for **200 ms** after the last observed change.
Metadata name/FPS are also compared cheaply to cover historic direct assignments
without changing their undo behavior. Camera, playback and selection do not cause
another scan. Only a pending refresh requests a timed repaint.

While refreshing, previous findings are explicitly stale and navigation is disabled.
A click rechecks session document identity, revision, metadata and an exact native
reference/asset snapshot. Stale clicks refuse selection and schedule a fresh scan.
This guards deletion, Undo, reused IDs, project replacement and out-of-band reference
changes. Arbitrary unsupported direct mutation of other fields is not polled each
frame; supported edits must use native commands. The click guard still rejects it.

Selectable findings open Properties, Slot Editor, Skins, Animations or Dopesheet.
An existing clip is selected before entering Animate: navigation cannot create a
clip. Supported bone/slot key locations use the shared revision-scoped key selection.
FFD/constraint/draw-order indices are not invented as editable KeyRefs. Ambiguous
schema names, missing targets and library images remain report-only when exact
selection is unavailable. Pose previews must be committed/cancelled first.
Geometry drafts are invalidated through the existing selection/geometry mechanisms.
The camera stays unchanged; universal offender framing is not implemented.

Reports, snapshots and window state are session-only, absent from `.ankh` and
exports. Inspection uses in-memory bytes, with no source filesystem writes.
Large scans are synchronous and decode images once per scanned revision; no
arbitrary-resource or worst-case latency guarantee is made. The 200 ms delay
does not promise 200 ms scan completion. Historical T-702 has no numeric budget.

## Export remains separate

A clean global report is **not** permission to export. Export preflight still
checks the selected target, fidelity consent, destination inventory/replacements,
revision and file safety; the staged publisher retains locks, backups and rollback.
Errors cannot be bypassed; target warnings still require explicit consent.
No new Save blocker or automatic correction is introduced.
See [Export preflight](export-preflight.md) and [Save safety](project-save-safety.md).

## Deferred manual checklist

Use disposable copies and the T-818 executable, with the retained T-813–T-816
checklists. Record actual SHA, Windows build, GPU/driver, DPI/theme and results.
All steps are **PENDING**, not PASS from headless tests.

1. Open a valid copied project.
2. Locate the bottom Diagnostics badge and its counts.
3. Open and close the list; verify no dirty-state change.
4. Shorten a copied clip so an existing key lies beyond its duration (reversible).
5. Check Warning / `key-outside-clip`, clip, time and retained key.
6. Click the finding and check selection/playhead without a new Undo entry.
7. Restore the duration or Undo the change.
8. Wait for the warning to disappear; verify no automatic key deletion.
9. Change clip, make an edit, then Undo/Redo.
10. Verify refresh/stale indication and updated counts without a camera-induced scan.
11. Keep the list open, delete an offender then Undo or open another copy; stale navigation must not select a reused object.
12. Open Export preflight; errors, warnings, consent, Cancel and destination checks must retain their behavior.
13. Save As to a new location, reopen, and check authored data/assets plus recomputed diagnostics.
14. Check no implicit correction, normalization, history entry or changed export; run earlier cumulative safety/animation/geometry/rigging checklists.
