---
title: Ankh formats and migration policy
description: Shipped author and runtime wire contracts, validation boundaries and version qualification.
---

# Ankh formats and migration policy

This specification describes the shipped code, including its limitations. Source references
are relative to the repository root. A Rust type being serializable is not evidence of an
editor authoring tool or visual fidelity in every runtime.

## Version matrix

| Contract | Writer | Reader / historical compatibility | Proof |
|---|---|---|---|
| Author schema version 1 | `formats/src/convert.rs` | Current reader accepts exactly 1 | Three authentic fixtures; profile/save/reopen tests |
| ANKH envelope version 1, codec 1 | `formats/src/container.rs` | Only envelope 1 and codec 1 | Header, corruption and version tests |
| Prototype ZIP + project.json | Removed at `f3defa55` | Not supported by current reader; historical numeric labels are not the current schema | Authentic parent-commit ZIP rejection fixture |
| Author v0 | No authenticated fixture found in examined history | No shipped migration; not qualified | Open T-605 criterion; never synthesize a v0 success |
| Native runtime `format_version: 2` | Built-in native preset | External TypeScript `parseRig` accepts runtime 1 and 2; missing format_version means 1 | Existing export/parity regressions and T-605 shape test |
| Export `context_version: 1` | `export/src/context.rs` | TypeScript requires 1 | Separate template context contract |
| Export `preset_version: 1` | Preset schema | Preset parser | Not an author/runtime version |
| Save recovery `save_version: 1` | `formats/src/safe_write.rs` | Recovery manifest reader | Separate publication metadata |

Application/package/npm versions are independent. `.ankh.runtime` in planning documents is
a conceptual target name: the shipped native preset writes JSON (normally `skeleton.json`)
and atlas pages, not a second project container or an implemented postcard twin.
The external runtime also has a separate author-file adapter (`core/src/ankh.ts`); this is
not the native runtime JSON reader or evidence of prototype ZIP compatibility.

<a id="author-v1"></a>
## Author version 1 profiles and envelope

| Profile | Writer suffix | Encoding |
|---|---|---|
| Binary | `.ankh` (default for other suffixes) | Compact-key MessagePack, optional raw Deflate |
| Readable | `.ankh.json` | UTF-8 JSON, descriptive keys |
| Compact JSON | `.ankh.min.json` | UTF-8 JSON, stable short keys |

Load sniffs `ANKH` magic rather than trusting the extension. Nonbinary input is JSON;
a top-level `version` selects descriptive keys, otherwise short keys are expanded.
Writers choose the profile using a lowercase suffix. Use the canonical lowercase names
above: asset-root suffix stripping is case sensitive. Slotmap IDs are never serialized.

| Byte offset | Size | Meaning |
|---|---|---|
| 0 | 4 | ASCII ANKH |
| 4 | 2 | Little-endian envelope version 1 |
| 6 | 1 | Codec 1, compact-key MessagePack |
| 7 | 1 | Bit 0: raw Deflate; other bits rejected by the T-813 reader |
| 8 | 4 | Little-endian uncompressed byte count |
| 12 | 4 | CRC-32 of uncompressed payload |
| 16 | remainder | Payload; writer compresses only if smaller |

Bad header, unknown envelope/codec, incorrect length or CRC fail. Inflation occurs before
length validation: there is no promised decompression resource budget. Short keys use the
append-only base-62 table in `formats/src/compact.rs`; never reorder or reuse entries.
Unknown object keys are escaped with `~` by contraction/expansion. This value-tree property
must not be confused with preservation through the loaded document model.

## Reading the normative author field tables

The tables below inventory all 33 serializable schema declarations, including 206 fields.
They are verified against `formats/src/schema.rs` and `format-schema.json` by normal Rust
tests. Rust primitive types specify representation: String is JSON string, bool boolean,
f32 number, u32 nonnegative integer, i32 signed integer; vectors are arrays, maps objects,
fixed arrays have exactly the stated length and tuples are positional arrays.
`Option<T>` accepts missing or null as None; it writes null unless skip_serializing_if
suppresses it. `extra: Extra` is **flattened unknown object members**, never a literal
extra property. `#[serde(rename = "type")]` names the wire member `type`.
Enum tags and rename_all on the container control wire spelling (snake_case for Timeline,
lowercase for Axis/Interp/Attachment). Newtype attachment variants inline their payload.
`Vec2Key` is a serializable helper, not the payload of current bone axis tracks.

| Annotation / helper | Missing-input value / output rule |
|---|---|
| No default | Required, except Option; wrong types fail |
| default | Rust Default: zero, false, empty collection/string, None; Axis X |
| one / yes / half / white | 1 / true / 0.5 / [1,1,1,1] |
| group_color_default | [0.55,0.58,0.65,1] |
| marker_color_default | [0.95,0.72,0.30,1] |
| stretch_limit_default | 1.1 |
| flatten | Members belong to the containing object, not a nested property |
| skip_serializing_if | Exact omission predicate shown in each field row |
| Interp flattened default | See executable key tests; missing curve is rejected by current Serde flatten reader despite the default annotation |

