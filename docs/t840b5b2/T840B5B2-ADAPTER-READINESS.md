# T-840B5B2 — Adapter readiness map

No graphical adapter is integrated or requalified in this mission.

| Surface | Current behavior and future connection | Remaining gate |
|---|---|---|
| core/src/player.ts RigPlayer.update/pose | Accumulates number dt, crossfades/events, private Legacy evaluation state; pose(dt) can step Legacy. Do not call it to redraw ReplayV1. Host update should eventually request bridge time and render retained ConsumerFrame. | No automatic profile switch or float bridge. |
| core/src/render/playback.ts RigPlayback.update/display/setSkin/destroy | Owns Legacy player, rate, event listeners and lastPose; update immediately invokes renderer; skin changes can sample player. | Separate opt-in owner path, no second simultaneous player; bridge publishes no events, fades or slot pose. |
| pixi/src/playback.ts, pixi/src/rig.ts | PixiPlayback delegates RigPlayback; constructor/controller, setTime, setPose, applyPose, appearance and destruction own geometry/textures/masks. | ConsumerFrame is not complete RigPose: need qualified static slot/draw/attachment composition and skin handoff from committed tick, no implicit sampling. |
| pixi/src/validate.ts | parse/validateSource requires native v2 and zero constraints (including Physics/Path), atlas and qualified attachments. | Keep refusal. Dedicated opt-in numerical plus WebGL2/capture/pixel campaign before Physics display. |
| phaser/src/phaser.ts | update uses milliseconds/1000, RigPlayback delegates Legacy. setPose applies then detaches; apply resolves appearance and updates vertices/UV/page/draw/masks. Constraints warn legacy/outside Pixi contract. | No update wrapper silently substitutes ReplayV1. Need matching tick skin + bone-only bridge to complete pose, real WebGL measurements and resource checks. |
| editor/src/app_state.rs tick_playback/set_playhead/refresh_pose | f32 session playhead and PhysicsState; preview overrides, Setup/Animate and physics_dt are authoring policies. | No AppState edit. Future integration must preserve edits, document-context resets, scrubbing and setup simulation separately. |
| render/export | Existing CPU consumers refuse Physics and unsupported modes; PathPosition export refusal retained. | A headless bone bridge proves no image/export fidelity; no refusal lifted. |

Immutable calendar memberships use the native default special case; appearance
fallback remains independent. Adapter must select skin using the committed frame,
not the host target or transport preview position. Stage full appearance/resources
before applying a changed skin; transactional GPU failure semantics need review.
Current disposal paths release player/listeners/meshes/masks; future bridge disposal
must accompany them without modifying shared atlas ownership. Multiple owners need
independent clocks/records despite shared immutable textures and author definitions.

Capabilities/versions: no new persisted profile or native field now. Future explicit
experimental adapter capability should declare native v2, bone-only single clip,
Physics-only, canonical 120 Hz, frozen skin calendar and no events/layers/fades/FFD.
Mixed IK/Transform/Path stays refused even when inactive. Do not claim broad hardware,
WebGL1/WebGPU/mobile or clipping/mesh fidelity from these headless snapshots.

Before GPU qualification: B6 private numerical rollback, exact game-time policy,
bounded pumps and visible lag, coherent skin appearance application, static slot
contract, resource lifecycle, failed renderer behavior, CPU/numerical oracle and
independent WebGL pixels on representative rigs. No implementation of those stages.
