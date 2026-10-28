> **P01-RC1 local candidate — 28 October 2026.** Read the [current contracts and qualification boundary](p01-rc1-workflow.md) before the historical notes below. Proposed Classic v0.1.6 is not tagged/released here. Runtime package versions are unchanged; ReplayV1 A17 is explicit and separate from Legacy playback.

# Getting Started — v0.1.5 Local Candidate (T838)

1. **Extract the ZIP completely** into a normal folder you can write to. Double-click **Ankhimate.exe**. No launcher script is required. This executable is unsigned, and this candidate is not a new public release.
2. On the welcome screen, or through **File > Open…**, open `Examples/Jade-Sentinel/Sentinelle.ankh` from the extracted folder. There is no dedicated “Open Jade” button. Keep the `Sentinelle.assets` folder alongside the project: it contains five required PNG images.
3. Explore the **bones** and **slots** in the hierarchy, then switch to **Animate** and choose a clip in the animation manager or Timeline. `idle` lasts 2 seconds; `attack` lasts 1.5 seconds and has an impact at 0.75 seconds. Try playback and move the playhead.
4. Before editing, choose **File > Save As…** (**Ctrl+Shift+S**) and save under a new name or in a new folder. Preserve the original example. Check whether you are in Setup or Animate mode before making changes.
5. Make a small edit, try **Undo/Redo**, then choose **Save** (**Ctrl+S**). Reopen your saved copy to check both the project and its assets. If Save fails, do not proceed with a workflow that closes or replaces the current document until your work is safe.
6. In the **Export** panel, use **Data / atlas** and the native preset for a compatible game runtime, or choose **Image sequence / Spritesheet** for rendered frames. Select a new output folder and read the preflight results. CPU exports use the default skin and Normal blending, without physics. MP4/GIF require an external FFmpeg installation.

## Recovery copies

An old recovery copy may trigger a **Recover unsaved work?** dialog. **Recover** loads the copy without overwriting its original project; use **Save As** afterwards. **Ignore** remembers the decision for that specific recovery state without deleting any files. You can revisit ignored copies through **File > Review Recovery Copies…**. New or changed copies can be offered again. If the Ignore decision cannot be saved, an error is shown; **Later (this session)** postpones the dialog only for the current session. Do not delete project sidecars just to try this candidate.

For more information, see [Features](FEATURES.md), [Compatibility](COMPATIBILITY.md), and [Known Limitations](KNOWN-LIMITATIONS.md). Physical Windows testing of this new candidate is still pending maintainer review.

## Build identity

The welcome screen intentionally has no version suffix. Use **Help > About Ankhimate** for product **0.1.5**, local/development build status, source commit and license information. This is a new local candidate; none of the pending physical checks is automatically passed.