Disk angles are degrees, CCW positive; coordinates are Y-up. Setup angles convert to core
radians (f32 rounding applies), animation angle keys retain the degree convention.
Translate/shear keys are additive offsets, scale keys multiplicative; X/Y are independent
scalar timelines. Scalar/color/deform interpolation uses `curve: linear`, `stepped`, or
`bezier` with four `handles` [out_x,out_y,in_x,in_y]. Author curves describe the segment arriving at the key; native scalar/deform curves describe the outgoing segment (the exporter shifts that convention). Visibility, attachment and draw-order
keys have no curve field and are stepped. Bone offsets are seconds; markers are editor-only.
Skin entries identify a slot and attachment name and hold a concrete attachment payload;
linked meshes identify their source skin/slot/attachment and deform inheritance.

<!-- schema-fields:start -->
### `Project` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Project | `version` | `u32` | required (Option accepts missing/null) |
| Project | `name` | `String` | required (Option accepts missing/null) |
| Project | `fps` | `u32` | required (Option accepts missing/null) |
| Project | `assets` | `Vec<Asset>` | `#[serde(default)]` |
| Project | `bones` | `Vec<Bone>` | `#[serde(default)]` |
| Project | `slots` | `Vec<Slot>` | `#[serde(default)]` |
| Project | `draw_order` | `Vec<String>` | `#[serde(default)]` |
| Project | `skins` | `Vec<Skin>` | `#[serde(default)]` |
| Project | `default_skin` | `String` | `#[serde(default)]` |
| Project | `constraints` | `Vec<Constraint>` | `#[serde(default)]` |
| Project | `constraint_order` | `Vec<String>` | `#[serde(default)]` |
| Project | `animations` | `Vec<Animation>` | `#[serde(default)]` |
| Project | `groups` | `Vec<Group>` | `#[serde(default, skip_serializing_if = "Vec::is_empty")]` |
| Project | `psd_layer_paths` | `std::collections::BTreeMap<String, String>` | `#[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]` |
| Project | `export_presets` | `Vec<serde_json::Value>` | `#[serde(default, skip_serializing_if = "Vec::is_empty")]` |
| Project | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Group` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Group | `name` | `String` | required (Option accepts missing/null) |
| Group | `color` | `[f32; 4]` | `#[serde(default = "group_color_default")]` |
| Group | `members` | `Vec<String>` | `#[serde(default)]` |
| Group | `parent` | `String` | `#[serde(default, skip_serializing_if = "String::is_empty")]` |

### `Asset` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Asset | `name` | `String` | required (Option accepts missing/null) |
| Asset | `file` | `String` | required (Option accepts missing/null) |
| Asset | `width` | `u32` | required (Option accepts missing/null) |
| Asset | `height` | `u32` | required (Option accepts missing/null) |
| Asset | `source_path` | `Option<String>` | `#[serde(default)]` |
| Asset | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Bone` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Bone | `name` | `String` | required (Option accepts missing/null) |
| Bone | `parent` | `String` | `#[serde(default)]` |
| Bone | `length` | `f32` | required (Option accepts missing/null) |
| Bone | `tx` | `f32` | `#[serde(default)]` |
| Bone | `ty` | `f32` | `#[serde(default)]` |
| Bone | `rotation` | `f32` | `#[serde(default)]` |
| Bone | `sx` | `f32` | `#[serde(default = "one")]` |
| Bone | `sy` | `f32` | `#[serde(default = "one")]` |
| Bone | `shear_x` | `f32` | `#[serde(default)]` |
| Bone | `shear_y` | `f32` | `#[serde(default)]` |
| Bone | `inherit_rotation` | `bool` | `#[serde(default = "yes")]` |
| Bone | `inherit_scale` | `bool` | `#[serde(default = "yes")]` |
| Bone | `inherit_reflect` | `bool` | `#[serde(default = "yes")]` |
| Bone | `color` | `Option<[f32; 4]>` | `#[serde(default)]` |
| Bone | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Slot` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Slot | `name` | `String` | required (Option accepts missing/null) |
| Slot | `bone` | `String` | required (Option accepts missing/null) |
| Slot | `attachment` | `Option<String>` | `#[serde(default)]` |
| Slot | `color` | `[f32; 4]` | `#[serde(default = "white")]` |
| Slot | `dark_color` | `Option<[f32; 4]>` | `#[serde(default)]` |
| Slot | `blend_mode` | `String` | `#[serde(default)]` |
| Slot | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Skin` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Skin | `name` | `String` | required (Option accepts missing/null) |
| Skin | `entries` | `Vec<SkinEntry>` | `#[serde(default)]` |
| Skin | `bones` | `Vec<String>` | `#[serde(default, skip_serializing_if = "Vec::is_empty")]` |
| Skin | `constraints` | `Vec<String>` | `#[serde(default, skip_serializing_if = "Vec::is_empty")]` |
| Skin | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `SkinEntry` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| SkinEntry | `slot` | `String` | required (Option accepts missing/null) |
| SkinEntry | `name` | `String` | required (Option accepts missing/null) |
| SkinEntry | `attachment` | `Attachment` | required (Option accepts missing/null) |

### `Attachment` (enum)

