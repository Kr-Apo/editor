# T-806 / T-809 — opt-in viewport observation and native bone rigging

Development source only; not in Alpha RC2. T-805.1 chat, animation tools,
providers, validation and confirmation remain in place. No runtime-format change.

## Manual test (maintainer, not automated qualification)

Use a complete **copy** of Jade Sentinel with its `.assets` folder. Start the
development executable and open that copy. Keep the original projects intact.

1. Choose **View → AI Assistant** or its detached window. In Settings select
   NInfer, `http://127.0.0.1:8080/v1`, and the exact model ID. The model/server
   must support images; discovery alone does not test that capability.
2. Click **View…** beside Send (or **Viewport image / preview / consent…** in
   Settings). Click **Capture viewport preview (no transmission)**. The visible
   Viewport must be open; the capture window/settings hide for the captured frame.
3. Inspect the preview. Authorize this specific image for the next explicit
   Send and its bounded tool rounds, close the preview, and ask for a visual
   description. Without consent Send refuses the attached image; **Remove image**
   permits text-only Send. Metadata sharing is a separate choice.
4. Switch the editor to **Setup**. Enable tools, metadata sharing and native
   edits with approval. Ask for a small accessory bone under the actual `chest`
   bone, with an explicitly estimated parent-local setup position. The agent can
   request an image, read exact bone properties, then propose `ankhimate_rig`.
5. Inspect the native card and details. Approve or Refuse. Check the hierarchy
   and position yourself, then Edit → Undo/Redo. Save manually; close/reopen the
   copy and confirm the new bone remains. Refusal must leave the copy unchanged.

These visual/model tests are **pending maintainer execution**. No live model or
GUI qualification is claimed by the automated T-806 tests.

## Capture and consent contract

The existing egui/eframe screenshot command reads the ROOT application render
target; no desktop/window-manager capture API is used. Only the current visible
canvas rectangle is inward-rounded and retained. Full application pixels exist
transiently in egui's readback event; only cropped canvas pixels reach encoding,
preview or HTTP. Project paths and other panels are outside that crop. Editor
overlays or other in-app popups can obscure the canvas: review before sending.

PNG with alpha, longest side <=512/768/1024 for Light/Balanced/Detailed, encoded <=1 MiB; oversized encodings fail
explicitly. Sampling uses deterministic pixel-center nearest downsampling, so
fine details may alias. Encoded image and preview live in session memory only.
The encoded image is not serialized into chat history, project, autosave or
preferences. No automatic capture at startup; a model request opens the consent
flow without capturing. Capture can be cancelled and times out after five seconds.
A hidden viewport requires reopening it and retrying. DPI/layout/crop mismatches
fail closed. Image consent expires after two minutes or a destination/model/
document change. One explicit Send consumes it; tool rounds within that bounded
Send can resend that same image. Later Sends never reuse it. Stop cannot recall
data already sent; error/cancellation never silently retries the image.

T-809 adds distinct native image encodings for all six profiles, preserving the
NInfer/LM Studio compatible transport. See the [provider/capability matrix](native-ai-multimodal.md).
All model capabilities default to unknown. The user must explicitly permit an
untested image attempt or declare model support; declared incompatible models
cannot receive an image. The preview states its resolution, encoded size,
destination and maximum image-carrying HTTP requests (default two). Stateless
continuations replay the image and required opaque reasoning; no persistent
upload or provider session is introduced. Changing quality requires recapture.
HTTP success or model discovery never certifies that pixels were actually used.
Real T-806/T-807/T-808/T-809 visual and model acceptance remains pending.

## Coordinates and native edits

The image carries dimensions, captured revision, playhead, mode, world origin at
the top-left image edge and signed world-units-per-pixel factors. World Y is up,
image Y down; add .5 for a pixel center. It is the captured evaluated pose, not
necessarily Setup. Bones use **parent-local absolute Setup** position/rotation/
scale; animation keys remain offsets/multipliers. Read current native properties
and parent Setup hierarchy instead of guessing exact coordinates from artwork.

`ankhimate_read` section `bone` plus exact `bone` name returns bounded metadata
including length, setup transform, current world affine/origin and lock state.
`ankhimate_rig` accepts only a typed rig-only batch in Setup: bone.create,
bone.setup, bone.rename. Exact fields/bounds and tested examples are in the
[rigging manual](ai/RIGGING.md). No existing-bone reparent/delete, mesh/FFD,
weight paint, attachments, skin/image editing, arbitrary scripts or MCP server.

Preparation uses existing CreateBone, SetBoneTransform and RenameBone commands
on an asset-free scratch. Approval revalidates revision, mode, locks, references,
name uniqueness, numeric bounds and hierarchy. The native transaction groups
inverses into one Undo and broadcasts recreated IDs via existing History on
Redo; assets and animation stores are not replaced. Rename preserves IDs and
name-based Save references. Setup edits preserve keys/binds but may change their
visual result: the card warns explicitly. No promise of preserving an unchanged
appearance after changing setup transforms. Save remains manual.

The [embedded manual](ai/ANKHIMATE-AI.md) remains capped at 6144 UTF-8 bytes;
vision/rigging sections are selected deterministically. Automated tests use
synthetic images and loopback mocks, not inference or Windows input automation.
