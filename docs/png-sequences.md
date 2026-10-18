# Export image sequences (M004.1 / T-601.1)

Open **Export → Image sequence**, choose PNG, JPEG or WebP lossless, an existing
animation, FPS and dimensions, then **Export … sequence…** and a new destination
folder. For JPEG, select the explicit opaque RGB background (white by default). The operation uses a
snapshot of the document at launch; subsequent edits do not change the export.
It does not change the playhead, selection, history, saved project or dirty state.
The existing **Data / atlas** preset pipeline remains available beside it.

## Formats and metadata

| Choice | Frame names | Pixels and background |
|---|---|---|
| PNG (default) | `frame_000000.png`, … | RGBA8, straight alpha, transparent. Existing encoder and output unchanged. |
| JPEG | `frame_000000.jpg`, … | RGB8, no alpha. Straight RGBA is composited onto the chosen opaque RGB background before lossy encoding at fixed quality 95. |
| WebP lossless | `frame_000000.webp`, … | RGBA8, straight alpha, transparent. Lossless decoded pixels, including RGB beneath zero alpha. |

JPEG compositing uses `(source * alpha + background * (255 - alpha) + 127) / 255`
with integer division, per encoded 8-bit colour channel; this is not linear-light
compositing. Fully transparent pixels become the chosen background, never an
implicit black fill. Compression can change RGB values, including background
pixels; use PNG/WebP when exact pixels or alpha are needed. JPEG quality is fixed,
not a new UI setting. All codecs come from the already enabled `image` 0.25.10
features; there is no additional dependency or external executable.

Every sequence includes `sequence.json`. Import frames using its FPS. This is
Ankhimate rendered-output metadata, not runtime v1/v2 or Phaser-native atlas JSON.
The schema and field types remain unchanged: the historical key
**`png_sequence_version: 1` is retained for all three encodings**. Consumers must
use `filename_pattern` for the actual extension, rather than assume PNG from that
legacy version-key name. `alpha` is `straight` for PNG/WebP and `none` for JPEG;
`background` is `[0,0,0,0]` or the selected JPEG `[r,g,b,255]`. Existing PNG sidecars
remain byte-identical. Duration, frame count, looping intent, dimensions, sampling,
skin and fixed world-space camera retain their meanings. No timestamps or absolute
paths are embedded. Encoders and the document/settings are fixed for repeatability;
there is no promise of identical lossy bytes across future codec versions.

Spritesheets remain **PNG only** with their existing Ankhimate JSON contract;
changing the sequence format does not change spritesheet encoding or metadata.

## Optional pixel bounds, trim and padding

T-601.2 adds explicit global pixel bounds, per-frame trim and sequence padding,
all off by default. See [rendered framing](rendered-framing.md) for reconstruction
metadata, transparent placeholders, JPEG behavior and reader requirements.
The descriptions below are the historical defaults; optional crops do not change
the common camera or sample times.

## Batch selection (T-601.3)

Optional [Batch export](batch-export.md) provides animation checkboxes, counts and
independent child folders in a new destination. Individual mode remains default;
its metadata and filenames are unchanged. Shared settings are captured once,
while each clip prepares its own camera/reconstruction.

## Time and framing

Frame `i` evaluates the existing core engine at `i / fps` seconds. The end is
exclusive for both looping and non-looping clips: a one-second clip at 60 FPS
produces exactly 60 frames, from 0 through 59/60 seconds. For a non-integral
duration × FPS, the count rounds up. Float representation noise immediately
around an integer is normalized. The metadata retains the authored duration;
playing the image sequence holds its final sampled frame until the next frame
boundary, so playback duration can exceed that duration by less than one frame.
The exact terminal pose is not appended to a non-looping clip.

A first pass evaluates every output pose and unions its rendered bounds. One
camera fits this union with the CPU renderer's existing 8% padding and is reused
for every frame. This preserves translations and avoids per-frame zoom changes.
The viewport's editor camera, selected bones, current skin selection, playback
state and physics accumulator do not enter this calculation.

An existing clip with positive duration and no keys exports a static sequence.
Missing clips, zero/non-finite durations, FPS outside 1–240, dimensions outside
1–8192 and counts outside 1–100000 are refused before writing. Images that cannot
be decoded or referenced textures that are missing are reported as errors.

## Fidelity and current limits