Container attributes: `#[serde(tag = "type", rename_all = "lowercase")]`.

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Attachment | `Region` | `Region` | — |
| Attachment | `Mesh` | `Mesh` | — |
| Attachment | `Clipping` | `Clipping` | — |
| Attachment | `Path` | `Path` | — |
| Attachment | `BoundingBox` | `BoundingBox` | — |
| Attachment | `Point` | `Point` | — |

### `BoundingBox` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| BoundingBox | `vertices` | `Vec<f32>` | `#[serde(default)]` |
| BoundingBox | `weights` | `Vec<Vec<(String, f32)>>` | `#[serde(default, skip_serializing_if = "Vec::is_empty")]` |
| BoundingBox | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Point` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Point | `x` | `f32` | `#[serde(default)]` |
| Point | `y` | `f32` | `#[serde(default)]` |
| Point | `rotation` | `f32` | `#[serde(default)]` |
| Point | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Sequence` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Sequence | `frames` | `Vec<String>` | `#[serde(default)]` |
| Sequence | `fps` | `f32` | `#[serde(default)]` |
| Sequence | `mode` | `String` | `#[serde(default)]` |
| Sequence | `setup_index` | `u32` | `#[serde(default)]` |

### `LinkedMesh` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| LinkedMesh | `skin` | `Option<String>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| LinkedMesh | `slot` | `String` | required (Option accepts missing/null) |
| LinkedMesh | `attachment` | `String` | required (Option accepts missing/null) |
| LinkedMesh | `inherit_deform` | `bool` | `#[serde(default = "yes")]` |

### `Path` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Path | `vertices` | `Vec<f32>` | `#[serde(default)]` |
| Path | `closed` | `bool` | `#[serde(default)]` |
| Path | `constant_speed` | `bool` | `#[serde(default = "yes")]` |
| Path | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Clipping` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Clipping | `vertices` | `Vec<f32>` | `#[serde(default)]` |
| Clipping | `end_slot` | `Option<String>` | `#[serde(default)]` |
| Clipping | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Region` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Region | `texture` | `String` | required (Option accepts missing/null) |
| Region | `offset_x` | `f32` | `#[serde(default)]` |
| Region | `offset_y` | `f32` | `#[serde(default)]` |
| Region | `rotation` | `f32` | `#[serde(default)]` |
| Region | `scale_x` | `f32` | `#[serde(default = "one")]` |
| Region | `scale_y` | `f32` | `#[serde(default = "one")]` |
| Region | `width` | `f32` | required (Option accepts missing/null) |
| Region | `height` | `f32` | required (Option accepts missing/null) |
| Region | `uv` | `[f32; 4]` | `#[serde(default)]` |
| Region | `pivot_x` | `f32` | `#[serde(default = "half")]` |
| Region | `pivot_y` | `f32` | `#[serde(default = "half")]` |
| Region | `sequence` | `Option<Sequence>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| Region | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Mesh` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Mesh | `texture` | `String` | required (Option accepts missing/null) |
| Mesh | `vertices` | `Vec<f32>` | `#[serde(default)]` |
| Mesh | `uvs` | `Vec<f32>` | `#[serde(default)]` |
| Mesh | `triangles` | `Vec<u32>` | `#[serde(default)]` |
| Mesh | `edges` | `Vec<u32>` | `#[serde(default, skip_serializing_if = "Vec::is_empty")]` |
| Mesh | `weights` | `Vec<Vec<(String, f32)>>` | `#[serde(default)]` |
| Mesh | `linked` | `Option<LinkedMesh>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| Mesh | `sequence` | `Option<Sequence>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| Mesh | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Constraint` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Constraint | `name` | `String` | required (Option accepts missing/null) |
| Constraint | `kind` | `String` | `#[serde(rename = "type")]` |
| Constraint | `target` | `String` | required (Option accepts missing/null) |
| Constraint | `bones` | `Vec<String>` | `#[serde(default)]` |
| Constraint | `bend_direction` | `f32` | `#[serde(default = "one")]` |
| Constraint | `mix` | `f32` | `#[serde(default = "one")]` |
| Constraint | `softness` | `f32` | `#[serde(default)]` |
| Constraint | `stretch` | `bool` | `#[serde(default)]` |
| Constraint | `stretch_limit` | `f32` | `#[serde(default = "stretch_limit_default")]` |
| Constraint | `stiffness` | `f32` | `#[serde(default)]` |
| Constraint | `transform_mix` | `Option<TransformMix>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| Constraint | `offsets` | `Option<[f32; 7]>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| Constraint | `local` | `bool` | `#[serde(default, skip_serializing_if = "std::ops::Not::not")]` |
| Constraint | `relative` | `bool` | `#[serde(default, skip_serializing_if = "std::ops::Not::not")]` |
| Constraint | `physics` | `Option<[f32; 4]>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| Constraint | `forces` | `Option<[f32; 4]>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| Constraint | `channels` | `Option<[bool; 2]>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| Constraint | `slot` | `Option<String>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| Constraint | `path` | `Option<[f32; 4]>` | `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| Constraint | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `Animation` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Animation | `name` | `String` | required (Option accepts missing/null) |
| Animation | `duration` | `f32` | required (Option accepts missing/null) |
| Animation | `looping` | `bool` | `#[serde(default = "yes")]` |
| Animation | `timelines` | `Vec<Timeline>` | `#[serde(default)]` |
| Animation | `events` | `Vec<Event>` | `#[serde(default, skip_serializing_if = "Vec::is_empty")]` |
| Animation | `markers` | `Vec<Marker>` | `#[serde(default, skip_serializing_if = "Vec::is_empty")]` |
| Animation | `bone_offsets` | `Vec<BoneOffset>` | `#[serde(default, skip_serializing_if = "Vec::is_empty")]` |
| Animation | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `BoneOffset` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| BoneOffset | `bone` | `String` | required (Option accepts missing/null) |
| BoneOffset | `offset` | `f32` | required (Option accepts missing/null) |

