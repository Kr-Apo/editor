# MP4 export (T-602.1 / T-602.3)

Choose **Export → Video → MP4**. Select a clip, FPS (1..240), even width/height
(2..8192), an explicit opaque RGB background (white initially), CRF (0..51;
23 initially; lower means higher quality), or **Target bitrate** in kbit/s,
and a positive integer cycle count. Encoder defaults to **libx264**. The optional
**Open after export** is off initially and available on Windows only.
The complete output is limited to 1,000,000 frames. Pick an unused `.mp4` file
in an existing directory, inspect the preflight and explicitly accept any
fidelity/qualification warnings before **Publish export**. Existing filenames
are always refused, including files appearing after review. Cancel is available
while the worker checks FFmpeg, prepares framing or encodes. No document edit
occurs in Setup or Animate: selection, dirty state, history and the original
project/assets remain unchanged. Changing/opening a project cancels its worker;
normal application close waits for cancellation to unwind.

## External executable

**Settings → Video** stores an optional executable path in the editor's own
preferences. Empty means search the current PATH directories for `ffmpeg.exe`
(Windows) / `ffmpeg` (other platforms) at export time. A custom path is resolved
as a file, never as a command line and never falls back to PATH. Windows paths
must resolve to `.exe`; scripts are refused to avoid implicit shell invocation.
Spaces and Unicode are supported. No shell, command string expansion or automatic
installation is used. Windows children use `CREATE_NO_WINDOW`.

