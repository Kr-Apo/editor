# T840B1 — Lifecycle matrix

All entries marked tested refer to headless production-code tests, not visual
GUI qualification. A fresh Physics state means no offsets, velocities, anchors
or old remainder and `paused=false`. Authored constraint parameters remain data.

| Situation | Physics outcome | Document/history/session outcome | Evidence |
|---|---|---|---|
| New AppState | Fresh | Existing default document/session | Tested |
| Successful File New, after Discard | Fresh, even after active/paused simulation | Empty document; clean, history cleared | New/confirmation tests |
| Direct successful document replacement with recycled IDs | Fresh | Incoming native snapshot preserved, history cleared | Converted T840A test + fresh/continuation tests |
| Different parented Physics rig | Fresh; follow-up bone poses exactly match independent fresh instance | Existing author parameters retained | Distinct-rig test |
| Successful Open/reopen, including background completion | Fresh | Existing path/saved-state and session reset rules | Open/save tests |
| Foreign imported rig adoption | Fresh | Existing imported report/adoption behavior | Adoption test |
| Successful recovery | Fresh | Recovered author document remains dirty; recovery source preserved | Recovery test |
| Missing, malformed or unsupported-version load | Retained | Current document preserved | Failure tests |
| Background result stale after newer revision | Retained | Result rejected, active context preserved | Controlled completion test |
| Cancel unsaved confirmation | Retained | Document and pending New retained | Cancel test + existing lifecycle suite |
| Failed Save before New | Retained | Pending destructive operation aborted/suspended under existing policy | Failed-save test |
| Failed or declined recovery | Retained, including pause | Active project and source bytes preserved | Corrupt recovery/Ignore test |
| Successful PSD Replace | Fresh | One undoable import, assets/references preserved | Real tagged PSD test |
| Undo/Redo PSD Replace | Fresh at each exchange | Three cycles restore complete author/asset snapshots | Real tagged PSD test |
| PSD Merge | Retained under existing policy | One undoable merge; complete snapshots restored | Real tagged PSD test |
| PSD preview/import refused in Animate or decode failure | Retained | No replacement or history mutation | PSD refusal tests |
| Native ReplaceDocument, grouped or pre-applied | Fresh | One existing command/group entry; no duplicate application | Native command tests |
| Undo/Redo native whole-rig exchange | Fresh, never restores old velocities | Three cycles preserve authored snapshots and stack depths | Native command tests |
| Refused native replacement with Redo pending | Retained | Redo preserved | Normal/refused-history test |
| Ordinary bone edit, Undo/Redo, empty stacks | Retained | Existing history policy | Normal-edit test |
| Manual Save / simulation-only frames | Retained | Author JSON/saved state unchanged by simulation | Snapshot/dirty/save tests |
| Replacement by rig without Physics | Fresh; old entries gone | No synthetic constraint or history entry | New/PSD/command tests |
| Reset instance A while instance B simulates | B retained | No global/shared simulation | Independent-instance test |
| Rig-session reset during export | Fresh | Existing cancelled worker retained until transaction completes | Existing worker/cancellation regressions in full suite |
| General seek, play/pause, clip switches | Existing policy retained | No new replay/invalidation semantics | Two unchanged T840A characterization tests |
| Native Windows picker gesture or visual Physics trajectory | Not newly qualified | No GUI sessions run | Manual qualification remains separate |
| Incremental topology/bind/constraint edits beyond these exchanges | Existing policy, not globally reset | Broader epoch policy outside B1 | Deferred, not claimed safe replay |

Relevant production boundaries: `app_state.rs` (`replace_document`,
`reset_rig_session`, `dispatch`, `dispatch_applied`, `undo`, `redo`), `fileops.rs`
(`new_document`, `open_path`, `adopt`), `app.rs` (`poll_file_load`,
`recover_project`, confirmation flow) and `ui/psd_import.rs::run_import`.
Only the AppState/command annotations change product behavior in B1.