### `Marker` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Marker | `time` | `f32` | required (Option accepts missing/null) |
| Marker | `name` | `String` | required (Option accepts missing/null) |
| Marker | `color` | `[f32; 4]` | `#[serde(default = "marker_color_default")]` |

### `Event` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Event | `time` | `f32` | required (Option accepts missing/null) |
| Event | `name` | `String` | required (Option accepts missing/null) |
| Event | `int_value` | `i32` | `#[serde(default, skip_serializing_if = "is_zero_i32")]` |
| Event | `float_value` | `f32` | `#[serde(default, skip_serializing_if = "is_zero_f32")]` |
| Event | `string_value` | `String` | `#[serde(default, skip_serializing_if = "String::is_empty")]` |
| Event | `audio` | `String` | `#[serde(default, skip_serializing_if = "String::is_empty")]` |
| Event | `volume` | `f32` | `#[serde(default = "one", skip_serializing_if = "is_one_f32")]` |
| Event | `balance` | `f32` | `#[serde(default, skip_serializing_if = "is_zero_f32")]` |

### `Timeline` (enum)

Container attributes: `#[serde(tag = "kind", rename_all = "snake_case")]`.

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Timeline | `BoneTranslate` | tagged variant | — |
| Timeline.BoneTranslate | `bone` | `String` | required (Option accepts missing/null) |
| Timeline.BoneTranslate | `axis` | `Axis` | `#[serde(default)]` |
| Timeline.BoneTranslate | `keys` | `Vec<ScalarKey>` | required (Option accepts missing/null) |
| Timeline.BoneTranslate | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |
| Timeline | `BoneRotate` | tagged variant | — |
| Timeline.BoneRotate | `bone` | `String` | required (Option accepts missing/null) |
| Timeline.BoneRotate | `keys` | `Vec<ScalarKey>` | required (Option accepts missing/null) |
| Timeline | `BoneScale` | tagged variant | — |
| Timeline.BoneScale | `bone` | `String` | required (Option accepts missing/null) |
| Timeline.BoneScale | `axis` | `Axis` | `#[serde(default)]` |
| Timeline.BoneScale | `keys` | `Vec<ScalarKey>` | required (Option accepts missing/null) |
| Timeline.BoneScale | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |
| Timeline | `BoneShear` | tagged variant | — |
| Timeline.BoneShear | `bone` | `String` | required (Option accepts missing/null) |
| Timeline.BoneShear | `axis` | `Axis` | `#[serde(default)]` |
| Timeline.BoneShear | `keys` | `Vec<ScalarKey>` | required (Option accepts missing/null) |
| Timeline.BoneShear | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |
| Timeline | `SlotColor` | tagged variant | — |
| Timeline.SlotColor | `slot` | `String` | required (Option accepts missing/null) |
| Timeline.SlotColor | `keys` | `Vec<ColorKey>` | required (Option accepts missing/null) |
| Timeline | `SlotVisible` | tagged variant | — |
| Timeline.SlotVisible | `slot` | `String` | required (Option accepts missing/null) |
| Timeline.SlotVisible | `keys` | `Vec<VisibleKey>` | required (Option accepts missing/null) |
| Timeline | `SlotAttachment` | tagged variant | — |
| Timeline.SlotAttachment | `slot` | `String` | required (Option accepts missing/null) |
| Timeline.SlotAttachment | `keys` | `Vec<AttachmentKey>` | required (Option accepts missing/null) |
| Timeline | `DrawOrder` | tagged variant | — |
| Timeline.DrawOrder | `keys` | `Vec<DrawOrderKey>` | required (Option accepts missing/null) |
| Timeline | `IkMix` | tagged variant | — |
| Timeline.IkMix | `constraint` | `String` | required (Option accepts missing/null) |
| Timeline.IkMix | `keys` | `Vec<ScalarKey>` | required (Option accepts missing/null) |
| Timeline | `IkBendDirection` | tagged variant | — |
| Timeline.IkBendDirection | `constraint` | `String` | required (Option accepts missing/null) |
| Timeline.IkBendDirection | `keys` | `Vec<ScalarKey>` | required (Option accepts missing/null) |
| Timeline | `IkSoftness` | tagged variant | — |
| Timeline.IkSoftness | `constraint` | `String` | required (Option accepts missing/null) |
| Timeline.IkSoftness | `keys` | `Vec<ScalarKey>` | required (Option accepts missing/null) |
| Timeline | `TransformConstraintMix` | tagged variant | — |
| Timeline.TransformConstraintMix | `constraint` | `String` | required (Option accepts missing/null) |
| Timeline.TransformConstraintMix | `keys` | `Vec<MixKey>` | required (Option accepts missing/null) |
| Timeline | `Deform` | tagged variant | — |
| Timeline.Deform | `slot` | `String` | required (Option accepts missing/null) |
| Timeline.Deform | `attachment` | `String` | required (Option accepts missing/null) |
| Timeline.Deform | `keys` | `Vec<DeformKey>` | required (Option accepts missing/null) |

