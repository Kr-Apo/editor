# Export preflight (M005.2)

The editor prepares the existing export plan after destination selection, then
opens **Export preflight**. Choosing a folder does not publish files. Cancel,
Escape and closing the review discard session state only. The project, assets,
selection, undo stack and dirty state are not edited by preflight/publication.

## Diagnostics and consent

| Level | Meaning | Publication |
| --- | --- | --- |
| Error | Integrity failure, unsafe output or a known unsupported rendered feature | Blocked; no checkbox overrides it |
| Warning | Known loss or a feature with implementation but without target qualification | Requires the unchecked-by-default fidelity/qualification consent |
| Info | Scope, a valid static clip, or publication rules | No extra consent |

Missing/corrupt image bytes, unresolved attachments, missing references, invalid
mesh/weight/deform shapes, non-finite transforms/keys, invalid curves, unsorted
key times and invalid durations are errors. An empty positive-duration clip is
a valid static pose, disclosed as information. A zero-duration data clip warns;
rendered exports require a positive duration. Keyframes outside the clip warn.

PNG and spritesheet checks concern the chosen clip, while data exports inspect
all clips. Geometry/assets and structural references are checked project-wide:
an unused broken skin can therefore block export. This is conservative, not a
claim that every attachment will be visible in the selected clip.

This is an export diagnostic layer, not a new importer, renderer, schema version
or general-purpose formal project validator. Existing import/load reports remain
relevant: information already discarded by an importer cannot be diagnosed here.

## Capabilities by built-in target

“Implemented, unqualified” means code exists and is not being represented as
absent; it does **not** promise complete game/rendering parity. Warnings require
explicit consent. No warning is added to the qualified walker or tagged reference
rigs simply because some other feature has not been tested.

| Feature | Ankhimate runtime v2 | Generic JSON example | Phaser 3 atlas | PNG sequence / Spritesheet |
| --- | --- | --- | --- | --- |
| Independent bone axes/curves, IK/transform reference rigs | Serialized; existing Rust/TS parity and Phaser references retained | Bone animation data only; incomplete rig | No skeletal animation | Existing CPU evaluator; reference pixels retained |
| Attachments, atlas pages, draw order, light tint/alpha | Serialized; tested on reference rigs | Geometry/skins and slot/draw-order tracks omitted | Texture regions only | Baked through existing CPU renderer |
| Dark tint | Static RGBA serialized by the current template; **current TypeScript/Phaser ignore it**, consent warning. Older saved native templates still omit it and warn | Partial-format warning includes omitted slot channels | Texture pixels only; atlas-only warning | CPU code exists; visual qualification warning |
| Non-Normal blending | Serialized, known Phaser mappings exist; fidelity unqualified warning | Partial-format warning | No composition; atlas-only warning | **Error**, existing Normal-only refusal |
| Physics | Serialized and evaluator exists; game-side parity unqualified warning | Omitted; partial-format warning | No simulation | **Error**, existing refusal |
| Multiple skins | Serialized; ordinary alternate skins supported in code; conditional gating warns as unqualified | Omitted; partial-format warning | All source texture regions only | Default skin only; warning when alternate skins exist |
| Valid weighted/deform tracks | Serialized/evaluated; complex target parity unqualified warning | Omitted; partial-format warning | No deformation | Implemented, unqualified warning; malformed shapes error |
| Linked mesh inheriting an animated source deform | Implemented; deformation qualification warning | Omitted | No deformation | **Error**: CPU currently looks up only the current attachment's deform |
| Path constraints / clipping | Code exists; target qualification warning | Incomplete | No rig interpretation | Code exists; qualification warning |
| Game events | Serialized | Exported in per-animation file | None | Images do not carry callbacks; information when events exist |
| Multiple atlas pages | Supported by native data | Region page recorded, consumer-specific | **Error:** shipped JSON template names only its first page | Existing deterministic multiple-page spritesheets |
| Static positive-duration clip | Information, valid | Information, valid | No timed playback | Information, valid; same frame count rules |

