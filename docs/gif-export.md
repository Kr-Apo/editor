# Animated GIF export (T-602.2)

Open **Export → Video → Animated GIF**, choose an animation, FPS, width/height,
explicit opaque RGB background and positive **Content cycles**. Select a new
`.gif` filename in an existing directory, review preflight, then publish. Existing
files always refuse, even if they appear after review. Progress and Cancel use the
same worker owner as [MP4](mp4-export.md); New/Open/Close cancellation and document
snapshot guarantees are retained. This creates no author edit in Setup or Animate.

## Selection and capabilities

MP4/GIF select by an in-memory animation ID; names are only labels. Each chooser
row includes its one-based document ordinal, so homonyms remain distinct. At
launch the selected animation alone is retained in a private document snapshot,
following T-601.3's existing principle. Its full timelines and source ID remain
intact there. Missing IDs fail rather than selecting a different clip. Legacy
headless callers can still use a unique name; ambiguous names refuse. Neither
IDs nor new fields are written to `.ankh`, runtime formats or image sidecars.

Detection and preferences are shared with MP4: an explicit executable path or
PATH, no shell, no download/bundling/linking, no console requested under Windows.
GIF probes `-encoders` for `gif` and `png`, and `-filters` for `palettegen` and
`paletteuse`, with the same supervised timeout/drainage. It does **not** require
libx264. Actual encode errors (including muxer/build limitations) still fail with
FFmpeg diagnostics. There is no codec/filter fallback. Missing capabilities point
to [official downloads](https://ffmpeg.org/download.html). The user's external
build and its licence remain their responsibility; no external licence is bundled.

## Rendering, palette and memory

The unchanged CPU `Sequence::prepare` supplies a constant camera and deterministic
samples. Default skin, Normal blending and existing stateless constraints only;
physics refuses. No trim, crop, rescale, renderer or evaluator change. GIF supports
odd dimensions; the panel permits 2..8192 per axis, as in the existing video pane.
Sequence validation also checks source parameters. GIF FPS is 1..100: higher
rates cannot keep one positive centisecond delay per source sample and refuse
instead of silently dropping frames. Total content is capped at 1,000,000 samples.

Each content cycle samples `i/fps`, `i=0..Sequence.frame_count()-1`, without an
extra terminal frame. Repeating a non-looping clip is also explicit content
repetition. Non-integral author duration × FPS keeps the existing rounded-up
sequence count; sampled duration per cycle is count/FPS. Both passes render these
same immutable poses in exactly the same order.

All RGBA, including partial alpha, is first composited onto the chosen opaque
background using the MP4 integer RGB rule. No binary transparency conversion is
performed. GIF has at most 256 palette colours; quantization and dithering can
change pixels. No transparency or PNG pixel equality is promised.

Pass A streams RGB24 to `palettegen=max_colors=256:reserve_transparent=0:stats_mode=full`.
The palette accounts for the entire animation, not just frame zero. Only one
private 16×16 PNG is stored; its dimensions and opacity are checked before pass B.
Pass B re-renders and streams all samples to `paletteuse`, using that palette.
No image sequence is stored and the application never accumulates all RGBA frames.
One frame is in flight, plus renderer/conversion memory. FFmpeg's own histogram,
filter and codec memory is external; no general memory/performance budget is
claimed for arbitrary projects/builds.

Exact vectors below use separate `Command` arguments; paths never enter a shell
or the filter expression. Common input:

```text
-hide_banner -loglevel warning -nostdin -n
-f rawvideo -pixel_format rgb24 -video_size WIDTHxHEIGHT -framerate FPS -i pipe:0
```

Pass A appends:

```text
-an -vf palettegen=max_colors=256:reserve_transparent=0:stats_mode=full
-frames:v 1 -c:v png -threads 1 -update 1 -f image2 OWNED_STAGE/palette.png
```

Pass B appends:

```text
-i OWNED_STAGE/palette.png
-filter_complex [0:v][1:v]paletteuse=dither=sierra2_4a:diff_mode=rectangle
-an -c:v gif -gifflags 0 -fps_mode passthrough -frames:v TOTAL
-loop -1 -final_delay LAST_CS -f gif OWNED_STAGE/encoded.gif
```

`-gifflags 0` disables delta transparency/rectangle encoding optimizations;
full opaque output frames are validated. `diff_mode=rectangle` controls dither
recomputation, not transparent GIF rectangles. No CRF/H.264 options appear in GIF
mode; MP4 options are unchanged. No byte determinism across FFmpeg builds is promised.

## Time and repetition

**Content cycles** multiply the encoded samples. **Container repeat** is fixed
to play the encoded content **once**, with `-loop -1`; no Netscape loop extension
is written. Two 1 s cycles mean approximately 2 s of content played once, not an
infinitely looping GIF. T-602.3 adds opt-in Windows **Open after export**, off
by default, with the [shared post-publication contract](mp4-export.md#open-after-export-mp4-and-gif).
It does not change GIF encoding, palette arguments or timing.

GIF delays are integer centiseconds. Source FPS is not an exact fixed-delay tag.
With the qualified FFmpeg 8.1.2 build, timestamps round to the nearest centisecond:
`round(i*100/fps)`. Consecutive rounded boundaries determine delays. The final
frame delay is supplied explicitly as `round(N*100/fps)-round((N-1)*100/fps)` so
the final interval is not guessed by the muxer. Half boundaries use half-up rounding.
For 60 samples at 30 FPS this produces 40 delays of 3 cs and 20 of 4 cs: total
200 cs, maximum timestamp error 1/3 cs. This is variable-delay timing, not an exact
33.333 ms delay for every frame. Total duration is checked within 1 cs of the
rounded sample duration before publication.

Some readers clamp short delays, especially below 2 cs. Above 50 FPS the preflight
requires explicit acceptance of this playback warning. Browser/player timing is
not universally qualified, even if the file's own timing is valid. Tests use
actual decoding and parsed delays, not screenshots. Different FFmpeg builds or
GIF readers may differ; unexpected counts/structure fail instead of announcing
partial content as complete.

## Supervision and publication

No duplicate process supervisor: both passes use the MP4 input pump, continuous
stderr reader, bounded log tail (256 KiB with explicit truncation), child status,
kill/wait/reap guard and thread joins. Probe timeout is 10 s per query; blocked
stdin or finalization timeout is 60 s per phase. A genuinely slow process can hit
that limit. Cancellation is checked during each pass, after palette validation
and before final publication; CPU work is cooperative between poses/frames.
The UI stays responsive. External executables creating hostile descendants and
abrupt application/OS termination remain outside ordinary shutdown guarantees.

The existing new-file publisher now accepts an explicit allowlist of two owned
names: `encoded.gif` and `palette.png`. Both are confined to an exclusively
created sibling staging directory. All directory entries are inspected before
cleanup; unknown entries, links or backups cause retention and a reported path.
Only declared ordinary files and the empty owned directory may be removed. MP4
still declares only `encoded.mp4` and uses the same publication behavior.

Publish only after both processes succeed, the palette is validated and the GIF
has a logical screen, colour tables, complete image/control/data blocks, trailer,
expected dimensions and frame count, positive delays, opaque frames, no repeat
extension and expected total duration. This bounded structural reader is **not**
a full LZW decoder. Separate integration tests decode every frame with FFmpeg;
`GIF89a` alone is never considered sufficient.

Publication remains a flushed, new hard link, without replacement or an unsafe
fallback. NTFS is qualified; FAT/exFAT or other filesystems lacking hard links
fail explicitly. Existing exports/backups remain untouched. If publication succeeds
but cleanup fails, report both the published result and retained staging path.
No crash durability, directory-fsync guarantee, automatic video recovery or
isolation from unrelated filesystem writers is claimed. A power loss/forced kill
can leave staging or the temporary link of a published file. No alpha ZIP is rebuilt.

T-602.3 completes the MP4 encoder/quality controls and the shared opening option;
GIF still exposes neither CRF/bitrate nor MP4 encoder settings. References:
[palette filters](https://ffmpeg.org/ffmpeg-filters.html#palettegen) and
[GIF muxer](https://ffmpeg.org/ffmpeg-formats.html#gif).
