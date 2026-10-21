# Format and release compatibility after PathPosition

This is a documentary finding at T-831, not a migration or new format change.
Author schema 1, binary envelope 1, native runtime 2 and app/package 0.2.0-alpha.1 are
independent identifiers. A shared author version alone does not imply identical
supported enum variants across binaries.

| Input/output | T-831 behavior | Historical reader/consumer | Evidence / risk |
|---|---|---|---|
| Old valid project without PathPosition | Existing fields, assets and animations retained; no manufactured track | Existing old profile | Historical migrations/reliability plus T-831 old-project witness |
| New save without PathPosition | Existing profile/field semantics retained | Compatible only within the old supported feature subset; not every arbitrary new project | No T-831 track added implicitly |
| New project with PathPosition | Named exact target/scalar keys/curves persist in all 3 profiles | Frozen T-830 rejects unknown `path_position` before returning an editable document | Forward opening is refused, not lossy downgrade |
| Readable/compact JSON/binary | Both conversion directions and strict references qualified | Existing compact field tags unchanged; new kind carried literally | Schema inventory/Serde case and actual-file old-reader probes |
| Native runtime v2, legacy, generic/custom/template export | Blocking preflight and independent plan guard | No new runtime field or runtime support | Explicit failure before atlas/writes |
| Script/plugin exporter | Refuses before executing script, returns original document | Current plugin context cannot encode the track | Native snapshot regression |
| CPU sequence and spritesheet | Existing Sequence/evaluator, common camera and exact reconstructed RGBA | Raster result does not require runtime Path support | Native pose/static reference at 0/.25/.5/.75; sheet pixels equal |
| Video/GIF | Same Sequence input and existing supervised encoders | Not newly visually qualified for PathPosition | Historical process/encoder gates only; do not claim a new video test |
| Whole-clip numeric retime | T-831 extends T-830 walkers to 14th timeline | Preserves old time/duration/collision semantics | New Path retime/lock/history regressions |

Descending compatibility (new reader, old valid files) and forward compatibility
(old reader, new feature) are different. Actual delivered T-831 files were rejected
unchanged by frozen T-830 in all 3 profiles. The older public reader also has the
closed enum without this tag; its inability follows from that source contract,
not a new public-binary GUI run. Never suggest removing the tag manually, baking
to a static path silently or saving an unsupported project through an older app.

Before a future alpha, decide and document the minimum feature-capable reader,
exchange warnings and candidate/release identifiers. A feature/version/migration
policy review is a release gate; **T-832 does not authorize or implement a bump,
migration, runtime change or repackaging**. Keep a separate complete copy for old
readers and retain originals. Author1 forward refusal is deliberate and prevents
silent loss, but can surprise collaborators using the public alpha.

Existing Save/staging/recovery guarantees and limitations are unchanged: ordinary
failure protection/rollback is tested, multi-file crash atomicity and power-loss
durability are not promised. No new corruption incident is demonstrated by this audit.
