# Static dark tint in native runtime v2 (M005.3a)

The current native template transmits static slot dark tint. This is **data
preservation, not visual Phaser support**. TypeScript runtime 0.1.1 at
`6d8f3454aacd6b69fbc3c242b54ba98e19c23f79` accepts the JSON, preserves the unknown
property in `parseRig`, and ignores it in evaluation/rendering. Its public slot
type does not yet declare or validate this property. Export requires explicit
fidelity consent (`dark-tint-runtime-unapplied`). M005.3b has not been implemented.

## Wire contract

```json
{
  "format": "ankhimate-runtime",
  "format_version": 2,
  "context_version": 1,
  "slots": [
    {
      "name": "body",
      "bone": "root",
      "attachment": "body",
      "color": "ffffffff",
      "dark_color": [0.25, 0.5, 1.0, 0.5],
      "blend": "normal"
    }
  ]
}
```

This excerpt omits the rest of the rig. `dark_color`, when present, is exactly
four finite JSON numbers in **R, G, B, intensity** order. The source is
`Slot.dark_color: Option<[f32; 4]>`. The regular serde JSON helper preserves those
f32 values (widened to f64 in the template context); it does not hex-encode,
round, clamp, or premultiply them. For example, authored f32 `0.2` can appear as
`0.20000000298023224`. Consumers must retain that value rather than interpreting
the extra digits as an export error.

The ordinary authoring range is 0–1. Finite values outside that range are
preserved too: the existing model has no range restriction. CPU output is
clamped when converted to PNG bytes. NaN and infinities are refused by
preflight; the updated built-in native planner also refuses them **before**
context conversion could turn them into JSON nulls. A malformed authoring array
(wrong length, strings or null channels) cannot deserialize as `[f32; 4]`.

- `None`: omit the property entirely; no new null/default is written.
- `Some([0,0,0,0])`: write all four zeros, including zero intensity.
- Missing property in older v1/v2 exports: dark contribution disabled.
- Existing light `color` hex, blend, channels, format/context/preset versions and
  all other meanings are unchanged. There is no new dark animation track.
- No-dark exports are byte-identical to the prior native template, including
  atlas bytes for unchanged inputs/settings.

Existing saved presets contain their own template bodies. They are **not
automatically rewritten**. The exact pre-M005.3a native template is identified
as `LegacyRuntime`, retains its original output and reports `dark-tint-loss`
when relevant. Add the current **Ankhimate runtime** built-in preset to transmit
the value. A changed/custom body remains unqualified; a matching display name
does not establish capability. Generic JSON, Phaser atlas, PNG sequence and
Spritesheet behavior remain unchanged.

## Existing Rust evaluation and CPU rendering

The complete path is:

`core::slot::Slot` ↔ authoring `schema::Slot` (`formats::convert`) →
`Context.skeleton.slots[].dark_color` → native Handlebars template → optional
runtime JSON property. Only the template was missing the value.

`pose::evaluate` copies the static setup value into `Pose.slot_dark_colors`.
`Timeline::SlotColor` animates the **light** RGBA only. CPU region and mesh
vertices carry both colors. For a sampled straight-alpha texel `T`, light color
`L` and dark value `D`, the existing rasterizer computes:

```text
source.rgb = T.rgb * L.rgb + (1 - T.rgb) * D.rgb * D.a
source.a   = T.a * L.a
```

Normal composition then applies `source.a` over the destination; PNG RGB is
stored with straight alpha. Dark intensity is independent of light opacity and
does not make transparent texels opaque. This does not claim every other
two-color-tint convention is equivalent. PNG/spritesheet still restrict blending
to Normal, physics is refused, and only the default skin is baked. Their existing
dark-tint qualification warning remains.

## Proposed M005.3b consumer work (not implemented here)

1. Add optional `dark_color?: [number, number, number, number]` to the public
   runtime slot contract; validate four finite numbers when present and preserve
   old exports with the field absent. Do not reinterpret light color/alpha.
2. Carry static dark RGBA through evaluation and region/mesh rendering, with
   absent dark tint contributing zero. Do not invent an animated dark channel.
3. Implement the equation above at the appropriate Phaser rendering layer;
   qualify texture filtering, straight/premultiplied alpha, light animation,
   regions/meshes and supported blend modes separately using real WebGL pixels.
4. Compare CPU/WebGL reference poses at 0, .25, .5 and .75, including intensity
   0/.5/1, RGB extremes and transparent borders; document actual tolerances.
5. Retire or narrow `dark-tint-runtime-unapplied` only after that qualification.

M005.3a does not change the runtime repository, renderer, animation evaluator,
staged publisher, project save/recovery guarantees or rendered-export formats.