This reuses `ankhimate-render`, not another rasterizer. It samples bones,
attachment/visibility/color timelines, draw order, texture sequences and the
existing stateless IK, transform and path constraints through `core::evaluate`.
It renders the **default skin** only. The sequence API caches decoded textures;
poses and encoded frames are produced one at a time.

**Physics is unsupported. Every project containing a physics constraint is
refused**, including an inactive or zero-mix one. The CPU preview evaluator
does not advance a `PhysicsState`; ignoring such constraints would produce
incorrect frames. Deterministic simulation, warm-up and loop behavior require a
separate milestone. This conservative refusal does not infer clip dependencies.

**Only Normal slot blending is accepted.** Additive, Multiply and Screen depend
on compositing assumptions that have not been validated for standalone RGBA
sequences. Projects containing these slot modes are refused rather than silently
flattened incorrectly. Dark tint, arbitrary deformations and unusual clipping
setups retain the CPU renderer's implementation; they are not certified by the
walker/PSD acceptance fixtures. No claim of complete GPU-renderer equivalence is
made. Events are not turned into game logic by an image sequence.

The existing renderer now correctly composes straight-alpha pixels, filters
textures through premultiplied colors before returning straight alpha, and uses
half-open triangle edges. These prevent dark fringes and double opacity on a
quad diagonal. Opaque preview images can differ at a few rasterization-edge
pixels; the reference comparison is recorded in the milestone report.

## Progress, cancellation and files

Framing, texture preparation, rendering, encoding and publication run in a
background worker. The document and reviewed encoding/background are captured at
launch. Current-frame RGBA, encoded bytes, codec working buffers, decoded textures
and lightweight per-file transaction entries are held in memory, rather than all
frames. JPEG also allocates the current RGB frame; generated JPEG/WebP bytes are
decoded for validation before staging. This adds per-frame memory and CPU cost,
without accumulating the sequence. Very large documents still incur the initial snapshot
copy and the existing renderer's per-frame memory cost. Maximum-size frames and
100000-frame sequences have not been performance-qualified.

The progress bar reports framing/rendering, and **Cancel image export** requests
cooperative cancellation. A frame encode or filesystem operation already in
progress finishes before the next cancellation point. Cancellation is checked
between poses, staged files, flushes and publication operations; rollback itself
cannot be cancelled. There is no hard cancellation-latency guarantee.

New/Open cancels a running export while retaining the worker until it finishes;
its result cannot replace the new document. Ordinary application Close waits
for cancellation/rollback before issuing the final close command. Cancelling
the unsaved-changes Close prompt leaves the export running.

The M001b publisher validates paths/collisions and prepares all output files in
a unique staging directory on the destination filesystem before publishing any.
For every image-sequence format, **any existing target name is refused**; choose another
folder, including when exporting another clip. Other files in the folder are
preserved. No overwrite-confirmation dialog is needed because replacement is
not offered. A collision is checked again by the publisher, not just by the UI.

Ordinary preparation failure/cancellation leaves previous output untouched.
Publication failure attempts reverse rollback. A failed rollback or cleanup is
reported with recovery information; retained backups/manifests are preserved.
Readers can observe a partially published set. Forced process termination,
power loss and non-cooperating filesystem writers remain outside these ordinary
failure guarantees. There is no multi-file atomicity, durable commit pointer or
automatic export crash recovery. See [export safety](export-safety.md).

## Validation

See [M004.1 validation](milestones/M0041-validation.md) for automated regression
coverage, real Windows UI tests and the walker/tagged PSD sequence evidence.

T-601.1 adds headless tests on the real walker clip: 60 frames per format,
WebP/PNG decoded RGBA equality and repeatability, explicit JPEG backgrounds,
metadata consistency, Unicode paths, cancellation, collisions, invalid parameters,
missing assets, physics/blending refusal and encoding failures. JPEG acceptance
for the tested 64×64 walker frames is per-frame RGB MAE ≤ 4 and RMSE ≤ 12 (0–255),
not a claim of losslessness or a universal image-quality bound. Historical walker
walk/leg_ik and tagged PSD PNG/spritesheet files are compared byte for byte.
No GUI or WebGL qualification is added by this task. Per-frame trim and global pixel crops are now available in T-601.2; general
background controls, physics stepping, maximum-size performance and the full original T-601
viewport-golden/budget acceptance remain open; T-601 stays partial.
