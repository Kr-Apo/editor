# Rendered framing, trim and padding — T-601.2

These opt-in controls apply to **Export → Image sequence** (PNG/JPEG/WebP) and
**Export → Spritesheet** (PNG + Ankhimate JSON). They reuse `Sequence::prepare`,
the existing common-camera CPU renderer and the staged publisher. Nothing is
written into the authored project, and runtime v1/v2 are unchanged.

## Modes and defaults

| Control | Meaning | Default |
|---|---|---|
| Historical framing | Existing union of evaluated drawing geometry, with the existing bone fallback when no drawing exists; existing 8% fit margin. | Selected |
| Global pixel bounds | Render every sampled frame with that same camera, union useful RGBA pixel rectangles, then crop all frames to that one union. No new zoom, resampling or per-frame camera movement. | Off |
| Trim transparent pixels per frame | Crop each rendered frame to its own useful RGBA rectangle. When combined with global bounds, the per-frame rectangles are contained in the common union. | Off |
| Image padding | Sequence-only guard band outside the cropped content, nearest-edge straight RGBA extrusion, including corners. Range 0–32 px. | 0 |
| Spritesheet padding | Existing 0–32 px guard band around each frame rectangle. | Existing 2 px |

Canvas width/height are the **original render canvas**, before any crop/padding.
Global bounds preserves one output size and offset across the clip; per-frame
trim can produce variable sequence dimensions. Neither mode magnifies the art.
Useful pixel bounds include every pixel with nonzero RGBA bytes, **including RGB
under alpha zero**: the CPU renderer can retain color when tiny alpha rounds to
zero. This conservative rule preserves exact decoded PNG/WebP bytes. It may keep
a slightly wider invisible fringe than an alpha-only crop. No positive-alpha
threshold, JPEG decoding or bone-only bound determines the crop.

An entirely zero RGBA frame becomes a 1×1 transparent placeholder with trim.
If the whole clip is zero RGBA, global bounds also uses a common 1×1 rectangle.
Padding surrounds that placeholder as usual. A visually transparent frame with
nonzero hidden RGB keeps those bytes instead. The `empty` flag means all alpha
values are zero; it does not authorize throwing away hidden RGB if byte equality
is required. Historical mode without trim retains full canvas dimensions.

Individual mode still selects one existing clip per job. T-601.3 adds optional
[batch checkboxes](batch-export.md), with independent child folders, bounds,
sampling and camera per animation. Shared settings do not imply a shared camera.

## Optional reconstruction extension, version 1

Old settings emit precisely the old bytes and metadata. New cropping, or nonzero
sequence padding, adds **`reconstruction`** to the existing JSON. The outer
`png_sequence_version: 1` / `spritesheet_version: 1` identifiers remain unchanged;
no existing field is renamed. The preflight requires explicit acknowledgement
that a reader must understand the extension. It is an additive, opt-in contract:
old exports remain readable, but old untrimmed-only readers are **not** upgraded.
This is not Phaser-native atlas JSON. The current independent Phaser sheet example
rejects trimmed sheets; no TypeScript consumer is changed by this task.

```json
{
  "reconstruction": {
    "version": 1,
    "required": true,
    "options": {"bounds": "global_pixels", "trim": true, "padding": 2},
    "source_size": {"w": 64, "h": 64},
    "global_rect": {"x": 8, "y": 10, "w": 45, "h": 40},
    "frames": [{
      "index": 0,
      "source_rect": {"x": 12, "y": 15, "w": 20, "h": 18},
      "content_rect": {"x": 2, "y": 2, "w": 20, "h": 18},
      "image_size": {"w": 24, "h": 22},
      "empty": false
    }]
  }
}
```

Rectangles are integer, top-left origin, Y down, right/bottom exclusive.
`source_rect` is in the original canvas; `content_rect` is in the encoded image,
excluding guard pixels. Their `w/h` agree. `image_size` is the actual sequence
image size or the complete sheet page size. `index` identifies the root frame
record / sequence filename index. For sheets, use the root frame's `page` and
page filename; `content_rect` equals that frame's `rect`.

To reconstruct PNG/WebP: allocate a zero RGBA `source_size` canvas, then copy the
`content_rect` pixels to `source_rect.x/y`, without blending or resampling. Ignore
the guard band. Source offsets and the untouched camera recover exact placement.
For JPEG, initialize an opaque canvas with the explicit `background` RGB, then
copy decoded RGB content. Reconstruction is geometrically exact but colors are
lossy. JPEG bounds are computed on CPU RGBA **before** padding/background encoding;
fully transparent pixels compose onto the chosen background as in T-601.1.

Sequence root `width/height` continue describing the original canvas. Root FPS,
duration, looping, filename pattern, frame count and `i / fps` end-exclusive
sampling retain their meanings. Actual variable encoded sizes come from the
extension. Sheets retain `source_size` and original normalized pivot `{0.5,0.5}`;
`trim:true` indicates enabled cropping, `rect` now encloses cropped content.
Original pivot/origin is **not** a pivot in the cropped texture. The crop-relative
pivot is `(source_size.w/2 - source_rect.x)/source_rect.w` and the analogous Y
expression; it can lie outside [0,1]. For world position `(x,y)`:

```text
source_px = source_size.w/2 + (x - camera.center[0]) * camera.zoom
source_py = source_size.h/2 - (y - camera.center[1]) * camera.zoom
image_px  = source_px - source_rect.x + content_rect.x
image_py  = source_py - source_rect.y + content_rect.y
```

## Packing, responsiveness and file safety

Spritesheets retain deterministic **source-sized fixed cells** and row-major
pages. Cropping reduces each content rectangle, not the reserved cell or page
count. Every cropped rectangle plus its padding fits inside its own original
cell, so guard bands cannot overlap. Oversized source canvas plus padding is
refused even if a hypothetical crop could fit. There is no new compact packer.
The page cap remains 4096 px and existing historical PNGs are unchanged.

New modes add one full pixel-analysis pass on the background worker. Only one
rendered frame is scanned at a time, with lightweight rectangles per frame.
Rendering/encoding rerenders each pose; no whole sequence RGBA is retained.
During a cropped page render, owned RGBA is bounded by one page plus two original
frames (page ≤4096², source plus guards must fit the configured page). The JSON
`rgba_buffer_bound_bytes` includes that additional crop copy for new sheet modes.
For sequences, source/crop buffers overlap temporarily; guard-band and codec/
validation buffers add per-frame cost. These are not bounds on total process
memory: textures, document, metadata and codec scratch are additional. Long-clip
and maximum-size throughput/cancellation latency remain unqualified.

Progress includes Pixel bounds. Cancellation is checked before each analysis
frame, at its end, between renders and by the existing staged publisher. An
in-progress frame render/encode finishes first. Cancelling the preflight writes
nothing. Options are captured by the reviewed job; later changes cannot alter it.
Sequence and sheet padding are kept independent. Path checks, collisions, staging,
Windows replacement rules and reverse rollback remain authoritative. Readers can
observe partial publication; abrupt crashes/power loss and unrelated writers are
not made atomic or durable. See [export safety](export-safety.md).

Default skin, Normal blending and explicit refusal of physics are unchanged.
This milestone adds headless tests, not a new GUI/WebGL qualification. General
background controls, dense trim packing, physics stepping,
maximum-size performance and the complete viewport golden acceptance stay open.
T-601 remains 🟡.
