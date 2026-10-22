# Opening, saving and recovering projects

Work on a copy when testing an alpha. Keep `Character.ankh` and its complete sibling
`Character.assets` folder together. The readable/compact JSON profiles use the same
asset folder convention. Choose an ordinary writable Windows folder; linked/reparse,
reserved/device, read-only and invalid paths are refused for writes. An absent parent
folder is an error, not an automatically created project location.

## Save and missing images

Save writes the active path; Save As changes it only after complete success. New, Open
and Close require Save / Discard / Cancel for persisted changes. A failed Save keeps the
current document and aborts the destructive action. Cancel keeps selection and history.
Undo/Redo history is session-only, not stored in `.ankh`.

Images are external, not embedded. Moving the project without its assets can open with
unresolved images. **Do not continue a long session with missing images:** Save and autosave
are refused until all indexed images have bytes. Restore the folder, or use Assets → Relink
on the missing image, choose the replacement, and retry Save As to a new location. Relink
is undoable. Explicitly deleting an unwanted asset is a separate deliberate edit. Existing
image names and original project/asset files are not silently rewritten by a failed Save.
The format is not a comprehensive repair tool: broken structural references are reported
and some unresolved entities are omitted by the established converter. Keep the original
and inspect diagnostics; do not overwrite your only damaged project while repairing it.

Non-finite schema numbers and unsafe publication paths also cause Save refusal. If cleanup
fails after publication, the error says the new project was published but recovery files
remain: inspect the stated folder and keep copies before retrying. A successful Save is
not permanent version history. Keep independent backups.

## Autosave copies

Settings controls the interval (zero disables). A dirty named project writes
`Character.ankh.autosave` beside the primary, with its own sibling asset namespace
`Character.ankh.autosave.assets`. Untitled copies live in the system temporary directory
under `ankhimate-recovery/untitled-PID-counter.ankh.autosave`, with their associated assets.
The monotonic timer works at rest; autosave never marks the primary as manually saved.
It writes only changed persisted content and retries failures after an interval.
Temporary-directory cleanup by Windows or external tools can remove untitled copies.

## Recover safely

Open the original path. Even a missing/corrupt primary can trigger an offer. The prompt
shows the actual recovery filename. Recover reads that copy and marks it dirty; it does
not write over the primary. Save As to a **new** folder/name if you want to keep both
versions. Keep the primary, sidecar and asset folders until you have reopened the result.
T837: Ignore/Escape records an immutable receipt for the exact canonical identity,
SHA-256 content and modification state, preserving every recovery/asset file.
An unchanged copy stays suppressed after restart; new identities or changed states
remain eligible. File > Review Recovery Copies… explicitly includes ignored copies
for the active project and retained untitled documents, behind Save/Discard/Cancel.
For another named project, open its path first. Multiple offers advance on Ignore.
If persistence fails, the offer remains and the error says Ignore was not completed.
Later (this session) is then an explicit non-durable escape; no data is removed.

A valid primary suppresses copies with equal/older modification times. Otherwise, valid
copies take precedence over corrupt sidecars; the newest valid one is offered. A corrupt
ordinary copy can remain a fallback to expose its error. Source timestamps can be changed
by backup/sync tools, so this is a recovery heuristic, not a version-control history.
A failed Recover preserves the current document, selection and history. A pending offer
cannot replace a document edited since discovery. No global disk browser or purge is provided. A targeted explicit review is available
in File; it retains the established stale-copy filter. Ignore receipts contain no
project payload or path in clear text. They live in the application config directory
under recovery-decisions-v1 (temp fallback only if config discovery fails).
Exclusive receipt creation avoids multi-instance lost updates. Incomplete/corrupt
receipts never suppress a copy and are never overwritten. File flush is requested;
directory durability, adversarial concurrent path changes and power-loss recovery
are not guaranteed. A changed/deleted copy at action time fails without replacing
the current document. Windows Temp cleanup remains outside the application's control.

Interrupted saves can leave a sibling `.ankh-save-PID-counter` directory containing
`manifest.json`, `new` and/or `previous`. Keep the entire directory and original asset
folder. Its manifest identifies the intended primary path; validated staged payloads use
that original asset namespace. Do not overwrite backups or manually delete stale locks
until no writer is active and you have copied the recoverable files. Do not deliberately
crash the editor to test a real unsaved project.

## Versions and crashes

The current author schema/envelope are version 1. Authentic v1 projects are tested across
all three profiles. Future versions, unsupported envelope flags, unknown author members
and unsupported blend/constraint/sequence values are refused rather than degraded into a
rewritable document. Opaque export presets remain opaque and preserved. The historical
ZIP prototype and unauthenticated v0 migration are not supported. Never change a version
label manually to bypass a refusal; use a compatible editor or a separately reviewed
converter and preserve the original.

There is no dedicated panic-time dump or persistent crash-log service. Completed periodic
copies and recoverable write stages are the recovery mechanism. Recent unsaved edits may
be lost on a kill/panic. There is no proven power-loss durability or multi-file atomicity.
No update checker/feedback/telemetry was added. See [format specification](format-spec.md)
and [technical save safety](project-save-safety.md) for exact boundaries.
