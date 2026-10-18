---
title: Export safety
description: JSON strings, portable paths, staged publication, rollback and crash limitations.
---

# Export safety — M001b

The public `Plan` and `run::write` API is unchanged. All consumers, including
plugin plans, use this writer for text files, skeletons, animations, atlas
metadata, atlas pages and copied textures. Rendered-image/video exports are a
different pipeline and are outside this milestone.

## JSON and template compatibility

The native runtime, Generic JSON and Phaser atlas built-in templates emit
user-defined string values and object keys through the existing `json` helper,
backed by `serde_json`. Write `{{json name}}`, **without surrounding quotes**.
This preserves Unicode, quotation marks, backslashes and all U+0000–U+001F
control characters. Literal hexadecimal colors and other constant strings do
not contain user input. HTML escaping stays disabled; changing it would corrupt
JSON, Lua and other text formats. Strict mode stays enabled.

Runtime format remains **v2**, context remains **v1**, preset format remains
**v1**. Per-axis channels, curves, timing, offsets and legacy v1 reader semantics
are unchanged. Safe ASCII names keep the same native output bytes.

Saved presets are not automatically rewritten. Copy/select the current built-in
preset or change string interpolations to the `json` helper. Third-party templates
retain ownership of their schema. Before touching disk, the writer parses every
text or binary output with extension `.json` (case insensitive) using `serde_json`.
Malformed JSON, invalid UTF-8 and decoder limit errors are refused with location
details; previously accepted malformed custom/saved templates must be corrected.
This is syntax validation, not a runtime/foreign-schema validator: duplicate JSON
keys or valid JSON with wrong semantic fields are not rejected by this check.
Other extensions retain arbitrary text/binary behavior. A custom JSON format
using another extension must validate its own content.

## Validation before disk changes

An empty output directory is refused. Planning validates the combined text and
binary paths. Writing validates them again because public `Plan` fields can be
constructed directly. Absolute paths
and traversal outside the output root are refused. Slash/backslash aliases and
redundant `.` / internal `..` segments are normalized. Duplicate paths, case
aliases and file/directory prefix conflicts are refused. Existing destination
prefixes are canonicalized to detect filesystem aliases, including Windows 8.3
names where the filesystem exposes them.

Names use a conservative portable Windows subset on every OS. Forbidden:
control characters, `< > : " | ? *`, trailing spaces/dots, leading whitespace,
DOS device names (including extensions and superscript COM/LPT digits), and
components starting `.ankhimate-export`. No alternate data streams are created.
Unicode remains permitted. Case comparison uses Unicode lowercase and is
conservative, not a complete emulation of every filesystem's case/normalization
rules. Exotic filesystem aliases remain a limitation.

Existing output roots, ancestors, destination parents and files are inspected
without following links. Symlinks, Windows junctions and other reparse points are
refused. Existing destinations must be ordinary writable files; directories and
read-only files are refused. Orphan surveys do not descend into reparse points.
An otherwise valid name inside JSON need not be a valid filename: Generic JSON's
per-animation filenames or loose-image names with forbidden characters require
a safer output-path template/name. The native single-skeleton export can retain
these names inside JSON.

## Preparation and publication

1. Acquire `.ankhimate-export.lock` with exclusive `create_new`. An existing lock
   prevents another cooperating exporter from proceeding; no stale lock is
   removed automatically.
2. Create a unique `.ankhimate-export-<pid>-<counter>` directory under the output
   root. Write `manifest.json`, mapping each destination to `new-N` and `old-N`,
   then write all staged bytes with `write_all` and `sync_all`. Close all handles.
   Original destination bytes remain untouched during preparation.
3. Create missing destination parents and recheck destinations before replacing
   them. Rename existing files to `old-N`; rename `new-N` to their destination.
   This does not depend on renaming over an existing file, which Windows refuses.
4. On success, remove only transaction-owned temporary files/backups and the
   lock. A cleanup failure returns an explicit error saying all outputs were
   published; it does not claim that the originals were restored.

One shared lock covers cooperating exports to the same root. The staging area
adds disk usage for the full new export until publication. Backups are renames,
not in-place truncation or copies. A destination on a separate mounted filesystem
can cause rename failure; it is handled as a publication failure with rollback.
Successful replacement creates a new file identity with staging metadata; it
does not promise to preserve original timestamps, file identity or Windows ACLs.
Rollback by rename restores the original file rather than recreating its bytes.

## Failure and recovery

Preparation failure leaves previous output files unchanged. Publication failure
rolls back entries in reverse order: remove newly published files and restore
old backups. Only directories created by this invocation are removed, and only
if empty. Orphans and user-owned lock files are never deleted. No recursive
directory sweep is used for transaction cleanup.

If rollback operations succeed, previous bytes are restored and new files,
temporary files and empty newly created parents are removed. If a rollback
operation fails, the error reports **ROLLBACK INCOMPLETE**, the failed operations
and the staging directory. Remaining backups and its manifest are retained.
Restoration refuses to replace an occupied destination. Some outputs may then
be new, old or temporarily missing; there is no silent success.

For manual recovery, stop competing writers, inspect `manifest.json`, and copy
the retained `old-N` files to a separate safe location before choosing which
version to restore. An absent backup may already have been restored, may never
have existed, or may not yet have been moved. The manifest is an inventory, not
a crash journal; do not infer completed publication from it. Inspect destinations
and staged files. Remove a stale lock only after confirming no exporter is active.
Automatic crash recovery is not implemented in M001b.

## Guarantees and limits

M004.1 PNG sequences use the same transaction with generated binary payloads:
only one encoded frame is held at a time while all files are prepared before
publication. This entry point refuses every existing target name and supports
cooperative cancellation. Cancellation during publication uses ordinary rollback;
rollback is never cancelled. The existing preset/plugin replacement behavior is
unchanged. See [PNG sequences](png-sequences.md) for timing and rendering limits.

This is **not a perfectly atomic multi-file export**. Each rename affects one
directory entry; readers can see a mixed set, or a file absent between backup
and publication. Process termination, OS crash or power loss can leave a lock,
staging files, backups and an incomplete export. `sync_all` flushes staged files
but directory rename durability is not guaranteed and filesystem/controller
behavior still matters. There is no commit pointer or durable recovery journal.

The lock coordinates this exporter, not arbitrary applications. A hostile or
uncooperative writer can change bytes, permissions or directory topology between
checks and operations. No handle-relative filesystem sandbox or cross-process
snapshot isolation is claimed. OS path-length limits, disk space and permissions
can still cause errors; network/virtual filesystems may have weaker semantics.

The tested Windows behavior includes real rename sharing violations, read-only
destinations, junction confinement and complete rollback. Fault injection covers
all staged-file writes, backup moves, publications, failed restores/removals and
cleanup errors. Unix symlink coverage is compiled conditionally and has not been
executed on the Windows validation host. See the
[M001b validation record](milestones/M001b-validation.md) for commands and results.