| Timeline.PathPosition | `constraint` | `String` | required (Option accepts missing/null) |
| Timeline.PathPosition | `keys` | `Vec<ScalarKey>` | required (Option accepts missing/null) |

### `Interp` (enum)

Container attributes: `#[serde(tag = "curve", rename_all = "lowercase")]`.

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Interp | `Linear` | tagged variant | `#[default]` |
| Interp | `Stepped` | tagged variant | — |
| Interp | `Bezier` | tagged variant | — |
| Interp.Bezier | `handles` | `[f32; 4]` | required (Option accepts missing/null) |

### `ScalarKey` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| ScalarKey | `time` | `f32` | required (Option accepts missing/null) |
| ScalarKey | `value` | `f32` | required (Option accepts missing/null) |
| ScalarKey | `interp` | `Interp` | `#[serde(default, flatten)]` |

### `Vec2Key` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Vec2Key | `time` | `f32` | required (Option accepts missing/null) |
| Vec2Key | `x` | `f32` | required (Option accepts missing/null) |
| Vec2Key | `y` | `f32` | required (Option accepts missing/null) |
| Vec2Key | `interp` | `Interp` | `#[serde(default, flatten)]` |

### `ColorKey` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| ColorKey | `time` | `f32` | required (Option accepts missing/null) |
| ColorKey | `value` | `[f32; 4]` | required (Option accepts missing/null) |
| ColorKey | `interp` | `Interp` | `#[serde(default, flatten)]` |

### `Axis` (enum)

Container attributes: `#[serde(rename_all = "lowercase")]`.

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| Axis | `X` | tagged variant | `#[default]` |
| Axis | `Y` | tagged variant | — |

### `TransformMix` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| TransformMix | `rotate` | `f32` | `#[serde(default, skip_serializing_if = "is_zero")]` |
| TransformMix | `translate_x` | `f32` | `#[serde(default, skip_serializing_if = "is_zero")]` |
| TransformMix | `translate_y` | `f32` | `#[serde(default, skip_serializing_if = "is_zero")]` |
| TransformMix | `scale_x` | `f32` | `#[serde(default, skip_serializing_if = "is_zero")]` |
| TransformMix | `scale_y` | `f32` | `#[serde(default, skip_serializing_if = "is_zero")]` |
| TransformMix | `shear_x` | `f32` | `#[serde(default, skip_serializing_if = "is_zero")]` |
| TransformMix | `shear_y` | `f32` | `#[serde(default, skip_serializing_if = "is_zero")]` |

### `MixKey` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| MixKey | `time` | `f32` | required (Option accepts missing/null) |
| MixKey | `value` | `TransformMix` | `#[serde(flatten)]` |
| MixKey | `interp` | `Interp` | `#[serde(default, flatten)]` |
| MixKey | `extra` | `Extra` | `#[serde(flatten)]` `#[serde(default)]` |

### `VisibleKey` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| VisibleKey | `time` | `f32` | required (Option accepts missing/null) |
| VisibleKey | `value` | `bool` | required (Option accepts missing/null) |

### `AttachmentKey` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| AttachmentKey | `time` | `f32` | required (Option accepts missing/null) |
| AttachmentKey | `value` | `Option<String>` | required (Option accepts missing/null) |

### `DrawOrderKey` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| DrawOrderKey | `time` | `f32` | required (Option accepts missing/null) |
| DrawOrderKey | `offsets` | `Vec<(String, i32)>` | required (Option accepts missing/null) |

### `DeformKey` (struct)

| Scope | Field / variant | Rust serialization type | Serde attributes |
|---|---|---|---|
| DeformKey | `time` | `f32` | required (Option accepts missing/null) |
| DeformKey | `offsets` | `Vec<f32>` | required (Option accepts missing/null) |
| DeformKey | `interp` | `Interp` | `#[serde(default, flatten)]` |

<!-- schema-fields:end -->

## Names, references and validation boundaries

| Input | Actual resolution / validation |
|---|---|
| Bones, slots, assets, skins, constraints | Names on disk become session IDs in convert::from_schema; name maps are not a global uniqueness validator |
| Bone parent | Resolved in a second pass; unresolved names reported |
| Slot bone | Unknown bone reported and slot skipped |
| Setup draw order | Unresolved names reported; remaining slots appended |
| Default skin | Named resolution; first available skin fallback |
| Skin/attachment | Resolved by slot and name; linked meshes retain source references |
| Unknown constraint kind | Author load/from_json rejects it before conversion (T-813) |
| Unknown blend / sequence mode | Author load/from_json rejects unsupported strings; legacy empty defaults remain accepted |
| Missing fields / types / enum tags / fixed-array size | Serde errors, except stated defaults and optional members |
| Unknown project version | Rejected; schema deserialization happens first, so malformed future input may report a field error before version error |
| Geometry and numbers | No comprehensive semantic validator in the format loader: odd flattened tails may be discarded by chunks_exact, unresolved weight bones filtered, domain/index/finite/sorted checks are not universally enforced |

