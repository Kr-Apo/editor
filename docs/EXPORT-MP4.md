# MP4 export from the Windows editor (T-810.5)

## Quick workflow

Open a **copy** of a project and select an animation. Choose **File → Export →
Video MP4…**. The Export pane opens with that clip, **1280 × 720**, **30 FPS**,
one content cycle and an opaque background. Choose 720p or 1080p, 24/30/60 FPS,
one/two/three cycles and the background colour. Aspect ratio is preserved by the
same common camera used for image sequences; the character is never stretched.

Click **Export MP4…**, choose a new `.mp4` filename, review the diagnostics and
explicitly accept any fidelity warnings before **Publish export**. Preparation,
frame encoding and container finalization run on the existing export worker.
Only **Complete** / the success message means the final file has been published.
The message includes its path. **Cancel export** cooperatively stops framing or
rendering between frames, terminates and waits for FFmpeg, and cleans owned
temporary files. One frame's CPU rendering cannot be interrupted internally.
New/Open/Close retain the existing worker cancellation/lifecycle protections.

This entry uses **libx264 / preset medium / CRF 23 / yuv420p**, without audio or
transparency. MP4 H.264 does **not** preserve RGBA alpha. The chosen background
is export-only and does not change the project. Existing Video-tab options
(GIF, custom dimensions/FPS, encoder and quality controls, opt-in opening) remain
available separately; T-810.5 adds no new codec or advanced encoding option.

**Existing files are refused, including files created concurrently.** Choose a
different name: this version offers no replacement operation, so it cannot
overwrite an existing video even if a native Save dialog offers confirmation.
No backup is replaced or deleted. The new basic entry limits cycles to three,
resolution to 1920 × 1080 and FPS to 60. The underlying existing pipeline also
limits each clip to 100,000 samples and a whole video to 1,000,000 frames.

## FFmpeg is a separate dependency

Ankhimate does not contain, bundle, download or install FFmpeg or an H.264 encoder.
Obtain a Windows FFmpeg build manually from the options linked by the
[official FFmpeg download page](https://ffmpeg.org/download.html). Follow that
distribution's installation instructions. Either put its executable directory
in `PATH` before launching Ankhimate, or select the actual **ffmpeg.exe** in
**Settings → Video**. A custom executable path never falls back to PATH.
Spaces, accented characters and Unicode are supported; shell scripts are refused
on Windows. Choose an executable you trust: it runs with your Windows account.

The panel reports whether an executable can be located; this is not a claim
that it can run or encode H.264. After explicit publication approval, the worker
checks `-encoders` for **libx264**, then runs the actual encoding. A build without
that encoder fails clearly, without choosing another codec. Install an appropriate
build yourself; no automatic download or fallback occurs. Paths and arguments
are passed separately to `std::process::Command`, without a shell.

FFmpeg and its optional encoders have their own licences and build configurations;
consult the chosen distribution's notices and the
[FFmpeg legal information](https://ffmpeg.org/legal.html). Ankhimate's licence
does not replace those notices. This delivery redistributes no FFmpeg files.

## Sampling, rendering and limitations

The unchanged `ankhimate_render::sequence::Sequence::prepare` fits one camera
over all sampled poses and renders each frame at absolute **t = i / FPS**.
For one clip, count is `ceil(duration × FPS)`, except representation noise near
an integer is rounded to that integer using the existing f32-relative tolerance.
Indices run from zero through count minus one, without a terminal duplicate.
For each repeat, the same clip indices are rendered again, without accumulating
real-time deltas. Non-looping clips also repeat when multiple cycles are chosen.
For fractional durations, encoded duration is `count / FPS` per cycle and may
exceed the author duration by less than a frame. Examples: 1 s × 24 FPS = 24
frames; 2 s × 30 FPS = 60; 1.01 s × 30 FPS = 31; 1 s × 60 FPS × 3 = 180.

The renderer evaluates bones, weighted meshes, FFD, attachment changes, draw
order, supported stateless constraints and its implemented tint/clipping
semantics. The existing **default skin**, **Normal blending** and **no physics**
restrictions remain. This is not a qualification of every editor/runtime feature
or of alternate skins. Unsupported cases and known fidelity limits retain
rendered-target preflight diagnostics and consent. Missing/invalid clips, assets
and invalid dimensions are refused. No evaluator or runtime format is changed.

Straight RGBA8 is flattened to RGB24 using the existing channel equation
`(source * alpha + background * (255 - alpha) + 127) / 255`. This is encoded-space
composition, not linear-light composition. H.264/yuv420p is lossy; pixels and
encoded bytes are not guaranteed identical across FFmpeg builds. There is no
audio or container-level repeat flag. Unlike a spritesheet, an MP4 contains an
encoded linear video stream; it does not provide frame rectangles for game logic.

Preparation stores decoded textures and one common camera. Encoding has at most
one RGB frame in flight plus the current rendering/conversion buffers; it does
not retain all frames. Asset, renderer and external codec memory remain additional.
The original document, selection, playback time, histories and assets are not
modified: the worker uses a cloned document focused on the chosen animation.

## Errors and publication

The worker drains process output continuously. Each captured diagnostic retains
at most a 256 KiB tail plus its truncation marker, and the UI shows an error
summary with expandable details. Detection times out after 10 seconds; a stalled
pipe or container finalization times out after 60 seconds. Ordinary cancellation,
launch/render/write/encoding errors kill and reap the child and report failures.
This is not a sandbox for a malicious executable that spawns descendants or
deliberately inherits pipes.

The encoder writes only into exclusively owned sibling staging. After successful
exit, a cancellation recheck, structural nonempty MP4 box checks and file flush,
publication creates a new hard link. No replacement or unsafe rename fallback
occurs. Choose local **NTFS**: FAT/exFAT and some network/virtual filesystems do
not support this publication method. Permissions, disk exhaustion, Windows file
locks and races cause explicit failures, preserving existing outputs.

Only owned temporary files are cleaned. Unexpected files, links or locks retain
staging with an explicit error; a cleanup failure after publication is reported
as an already-published video with cleanup failure. Forced termination, OS crash
or power loss can leave staging or a published file plus its temporary link.
Directory metadata durability and full crash atomicity are **not** guaranteed.
There is no new video crash-recovery journal. See the detailed, unchanged
[MP4 process/publication contract](mp4-export.md).

## Qualification status

Deterministic tests cover command routing/UI state without a native window,
sampling, CPU/PNG pixel agreement, compositing, process errors, cancellation,
bounded diagnostics and publication. A short real FFmpeg headless encode is
conditional on the existing local installation; its actual execution is recorded
in the delivery report. These checks are **not** manual video or artistic
qualification. The maintainer must perform the delivered A–F quickstart before
claiming that qualification. No GUI, browser, inference or GPU test is part of
T-810.5.
