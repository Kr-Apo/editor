# Animated spritesheets — M004.2

In **Export → Spritesheet**, choose an existing clip, FPS, frame width/height,
maximum page size and padding. Scroll within a small export pane to reach the
export button and result. Export to a fresh folder. Existing output names are
refused; other files are preserved. The output is `spritesheet.json` and
`sheet_000000.png`, `sheet_000001.png`, … . There is no overwrite mode.

This uses the M004.1 `Sequence::prepare`, common camera, core evaluator and CPU
renderer. It exports the default skin and Normal blending only; projects with
physics constraints are refused. No skeletal runtime is required to play the
result, and runtime formats v1/v2 are unchanged.

## Optional cropping (T-601.2)

Historical defaults below stay byte-identical. Global pixel bounds and per-frame
trim are opt-in and require the [reconstruction extension](rendered-framing.md).
Cropped rects keep source-canvas offsets and source-sized cells; no denser packing
or new Phaser consumer is supplied. The existing example refuses cropped sheets.

## Multiple animations (T-601.3)

[Batch export](batch-export.md) writes each checked clip's pages and existing JSON
inside its own child folder. Page indices and camera/reconstruction remain local
to that clip. Cropped sheets still require the T-601.2 reconstruction reader;
the existing Phaser example is not changed.

## Sampling and packing

The frame count and sample times are exactly those of [PNG sequences](png-sequences.md):
`t = index / fps`, end exclusive, 60 frames for one second at 60 FPS. Both looping
and non-looping clips use this sampling rule. Non-integral duration × FPS rounds
up except for normalized floating-point representation noise. The frame duration
is always `1000 / fps` milliseconds; image playback can extend the authored
duration by less than a frame. No terminal pose is appended.

Historical packing is deterministic and does not rotate or trim frames. Let `p` be padding,
`cw = frame_width + 2p`, `ch = frame_height + 2p`. A page can hold
`floor(max_page/cw) × floor(max_page/ch)` frames. Cells run left-to-right, then
top-to-bottom. New pages continue increasing frame indices. A final page uses
only its occupied columns/rows, so actual page dimensions can be smaller than
the maximum; unused cells in a partial row remain fully transparent. Always use
the rectangles and page dimensions in the JSON rather than inferring them.

Padding is an individual guard band on all four sides of every frame. It repeats
the nearest edge pixel, including its **straight RGBA** and corner pixels, without
blending. Frame rectangles exclude this padding, preserving the original image
exactly. Default padding is 2 px; permitted range is 0–32 px. Setting 0 explicitly
disables the guard band. Do not assume isolation at arbitrary downscales or with
mipmap levels wider than the available padding; mipmap/LOD behavior is not
certified in this milestone. The Phaser example uses linear filtering at 1:1.

FPS must be 1–240; M004.1 frame dimensions must be 1–8192. The page limit is 1–4096
pixels per side, and the entire padded frame must fit. Consequently a frame
larger than the page limit is refused before staging or image decoding. The
sequence is limited to 100000 frames. Invalid timing, missing clips/textures,
undecodable assets and the existing physics/blending exclusions are reported.

## Metadata contract: `spritesheet_version: 1`

This is a separate rendered-output format, **not** Ankhimate runtime v1 or v2,
and not Phaser's built-in atlas JSON schema. Consumers must inspect the version
and refuse unsupported versions. The included Phaser reader registers explicit
rectangles through public texture APIs; no runtime adapter is patched.

| Field | Meaning |
|---|---|
| `spritesheet_version` | Integer 1 |
| `animation`, `fps`, `duration`, `looping`, `frame_count` | Authored clip name, sampling FPS, authored duration in seconds, loop intent, total frames |
| `sampling` | `i / fps; end exclusive` |
| `alpha` | `straight`; pages are PNG RGBA8 with transparent background |
| `padding`, `extrude` | Guard-band width in pixels; both equal the configured padding |
| `trim`, `rotated` | Both false by default; optional pixel crops set `trim:true` and include `reconstruction`; rotation remains false |
| `max_page_size` | Configured maximum page side, not an actual PNG dimension |
| `skin` | Default skin name actually rendered |
| `camera.center`, `camera.zoom` | World-space camera center `[x,y]` (Y up) and pixels per world unit, common to all frames |
| `origin.pivot` | Normalized frame pivot `{x:0.5,y:0.5}`; anchor corresponds to `origin.world` |
| `origin.world` | World point at that common pivot, equal to camera center |
| `origin.world_zero_in_frame` | Pixel position of world [0,0] inside each frame (screen Y down); may lie outside the frame |
| `pages[]` | Explicit `index`, relative PNG `image`, actual pixel `width`, `height` |
| `frames[]` | Explicit `index`, unique `name`, `page` index, `rect:{x,y,w,h}`, `source_size:{w,h}`, `duration_ms`, normalized `pivot` |
| `rgba_buffer_bound_bytes` | Upper bound for the explicitly owned page + current frame RGBA buffers; not total process memory |

Frame/page indices start at zero. Reconstruct animation order by **frame index**;
resolve its page by **page index**, not by array position or filename sorting.
Rectangles use top-left pixel coordinates with Y down. `source_size` equals the
frame dimensions in historical mode. Optional crop source offsets are in `reconstruction`. `duration_ms` uses milliseconds even
though authored `duration` is in seconds. Every frame has the same pivot, which
keeps the subject's placement stable through both frame and page changes.

For a world point `(x,y)`, its frame position is:

```text
px = frame_width / 2 + (x - camera.center[0]) * camera.zoom
py = frame_height / 2 - (y - camera.center[1]) * camera.zoom
```

Names are serialized with serde_json, including quotes, controls and Unicode.
The metadata contains no timestamps or absolute source/output paths.

## Memory, responsiveness and file safety

The worker prepares one page at a time. `Sequence::render_rgba` returns the
existing renderer's frame before encoding; the page copies that frame and its
guard band and then discards the frame. It does not retain the sequence RGBA.
One page plus one frame is at most **134217728 bytes (128 MiB)**, given the 4096
page cap. This excludes decoded project textures, the document snapshot,
metadata/layout records, the one encoded PNG and PNG codec scratch. Total memory
is not independently bounded by 128 MiB. Metadata/transaction records scale with
frame/page count; very long clips have not been performance-qualified.

The UI computes only a constant-space page count; it does not rebuild all frame
rectangles per repaint. The worker alone builds the full layout once. Progress,
cancellation, snapshot isolation, New/Open cancellation and ordinary Close
waiting are shared with M004.1. Cancellation is checked between individual
frames and filesystem operations; an encode already in progress finishes first.
There is no hard cancellation-latency guarantee.

All pages and JSON are staged and flushed before publication through the
unchanged generated-PNG publisher. Ordinary failure/cancellation preserves
previous files; a publication error triggers reverse rollback. Failure to roll
back or clean up is explicitly reported, with retained transaction data.
No partial result is announced as complete. This is **not multi-file atomicity**:
concurrent readers can see a mixed set, and forced termination, OS/power failure
or non-cooperating writers retain the limitations in [export safety](export-safety.md).
There is no durable export journal or automatic crash recovery.

## Phaser example

The independent [Phaser example](../examples/spritesheet-phaser/README.md) includes
three real clips, a versioned metadata reader, public-API loading/animation,
pixel comparisons against accepted M004.1 PNGs and a reproducible WebGL test.
It does not depend on `@ankhimate/runtime` or the external runtime repository.
See [M004.2 validation](milestones/M0042-validation.md) for results and exclusions.