Do not use successful load alone as validation of an untrusted or malformed project.
Preflight and render/export checks have additional target-specific rules; they are not
schema migrations. Older paired-axis key arrays and legacy positional mix arrays mentioned
in some source comments are **not** migrated by the current reader. Numeric version labels
from the removed prototype must not be relabeled to bypass version checks.

## External assets, metadata and preservation

For canonical `hero.ankh`, `hero.ankh.json` and `hero.ankh.min.json`, bytes live under
sibling `hero.assets/`. Asset.file is relative to that directory, e.g. `8f/<sha256>.png`,
**without** the `hero.assets/` prefix. SHA-256 covers original encoded bytes; extensions are
sniffed PNG/JPG/WEBP/bin. Bytes are not embedded, decoded or re-encoded by project save.
The loader does not verify hash, image dimensions or image decodability. Identical existing
bytes are reused; conflicting existing bytes are an error, never overwritten. Missing images
report dangling asset references while retaining the rig. T-813 refuses file Save, Save As
and autosave while any indexed image has no bytes (including an unused library entry).
The original serialized URI is not retained in the editable image model, so refusing saves
avoids replacing it with an empty-content hash. Relink/reload the image through Assets,
or explicitly remove an unwanted entry, before retrying. Pure to_json/to_min_json helpers
are projections, not safe file publication. `source_path` is advisory and
machine specific; `psd_layer_paths` preserves reimport provenance; export_presets is opaque JSON.

Asset URIs reject absolute paths, drive/UNC prefixes, dot and parent components, including
portable separators. This is lexical reader confinement, **not** a canonical symlink sandbox.
The writer separately rejects symlink/reparse ancestors, reserved Windows device names,
ADS, invalid components, read-only replacement targets and unsuitable paths. Autosaves have
a distinct project filename/asset namespace; staged recovery uses the original asset origin.

| Unknown-data path | Actual guarantee |
|---|---|
| Raw compact contraction/expansion | Unknown object keys survive with escaped tags |
| Direct schema Serde round trip | Unknown members retained where a flattened Extra map exists; MixKey caveat below |
| Direct Serde on a struct without Extra | Unknown properties ignored; unknown enum discriminants rejected; author loading additionally rejects unknown members |
| Author load/from_json -> editable model | Unknown members are rejected before conversion, including members ignored by direct Serde; raw files remain intact |
| Opaque export_presets values | Nested unknown members preserved by loaded round trip |
| Known metadata / assets | Historical save/reopen tests compare values and original encoded bytes |

A concrete additional limitation: deserializing MixKey directly captures `curve` in both
flattened Interp and Extra, then serializing that schema value writes duplicate curve
members; direct typed reopening rejects it. A dedicated compiled case records this defect.
Loaded-model conversion drops Extra, so this is not a claim that ordinary editor Save emits
duplicate keys. This pre-existing direct-Serde limitation is not a claim of a duplicate field in native Save; it remains outside the T-813 repair.

T-813 guards both author reader entry points before conversion. It compares input members
with the actual Serde projection after clearing Extra maps, permits only scoped declared
skip_serializing_if defaults, and exempts opaque export_presets. This is fail-closed reading,
not opaque document preservation or a migration. Unsupported member errors identify a JSON
pointer (not its value). Low-level direct Serde and convert::from_schema are not equivalent
compatibility gates; foreign importers keep their existing conversion policy. Known-schema
structural/reference validation is still partial as documented above.

## Native runtime JSON (format version 2)

This is an evaluation/render projection, not an editable author project. All root members
below are emitted; optional nested fields are called out. The native preset and context are
the authoritative writer. Reader source qualification: external runtime checkpoint
`56dc0f10ca27806fcb88839f782a585438bf226a`; no new WebGL qualification is claimed by T-605.

