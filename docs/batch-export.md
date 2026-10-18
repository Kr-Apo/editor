# Batch animation export — T-601.3

**Export → Image sequence / Spritesheet → Batch export** enables per-animation
checkboxes. Each row shows its frame count at the chosen FPS. At least one clip
is required; the panel reports the sum of selected frames (and sheet pages).
Invalid timing/dimensions/page sizes are diagnosed. A positive-duration clip
without keys remains a valid static sequence under the existing rules.

Choose format, FPS, canvas dimensions, bounds, trim and padding once: they apply
to all checked clips. JPEG keeps the explicit opaque RGB background and quality
95; WebP stays lossless RGBA. Sheets remain PNG + **Ankhimate** JSON, not Phaser
native JSON. Cameras and reconstruction bounds are prepared separately per clip.
No evaluator, renderer, individual metadata or runtime v1/v2 change is involved.

Click **Export batch…**, select an **existing parent directory**, and use the
**New batch folder** name entered in the panel (default `animation-batch`). The
final directory must not exist, even if empty. Choose a different folder name
for each run. The preflight lists all files and diagnoses the complete selection
before launch. Cropped/padded modes retain the explicit reconstruction warning
and consent; the current Phaser sheet example still refuses cropped sheets.
[Bounds/trim contract](rendered-framing.md) and [individual image formats](png-sequences.md).

Single-clip **individual mode remains the default** with its original destination
and filenames. Selecting one clip in batch mode instead creates the batch
structure below; its child files are identical to an individual export with the
same settings. The batch toggle, checked IDs and folder name are session state,
not saved author data. A launched job captures its document, selected clips and
settings. Later UI changes do not change the reviewed job.

## Folder naming and manifest

Order is the document's animation iteration/serialization order, independent of
checkbox click order. Each selected clip uses its one-based source ordinal plus
a readable suffix, e.g. `0001-walk`, `0002-leg_ik`. Unicode and interior spaces are
retained. Control characters and `/\<>:"|?*` become `_`; leading/trailing spaces
and dots are removed; an empty suffix becomes `animation`. The entire component
is limited to 80 UTF-16 units without splitting a Unicode scalar value.

The ordinal prefix separates identical names, case aliases, forbidden-character
collisions, long-name truncation and canonical Unicode lookalikes. No Unicode
normalization dependency is introduced. Prefixed device-like names are ordinary
Windows names, e.g. `0001-CON`; the unprefixed **batch destination folder** still
refuses Windows reserved names. No animation name is used as a raw path.
Changing the document's clip order/content can change the ordinals; each export
records the actual mapping. All relative paths also pass M001 portable path and
collision validation, and existing parent reparse points/links are refused.

`batch.json` is an additional manifest; individual `sequence.json` /
`spritesheet.json` files are unmodified. Its contract:

| Field | Meaning |
|---|---|
| `batch_version` | Integer 1 |
| `kind` | `image_sequence` or `spritesheet_png` |
| `total_frames` | Sum of all selected sample counts |
| `clips[].source_index` | Zero-based ordinal in the source document, not a serialized SlotMap ID |
| `clips[].animation` | Original author name, including Unicode/escaping and duplicate names |
| `clips[].folder`, `metadata` | Safe relative directory and relative individual sidecar path |
| `clips[].fps`, `duration`, `frame_count` | Sampling FPS, authored seconds and actual end-exclusive sample count |

Selection uses in-memory animation IDs. A private one-clip document snapshot
resolves homonyms unambiguously through the existing name-based evaluator API;
it is never written back to the author project. This also prevents unselected
namesakes from changing the selected clip's duration/keys. Assets and history in
the original document remain intact.

```text
animation-batch/
  batch.json
  0001-walk/
    sequence.json
    frame_000000.png … frame_000059.png
  0002-leg_ik/
    sequence.json
    frame_000000.png … frame_000059.png
```

For sheets, each child contains `spritesheet.json` and its own `sheet_*.png`
pages. No atlas pages, camera, trim rectangles or frame numbering are shared
between clips. The source-sized fixed-cell packing of T-601.2 remains unchanged.

## Preparation, publication and failures

The new directory publisher is a small wrapper around the existing individual
exports; it does not replace M001's file publisher. It first validates all planned
relative paths and the absent final destination, then creates a unique, owned
`.ankhimate-batch-<pid>-<counter>` sibling on the same filesystem. Each clip runs
sequentially through its existing staged export into a private child. After all
clips, the manifest is written with the existing writer. A full inventory must
match the planned set of nonempty ordinary files before publication.

A last destination/ancestor check is followed by a **no-replace directory move**:
Windows uses `MoveFileExW` with flags 0, rather than `std::fs::rename`, which the
regression test found could replace an empty directory on this machine.
[Windows API contract](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw).
Linux uses `renameat2(RENAME_NOREPLACE)` and requires its libc/kernel/filesystem
support; failure never falls back to a replacing rename.
[Linux API contract](https://man7.org/linux/man-pages/man2/rename.2.html).
Other platforms refuse batch publication before staging. Windows is tested here;
the Linux branch is not qualified by this Windows run. The existing individual
exports remain available and unchanged on other platforms.

A destination appearing during rendering **or after the last advisory check** is
not replaced. A Windows sharing violation/rename error is reported. Ordinary
preparation errors, a failure on any clip and cooperative cancellation publish
nothing at the final location. Cleanup visits only the exclusively created
sibling tree, never a wildcard/prefix sweep of older temporaries or output folders.
If a child transaction/backup, unexpected link, lock or cleanup failure remains,
the temporary is preserved and its path reported for inspection. Existing backups
are never overwritten or deleted to make a retry work. Do not treat a leftover
`.ankhimate-batch-*` tree as a completed export; it has no automatic crash recovery.

Once the final move succeeds, the complete batch is reported; cancellation arriving
after that boundary does not undo success. In-progress frame render/encoding and
the final OS move cannot be interrupted halfway. This is not a durability claim:
there is no flushed parent-directory commit journal, guarantee against power/OS
failure, or protection against unrelated writers replacing ancestors or changing
files after publication. Network filesystems and hostile concurrent mutation
are not newly qualified. Do not assume absolute atomicity from the directory move.

## Current limits and evidence

Default skin, Normal blending and refusal of physics are preserved. No new
advanced-rendering fidelity is claimed. Memory includes the reviewed document,
per-clip private snapshot, plan/manifest entries O(total files), current sequence
buffers/textures and codec scratch; clips render sequentially, not all RGBA frames
at once. Textures are prepared per clip by the existing API. Very large selections,
path-length limits, throughput and cancellation latency are not performance
qualified. Original T-601 physics, general backgrounds, advanced rendering and
viewport/budget acceptance remain open: **T-601 stays 🟡**.

T-601.3 headless tests cover egui checkbox clicks/counts, single/two/three clips,
fractional durations, duplicate names and Windows/Unicode/truncation aliases,
PNG/JPEG/WebP and multipage sheets, per-clip reconstruction, consent, stale reviews,
history/Undo/Redo preservation, errors on the second/third clip, cancellation,
late destination races, Windows locks and retained recovery backups. Historical
207-file comparisons keep individual PNG/sheet bytes unchanged. No Computer Use,
video, screenshots, remote Windows demonstration or WebGL battery is added.
