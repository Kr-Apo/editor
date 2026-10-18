# Ankhimate spritesheets in Phaser

An independent Phaser 3 example using actual M004.2 exports of `walker` (`walk`,
`leg_ik`) and the M002 project authored from `tagged_rig.psd` (`animation_1`).
Each has 60 RGBA frames at 60 FPS, split across seven pages with a 2048 px page
limit, 512×512 frames and 2 px padding/extrusion. No exported data is patched.
The four individual PNG references per clip come unchanged from M004.1.

## Run

Install Node.js 22 or later, then from this folder:

```sh
npm ci
npm start
```

Open `http://127.0.0.1:4174` in a browser with WebGL. Choose a clip, play/pause,
or compare at 0, 0.25, 0.5 and 0.75 seconds. Comparison shows the spritesheet
frame at left and the corresponding individual PNG at right. The checkerboard
is outside the transparent WebGL canvas. Playback follows the exported loop
intent; all three included clips loop. Source image size is preserved at 1:1.
This demonstration's canvas/comparison positions target its included 512×512
frames; adapt viewport/placement for differently sized exports.

## Integration

The full rendered-output contract is in
[`docs/spritesheets.md`](../../docs/spritesheets.md). This JSON is not Phaser's
built-in atlas schema. `src/metadata.js` validates spritesheet version 1, sorts
by explicit indices and refuses unsupported layouts/timing. `src/demo.js`:

1. Loads each `pages[].image` as an ordinary Phaser texture.
2. Registers `frames[].rect` with `texture.add(name, 0, x, y, w, h)` on its page.
3. Creates an animation from explicitly indexed `{ key: pageTextureKey, frame:
   frameName }` records at `metadata.fps`, with repeat determined by `looping`.
4. Keeps origin at the common exported pivot and sprite position fixed.

`duration_ms` is checked against `1000/fps`; this consumer intentionally refuses
variable frame durations instead of guessing how to map them to Phaser's
additional-frame-duration field. It also refuses trim, rotation and unknown
format versions. There is no skeletal runtime, mesh evaluator, animation mixer,
physics or dark-tint compatibility claim in this example. Clip selection starts
a fresh animation; blending between clips is not provided.

To use your own export, copy its JSON/pages into an asset folder, add an entry to
`assets/manifest.json`, and update or omit the optional PNG comparison references.
The source files remain portable and need no editor checkout or Ankhimate runtime.
When removing comparison references, remove their comparison buttons or supply
your own reference frames; the automated acceptance test targets the three
included fixtures. No remote service or CDN is used.

## Tests and versions

```sh
npm run check
npx playwright install chromium
npm test
```

`npm run check` tests metadata validation, shuffled explicit indices and all
recorded export/reference hashes. `npm test` additionally opens real WebGL,
observes three loops and all 60 frame changes for each clip, checks animation
duration/rate/position/origin and compares 12 rendered poses against PNGs.
Comparison tolerance is 2/255 per RGBA component; the validation host measured
zero error. Real-clock repeat intervals must be within 200 ms of one second;
this is a scheduler tolerance, not a claim about exact display refresh.

Test output/captures go to ignored `proofs/`, or `ANKHIMATE_PROOFS`. Tests use the
bundled full Chromium channel; `ANKHIMATE_CHROMIUM` may select an existing browser
executable. The actual browser version and GPU are recorded in the results.

Validated on Windows with Node 24.18.0, Phaser **3.90.0**, Playwright **1.58.2**,
and existing Chromium **154.0.8037.0**, explicitly selected through that override.
WebGL backend: ANGLE / Direct3D11, NVIDIA RTX 3090. The Playwright-default browser
download is not the browser used for these recorded results. Package versions
are pinned in `package-lock.json`; browser provisioning is a separate test step.
No `@ankhimate/*` package or dependency on the external runtime is present.

`assets/provenance.json` records the input project hashes, export settings and
SHA256 of each shipped export/reference. `.gitattributes` preserves these bytes
on Windows checkouts. The PNGs inherit the repository's own generated sample and
test fixture provenance; no third-party rig is included.