| Object | Fields and types |
|---|---|
| Root | format:string=ankhimate-runtime; format_version:number=2; context_version:number=1; name:string; fps:number; bones:Bone[]; slots:Slot[]; drawOrder:string[]; defaultSkin:string or null; skins:Skin[]; constraints:Constraint[]; constraintOrder:string[]; animations:map<string,Animation>; atlas:Atlas or null |
| Bone | name:string; parent:integer (-1 root, otherwise prior bone index); length,x,y,rotation,scaleX,scaleY,shearX,shearY:number; inheritRotation,inheritScale,inheritReflect:boolean |
| Slot | name,bone:string; attachment:string or null; color:hex RGBA string; dark_color?:[r,g,b,intensity]; blend:string |
| Skin | name:string; bones,constraints:string[]; attachments:{slot:string,name:string,attachment:Attachment}[] |
| Animation | duration:number seconds; looping:boolean; bones:BoneTrack[]; slots:SlotTrack[]; deform:DeformTrack[]; drawOrder:DrawOrderKey[]; ik:IkTrack[]; transform:TransformTrack[]; events:Event[] |
| BoneTrack | name:string; rotate?,translate_x?,translate_y?,scale_x?,scale_y?,shear_x?,shear_y?:ScalarKey[] |
| ScalarKey | time,value:number; curve:linear or stepped string, or {type:bezier,handles:[number;4]}; has_next,is_bezier:boolean; points,line_points:number[] or null (context helper data also emitted) |
| SlotTrack | name:string; channel:string (color,visible,attachment); keys:array of time plus color:[number;4] / value:boolean / name:string or null; all keys carry curve (visible/attachment stepped) |
| DeformTrack | slot,attachment:string; keys:{time:number,offsets:number[],curve:Curve}[] |
| DrawOrderKey | time:number; offsets:{slot:string,offset:integer}[] |
| IkTrack | constraint,channel:string; keys:ScalarKey[] |
| TransformTrack | constraint:string; keys:{time:number,rotate,translate_x,translate_y,scale_x,scale_y,shear_x,shear_y,translate,scale,shear:number,curve:Curve}[] |
| Mix | rotate,translate_x,translate_y,scale_x,scale_y,shear_x,shear_y:number |
| Event | time:number; name:string; int:integer; float:number; string:string; audio:string or null; volume,balance:number |
| Atlas | pages:Page[]; regions:AtlasRegion[] |
| Page | index,width,height:integer; file:string |
| AtlasRegion | name:string; page,x,y,width,height,offset_x,offset_y,original_width,original_height:integer; rotated:boolean |

Bone sample offsets are baked into exported key times. Runtime v2 has independent axis
tracks; v1 paired translate/scale/shear and optional offsets are a reader compatibility
path, not current writer output. V1/v2 must not mix paired and scalar-axis representations.
Static dark_color carries exactly authored RGBA (fourth component is intensity in the
qualified renderer); it does not invent a dark-color animation channel.

| Attachment type | Additional fields (numeric unless annotated) |
|---|---|
| region | texture:string; x,y,rotation,scale_x,scale_y,width,height; uv:number[]; source_width,source_height; pivot_x,pivot_y; sequence:Sequence or null |
| mesh | texture:string; vertices,uvs:number[]; triangles,edges,edges_x2:integer[]; hull,vertex_count:integer; source_width,source_height,scaled_width,scaled_height; weights:WeightVertex[]; flat_vertices:number[]; weighted:boolean; linked:Linked or null; sequence:Sequence or null |
| clipping | vertices:number[]; vertex_count:integer; end_slot:string or null |
| path | vertices:number[]; vertex_count:integer; closed,constant_speed:boolean |
| bounding_box | vertices:number[]; vertex_count:integer; weights:WeightVertex[]; flat_vertices:number[]; weighted:boolean |
| point | x,y,rotation |

Sequence is {frames:string[],fps:number,mode:string,setup_index:integer}; allowed modes are
hold, once, loop, ping_pong, once_reverse, loop_reverse, ping_pong_reverse. WeightVertex is
{count:integer,bones:{bone:string,x:number,y:number,weight:number}[]}; Linked is
{skin:string or null,slot:string,attachment:string,inherit_deform:boolean}. Atlas is null when not baked.

Attachment details are projected by Context from the author fields; weighted influences
carry bone reference, weight and bind-space x/y. Consult `docs/formats/export-runtime.md`
for weight/FFD conventions and measured target limitations; native transport is not a
claim that every consumer renders every attachment. Constraint objects carry common name:string, type:string, target:string, bones:string[],
mix:number and not_last:boolean (context separator hint also emitted by the native preset).
Kind-specific members follow. Optional blocks are emitted only when present in author data.

| Constraint kind | Additional fields |
|---|---|
| ik | bend_direction,softness,stretch_limit,stiffness:number; stretch:boolean |
| transform | mixes?:Mix plus translate,scale,shear:number legacy aliases; drives?:boolean members rotate,translate,scale,shear,translate_x,translate_y,scale_x,scale_y,shear_x,shear_y,any; offsets?:{x,y,rotation,scale_x,scale_y,shear_x,shear_y:number}; local,relative:boolean |
| physics | physics?:{inertia,strength,damping,mass:number}; forces?:{wind_x,wind_y,gravity_x,gravity_y:number}; channels?:{rotate,translate:boolean} |
| path | slot?:string; path?:{position,spacing,mix_rotate,mix_translate:number} |


Constraints are transmitted without implying deterministic physics or complete path support.

The TypeScript parseRig reader checks the format discriminator, context version, supported
format versions, required root collections, unique bone/slot names, parent ordering and slot
bone references. V2 scalar-axis keys validate finite time/value, nondecreasing times and
Bezier X handles in [0,1]; duration is finite/nonnegative and dark_color is four finite
numbers. This is not exhaustive validation of all rotation keys, constraints or geometry.
Unknown members remain on its returned object but have no generic migration/write promise.

## Version changes, migration and rollback policy

1. Classify the change: author schema, envelope/codec, native runtime, export context,
   preset or recovery manifest. Do not bump an unrelated number.