The Generic JSON and Phaser atlas examples always disclose their limited scope.
Custom/edited templates and plugin plans require an **unqualified target** warning:
arbitrary template bodies cannot be capability-certified from a preset label.
Built-in detection compares the complete template list, not the display name.
Atlas options remain editable; a disabled atlas with images warns about consumer
loading/UV qualification. M005.3a adds only the optional static `dark_color`
field to the native v2 template; existing fields and the v1 reader are unchanged.
See [runtime-dark-tint.md](runtime-dark-tint.md). Exact older native templates
are recognized separately and retain the `dark-tint-loss` warning. They are not
rewritten on project load. Add the current built-in preset to opt into static
RGBA transmission. Other edited templates retain the custom/unqualified policy.

## Planned files and publication

The review shows every planned relative filename, creations, replacements and
orphans. Lists are virtualized for large sequences. Orphans mean existing files
outside this plan; they are **preserved**, not candidates for automatic deletion.
Discovery is best effort and does not follow directory links/reparse points.
Inventory uses the publisher's normalized portable names, including plugin paths
with backslashes; a planned Windows path must not also be listed as an orphan.
Ordinary existing Windows files are compared by canonical names to account for
case/8.3 aliases; orphan links/reparse points themselves are not followed.

Data export requires a separate unchecked-by-default replacement consent when
existing output names will be replaced. Rendered exports still refuse **every**
planned-name collision; fidelity consent never authorizes these replacements.
Names-only rendered plans come from the same helpers used by the actual sequence
and spritesheet exporters. Sampling, common camera, page layout and metadata
formats are unchanged; the whole RGBA sequence is never accumulated.

The review uses `run::inspect`, which reuses M001's portable path, ancestor,
destination, alias/collision and JSON checks. It creates no directory/file/lock.
It is **not** a permission probe or destination reservation. A busy/stale export
lock is an error; existing recovery backups must be inspected, never overwritten.

Before publication the editor rejects a changed document/revision, repeats path
inspection and inventory, and resets consent if the inventory changed. The data
publisher's `write_reviewed` also compares its own replacement inventory with the
approved names before any writes. This closes the gap between the UI's last
inventory and the writer's initial inspection. Consent covers paths, not a hash
of existing contents: a third party editing an already-approved file is not
detected as a different replacement. The existing writer checks appearance/
disappearance again during publication. Unrelated processes can still race OS
operations; no hostile-concurrency or perfect multi-file atomicity is claimed.

All actual output still goes through the existing staged publisher: exclusive
cooperating-export lock, preparation on the destination filesystem, flushes,
backups, rollback on ordinary errors and retained backups if rollback itself
fails. Windows locks, permission denial and rename failures remain authoritative
publisher errors. A green preflight does not guarantee a future write will work.
See [export-safety.md](export-safety.md) for crash/rollback limitations. No new
power-loss durability, directory-sync or multi-file crash-atomic guarantee is added.

Rendered publication starts the existing background worker and keeps its progress,
cancellation and final success/error status. Launch is not reported as completion.
Data publication remains synchronous as before; planning also remains synchronous.
Large rigs can still take time to bake/decode during review. No benchmark-based
latency promise is made in this milestone.

## Recording an approval

**Copy diagnostics** copies an awaiting-publication report. **Copy last preflight
receipt** copies the last attempted data publication / requested rendered worker
launch, including diagnostic codes, destination, inventory and explicit consent
booleans. Receipts are session-only and can be pasted into a test or integration
report; no extra export file or runtime format field is added. A rendered launch
receipt must be read together with the worker's completion/error status. Receipts
are not a durable audit log and are not saved in `.ankh` files.

Headless consumers can call `preflight::inspect` and `run::inspect` without UI.
`run::write` remains the compatible low-level publisher; it enforces file safety,
not interactive fidelity consent. Callers outside the editor must decide and
record their own warning policy. Rendered orchestration additionally blocks
integrity errors even when called without the UI.