If missing, unusable or lacking the explicitly selected encoder, the worker reports the cause and links
to [official FFmpeg downloads](https://ffmpeg.org/download.html). No binary or
library is bundled, downloaded, installed or linked. The application and other
exports work without FFmpeg. The user's FFmpeg build determines available codecs
and its own licence; libx264-enabled builds commonly include GPL components.
No licence for the external executable is supplied by Ankhimate. Redistribution
or codec/patent requirements depend on that build and usage; no packaging changes
are part of this task.

## Samples, pixels and encoder

Reuses `ankhimate_render::sequence::Sequence::prepare` unchanged: one camera fitted
to all poses, default skin, Normal blending, existing stateless constraints and
CPU rendering. Physics constraints are refused. Existing rendered-target diagnostics
and explicit fidelity consent apply. No renderer, evaluator, author/runtime schema,
PNG/JPEG/WebP, spritesheet or batch implementation is changed.

For each cycle render exactly `sequence.frame_count()` samples at `i / fps`,
`i=0..count-1`; repeat the same indices, without a terminal frame. This also
repeats non-looping clips when more than one cycle is requested. A 1 s clip at
30 FPS and two cycles yields 60 frames / 2 s. As in image sequences, non-integral
`duration*fps` rounds up (apart from f32 boundary noise); encoded cycle duration
is the sampled frame count / FPS and can exceed author duration by less than one
frame. MP4 does not export the editor's looping intent as a player repeat flag.

RGBA8 straight alpha is flattened onto the chosen background, per encoded channel:
`(source*alpha + background*(255-alpha) + 127) / 255`. This is encoded-space
composition, not linear-light composition. Only RGB24 bytes are streamed to stdin.
No intermediate encoded images are written. No trim, padding, rescale or crop.

The exact argument vector, with each item passed separately to `Command`, is:

```text
-hide_banner -loglevel warning -nostdin -n
-f rawvideo -pixel_format rgb24 -video_size WIDTHxHEIGHT -framerate FPS -i pipe:0
-an -c:v libx264 -preset medium -crf CRF -pix_fmt yuv420p -frames:v TOTAL
-movflags +faststart -f mp4 OWNED_TEMPORARY/encoded.mp4
```

The default profile is exactly `libx264`, preset `medium`, CRF 23, yuv420p; its
argument vector and qualified reference bytes remain unchanged. MP4 dimensions
must be even (also retained for RGB for a consistent MP4 contract); invalid
settings fail before spawning an encoder. No audio track or codec fallback.
Selected encoder availability is probed using
`-hide_banner -encoders`, then the actual encode must also succeed. MP4/container
metadata carries dimensions, FPS, count/duration; there is no new sidecar or
runtime format. Lossy H.264/chroma subsampling can change pixels; no identical-byte
promise across FFmpeg versions/builds/hardware.

See FFmpeg's [rawvideo demuxer](https://ffmpeg.org/ffmpeg-formats.html#rawvideo)
and [libx264 encoder](https://ffmpeg.org/ffmpeg-codecs.html#libx264_002c-libx264rgb).

## Process ownership and bounded work

All probing, framing, rendering and encoding run on the existing rendered-export
worker owner. A dedicated input pump writes one RGB frame at a time; the worker
waits for its acknowledgement while polling cancellation and child exit every
10 ms. CPU rendering is cooperative between frames, not interruptible inside one
frame. At most one frame is in flight, plus the current RGBA/RGB conversion and
renderer/asset memory; memory does not scale with clip length or cycles. Child
codec/OS buffer memory is external and not bounded by this policy.

Probe stdout and stderr are continuously drained on separate readers. During
encoding, stdout is null and stderr is continuously drained. Each captured log is
a tail of at most 256 KiB (plus a read chunk transiently); an explicit truncation
marker is prepended when data was dropped. Non-UTF-8 diagnostics use replacement
characters. Errors include actual process status and the captured diagnostic.
Successful progress counts RGB frames accepted by the pipe, not finalized frames:
only the final **Complete** indicates publication succeeded.

Dependency detection has a 10 s timeout. A pipe write with no acknowledgement or
finalization after EOF with no exit for 60 s is treated as stalled. Slow but valid
external encoders may therefore need a future configurable timeout. Cancellation,
ordinary render/encode errors and timeouts close the queue, kill the child if
necessary, wait/reap it, join input/log threads, then clean up staging. The child
also has a kill/wait ownership guard. No detached ordinary FFmpeg child is left
behind. This is not a sandbox for arbitrary user-supplied executables that create
descendants or deliberately inherit pipes. Abrupt process termination is outside
ordinary cancellation guarantees. Opening uses a separate post-publication step
described below; it does not alter FFmpeg process supervision.

## Publication guarantees and limits

Paths reuse M001 portable-name and ancestor/reparse checks. Preparation creates
an exclusively owned sibling `.ankhimate-video-PID-counter` directory on the same
filesystem. FFmpeg writes only its new `encoded.mp4` there with no-overwrite mode.
After rendering, EOF, exit 0 and cancellation recheck, the output must contain
structurally bounded nonempty ISO-BMFF `ftyp`, `moov` and `mdat` boxes. These are
basic checks, not a full H.264 decoder; real integration tests separately decode
and count frames. The file is flushed using a writable handle (required by Windows).

Publication uses `hard_link(staged, new_destination)`: one new directory entry,
never overwrite. The destination/ancestors are rechecked immediately beforehand;
a concurrent file creation still makes the link fail without replacing it.
Filesystems without hard links (e.g. FAT/exFAT and some virtual/network stores)
fail explicitly; choose a supported local filesystem such as NTFS. No unsafe
rename/overwrite fallback or backup replacement is used. Existing exports and
backups are untouched.

Only the owned encoded file and empty owned directory are removed afterwards.
Unexpected entries, links, backups or Windows locks prevent safe cleanup: the
operation reports the retained path instead of recursively deleting it. If linking
succeeded but cleanup fails, the error explicitly states that the video was
published and staging was retained; it does not pretend nothing happened.

This is not full atomicity/durability. Publication makes one new filename visible,
but directory metadata is not synchronously committed. Power loss/OS crash/forced
termination can leave staging, or a published file plus its temporary link. There
is no automatic crash cleanup or video recovery journal. Uncooperative filesystem
writers can change topology/permissions between checks; no handle-relative sandbox
or isolation is claimed. Permission errors, disk exhaustion, Windows sharing
violations and unsupported filesystems remain ordinary reported failures.

T-602.2 also adds [Animated GIF](gif-export.md) using this same process owner.
The default libx264/CRF/yuv420p arguments above remain unchanged.
Both formats now select clips by session animation ID and isolate the selected
clip on a private snapshot, as in T-601.3. The chooser labels rows with their
one-based source ordinal to distinguish identical names. Missing/stale IDs fail;
legacy name-only API calls work only when exactly one clip has that name.
IDs are not added to author/export formats.

## Quality modes and qualified encoder choices (T-602.3)

- **CRF**: 0..51, initially 23. Passes `-crf VALUE`, never `-b:v`.
- **Target bitrate**: integer **64..100000 kbit/s**, initially 2000 when selected.
  Replaces `-crf VALUE` with `-b:v VALUEk`. This is x264 single-pass average bitrate
  targeting, not a constant bitrate, hard cap, VBV budget, or two-pass guarantee.
  Short/simple clips can differ substantially from the requested rate. CRF's
  inactive value is ignored, and vice versa. Preflight reports mode and value.
- **libx264**: software H.264, RGB24 input converted to yuv420p by FFmpeg;
  preset medium. This remains the default, with broader player compatibility.
- **libx264rgb**: software H.264 RGB, RGB24 input/output request, preset medium,
  same two quality modes. The tested bitstream decodes to gbrp / High 4:4:4
  Predictive. No chroma-subsampling conversion is requested. This is not a new
  container or a GPU encoder. Many browsers, TVs and hardware players cannot
  decode this profile; preflight requires explicit acceptance of that warning.
  Only FFmpeg software decoding is qualified. This is not a lossless guarantee
  at CRF 23. No silent fallback to libx264 occurs if RGB support is absent/fails.

Qualification on Windows x64, FFmpeg 8.1.2 full_build (Gyan), real walker walk,
2 cycles / 60 frames / 256×256 / 30 FPS / 2 s, background RGB [220,230,240]:

| Profile | Bytes | Video bit/s measured | MAE/channel | P99 error | PSNR dB |
|---|---:|---:|---:|---:|---:|
| libx264 target 64 kbit/s | 13282 | 46852 | 2.36108 | 36 | 30.4309 |
| libx264 target 2000 kbit/s | 201699 | 800520 | 0.79813 | 11 | 40.7128 |
| libx264rgb CRF 23 | 75719 | 296548 | 0.35074 | 8 | 43.1001 |
| libx264rgb target 2000 kbit/s | 279500 | 1111676 | 0.04836 | 1 | 57.1111 |

Errors compare every decoded RGB channel with the unchanged CPU rendering and
integer background composite. The retained integration thresholds are MAE ≤8
and P99 ≤40, on these fixtures only. Output size/quality is not predicted for
other projects/builds. The default MP4 remains 37123 bytes, SHA-256
`09dff320e35f665c07de0741c3063dcbd7006e215e5f41ddcde5c7ceabad923a`.

## Open after export (MP4 and GIF)

Windows only; **off by default**, session setting. After the worker has completed
encoding, validation, publication and owned staging cleanup successfully, it checks
cancellation, then makes **one** association-open request for the published absolute
path. No request on failed/cancelled/refused publication, including cleanup errors
reported after publication. UI polling never relaunches the file. Controls remain
disabled until the worker completes. Once hand-off starts the progress says that
the file is published and opening is in progress; it cannot be withdrawn by Cancel.

The native `ShellExecuteExW` API receives a UTF-16 filename and the `open` verb,
with null parameters; no command shell/string, URL, environment expansion or
codec/player installation. COM is initialized on the worker, `NOASYNC` keeps the
request alive until hand-off, and `FLAG_NO_UI` asks Windows to return errors
instead of its own error dialog. Spaces and Unicode remain literal path data.
The application associated with the extension is selected by Windows, not by
Ankhimate. Other platforms hide the option until a safe mechanism is implemented.

An opening error is shown **separately** from the successful export result and
never deletes or invalidates the published video. An accepted OS request means
hand-off, not proof of playback, decoding support or completion in the reader.
A slow association handler may delay the worker's completion; the editor UI remains
responsive. New/Open/Close cancellation before hand-off skips opening; a request
already handed to the OS cannot be recalled. External readers are not FFmpeg
children owned/reaped by the export supervisor.

Automated tests inject an opener and assert once-only post-publication ordering,
Unicode/special paths, separate opening errors and zero requests after errors,
collisions/cancellation. Native reader launch is deliberately forbidden in tests;
Windows API code is build-checked, not a claim that a specific reader was launched.
See Microsoft's [ShellExecuteExW](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecuteexw)
and [SHELLEXECUTEINFOW](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shellexecuteinfow).

T-602's functional scope is complete with these explicit contracts. Physics,
viewport fidelity and performance budgets of T-601 remain separate. No alpha ZIP
is rebuilt, and no arbitrary encoder list, audio, new codec family or platform
qualification is implied.