2. Additive optional author fields require documented defaults, compatibility tests and an
   updated schema inventory. Removing/renaming/reinterpreting fields requires a reviewed
   new version and an explicit reader/migration policy. No version changes occur in T-605.
   The published 0.2.0-alpha.1 reader predates the T-813 unknown-member guard. Future
   meaningful additions must not rely on that old binary preserving unknown data:
   review an author-version bump/migration when older readers would discard it.
3. For a new author/envelope version, record an explicit format identity, authentic fixture,
   provenance and SHA-256 in `formats/tests/fixtures/format-versions.json`; add its specification
   anchor and reading/migration tests. Existing historical fixtures stay immutable. Normal
   Rust tests compare CURRENT_VERSION, envelope VERSION, manifest, fixture headers, logical
   payload version and spec anchor; changing a constant alone fails the gate.
4. Test conversion on copies: load old -> migrate in memory -> save to a new path with its
   assets -> reopen -> compare semantics/references/encoded assets. A serializer helper or a
   successful parse alone is not proof of lossless document migration.
5. Do not save unknown future versions in place. The present reader rejects them and has
   no migration dispatch. Do not treat the removed ZIP prototype as current author v1.
6. Obtain an authenticated v0 file, its producing revision and historical decoding contract
   before promising v0 support. Inspect it with that reader in isolation, establish expected
   model/assets, design a separately reviewed converter and retain the unmodified original.
   No such fixture was found in this audit, so T-605 remains partial on this criterion.
7. Native runtime changes require producer/consumer compatibility tests and appropriate
   format/context identifiers. Runtime v1 reading is already supported, not newly implemented
   or requalified by this documentation mission.

Safe save writes immutable assets first and publishes the project last. A sibling exclusive
lock and unique staging directory hold manifest, new payload and previous payload. Files
are synced and handles closed before renames. Ordinary publish failure restores the old
payload only if the destination remains vacant; incomplete rollback keeps recoverable stage
and backup and reports failure. Never overwrite an occupied recovery destination or backup.
Successful cleanup removes the publisher-owned temporary previous payload, **not** a promise
of permanent historical backup retention. Existing user backups are not garbage-collected.

There is no multi-file atomicity or proven power-loss durability: the project path can be
briefly absent between renames, directory fsync is not guaranteed, new unused assets can
remain after failure, and a crash can require recovery. Autosaves are separate copies and
are not erased by manual Save. See [project save safety](project-save-safety.md) for the
established failure/recovery protocol. Tests preserve original fixture hashes and sentinel
backups; they do not simulate every possible kernel/filesystem crash.

## Automated evidence and remaining qualification

`formats/tests/schema_contract.rs` checks the structured inventory, every documented field,
compact tag ordering and compiled Serde examples for every type and enum variant. `formats/tests/migrations.rs` runs
historical files through all three current profiles with asset/reference comparisons and
failure preservation (normalized model comparison with absolute numeric tolerance 0.0001,
exact names/references and encoded asset bytes). `export/tests/format_contract.rs` checks
actual native axes and Crapaud region exports; the source review gate also covers the native
projection/template. It is deliberately conservative and can require inventory review after
nonsemantic source changes. It is not a general Rust parser or a substitute for future
consumer interoperability tests.
The deliberately red version-gate scenario uses a temporary manifest, never a source version
bump. These tests run in the existing workspace test suite without an external service.

Known outstanding qualification: authentic v0 migration; removed prototype conversion;
end-to-end opaque preservation of unknown schema members (unsupported author input is now
refused by T-813). T-813 changes reader acceptance and pre-write validation only; versions,
wire fields, compact tags and supported valid writer bytes remain unchanged. Authoring access and advanced visual fidelity remain governed by their own
milestone evidence, not by these tables.


## T-813 pre-publication validation

Before any project/asset write, Save validates the typed JSON projection. A non-finite
f32 otherwise becomes null without a serde_json serialization error, even in the binary
profile built from that value tree. A non-reopenable typed projection is now an error
before publication, preserving the old project and assets. This is not comprehensive
geometry, finite-value or graph validation of every opaque/custom payload. Existing
safe-write staging, Windows path restrictions, rollback and recovery contracts still apply.

The T-605 authentic fixtures and CURRENT_VERSION/envelope/spec gates remain unchanged.
No v0 or prototype migration has been invented. See [project recovery](project-recovery.md)
and [save safety](project-save-safety.md) for the user workflow and crash limitations.

## T-831 tagged PathPosition addition

Author/envelope v1 is retained for the additive closed enum tag `path_position`,
with unique constraint name and existing scalar keys. Old projects are unchanged;
older closed-enum readers reject new files, never silently discard this track.
Three-profile frozen-reader and new-reader round trips qualify this bounded
addition. Runtime v2 is unchanged and data exports refuse the track. See the
[complete contract](path-position-animation.md) for references, units and limitations.

The T-831 Context source digest was reviewed for its exhaustive PathPosition arm.
No native-v2 field is added: `run::plan_with` and plugin planning reject the track
before building outputs. Existing native preset bytes and projection tables remain
unchanged; regression tests cover refusal and the static-project witness.
