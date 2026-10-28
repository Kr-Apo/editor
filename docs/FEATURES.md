> **P01-RC1 local candidate — 28 October 2026.** Read the [current contracts and qualification boundary](p01-rc1-workflow.md) before the historical notes below. Proposed Classic v0.1.6 is not tagged/released here. Runtime package versions are unchanged; ReplayV1 A17 is explicit and separate from Legacy playback.

# Ankhimate — Available Features

**Historical T838 v0.1.5 feature baseline.** Ankhimate — Kr-Apo Community Fork (Unofficial) is a native Windows application for 2D skeletal animation, derived from Ankhimate/editor. Upstream licenses and credits still apply. The application's interface uses English labels.

A character combines images, **bones**, **slots**, and **attachments**; animation clips bring those elements to life in an `.ankh` project. Jade Sentinel is the only user-facing example bundled with this candidate. This build is different from the historical v0.2.0-alpha.1 and the RC2 archive: the product numbering reset is independent of project/runtime format versions and does not make this candidate a release.

## Importing and managing images

- Import PNG, JPEG, or WebP files into the **Assets** library, then assign them to attachments. Source image pixels are stored as assets separate from the project file.
- **File > Import** opens registered importers. PSD import shares a preview across the File menu, Assets panel, and PSD drag-and-drop workflows. Choose layers, **Scale**, **Skip hidden layers**, replacement or merge before confirming. Cancel does not replace the document.
- PSD groups become bones, while image layers become slots/regions. Nested groups are traversed without duplicating descendants. The `$pivot`, `$ik `, and `@skin:` tags have specifically defined uses; they do not imply full reproduction of every Photoshop effect.
- **Update existing PSD images (keep rig)** preserves the rig and animations within its supported compatibility scope. Changes to layer paths require explicit mapping. Ambiguous cases, topology replacements, and unrepresentable FFD migrations are refused rather than guessed.
- Use image relink/reload tools when assets are missing and inspect the diagnostics. Saving may refuse to publish a project when required image data is absent.
- Texture atlases group images for export. Third-party formats depend on which plugins are actually installed; not all are bundled in this Windows package.

## Building a rig in Setup mode

Create and name **bones**, edit their hierarchy and transforms, then associate **slots** with their **attachments**. Region attachments have position, size, pivot, and image properties. Meshes, paths, bounding boxes, and clipping attachments have their own tools: a mesh is not simply a region.

The inspector supports numeric editing of transforms and geometry. For selected vertices, **Set/Add**, **Apply coordinates**, and **Apply UV coordinates** validate the entered values before committing an Undoable command. UVs use source-texture coordinates, with U increasing to the right and V downward; Setup positions are local to the bone, with Y upward. Non-finite values and UVs outside the accepted range are rejected. Expressions such as `120/2` are not supported as a calculator.

Named **skins** provide variations of attachments and images, with fallback to the default skin. Image assignment is independent per skin in representable cases. Skin composition in the editor does not guarantee identical arbitrary composition in a game runtime.

The Rust engine includes **IK** (one target, chains of one, two, or multiple bones), **Transform**, and **Path** constraints. Constraint parameters and evaluation order matter. Paths use polylines; physics has a simulation state. Not all these tools are supported by the distributed game runtimes, so check your target before building a rig that depends on them.

Clipping uses a mask attachment, an end slot, and draw order. It is not just an image overlay. Its availability in the editor does not certify every possible mask, overlap, or rendering mode.

## Editing meshes and weights

Create a mesh from a compatible region, edit its vertices, triangulation, and UVs, then use weight tools to deform it with multiple bones. The **Weights** panel offers Add/Subtract/Replace/Smooth, direct numeric input, radius, feather, and session locks. Influences and inverse binds must remain consistent with the bind pose.

Retracing a weighted mesh is a protected operation. Compatible owned meshes without FFD or ambiguous sharing can be prepared and validated before Apply. Linked meshes, shared ownership, topology constraints, or ambiguous FFD references may cause the operation to be refused. There is no universal migration of weights or FFD data to arbitrary new topology.

In **Animate**, moving vertices creates or edits an FFD deformation according to the native edit routing. A completed drag is a single Undoable command with a stable target reference. Existing offsets, transforms, and influences are taken into account. Add rest keys before and after an impact if you need the mesh to return to rest: a single held key does not create an automatic return animation.

## Animating

- Create, duplicate, rename, and delete animation clips. The animation manager provides **Move Up / Move Down**, preserving a saved authored order without renaming clips. Displaying durations in frames in the manager and automatically entering Animate mode when selecting a manager row remain incomplete.
- **Setup** changes the reference structure and pose; **Animate** works on the active clip. Auto-key can record an exact return to Setup for an already animated channel without creating keys on every other channel. With auto-key disabled, existing preview and manual keying operations remain available.
- The **Timeline / Dopesheet** shows bone tracks and editable slot, constraint, event, draw-order, and FFD tracks according to their types. Not every track offers the same menus or multi-selection operations.
- The **Graph Editor** edits scalar Linear, Stepped, and Bézier curves, including overshoot. **Broken / Aligned / Mirrored** tangents, marquee selection, and **Normalize View Only** make curve work easier. Normalize changes the view, not stored values. Universal normalized-value selection/editing and a scalar Graph Editor for FFD vectors are not provided.
- Numeric retiming, project FPS changes with Undo/Redo, and time offsets for keys belonging to explicitly selected bones are available within their validated scopes. Selected-bone offsets do not retarget arbitrary constraint tracks or provide universal time scaling.
- The scalar-key clipboard preserves relative times and curves, with explicit destinations. **Paste Keys Mirrored (Offsets)** mirrors bone-track offsets: X, rotation, and shear change sign, while Y and scale remain unchanged. It does not guarantee a symmetric absolute pose for asymmetric Setup rigs and does not infer left/right suffixes.
- Compatible multi-bone pose blocks and key blocks are applied through one native transaction. Ambiguous, locked, or incompatible destinations are refused before any edit is made.
- **Deform** keys have controls for time, deletion, and interpolation. Their dedicated clipboard accepts a block from one track belonging to the same owned mesh in the same document, with validated ownership and topology. It does not freely copy FFD between projects, ambiguous skins, or linked meshes.
- **Onion Skinning** displays nearby poses with tint/opacity settings and selection options; these ghosts are not a complete physics simulation.
- **Path Position** animates the position fraction of an existing Path constraint. It provides **Key Position**, a preview and manual keying when auto-key is off, and a dedicated Dopesheet row. Animated Path Spacing and PathPosition runtime export are not available.
- Events, attachment switches, and draw-order changes have their own semantics. Do not assume all clipboard, Graph Editor, or mirror operations apply to them. Undo/Redo history covers document edits; camera and selection remain session state.

## Saving and recovering work

**Save** writes to the active path; **Save As…** only changes that path after a successful write. **New**, **Open…**, closing, and opening recovery copies while a document is modified follow **Save / Discard / Cancel** protections. A failed Save must not cause the current document to be lost. Work on a copy and keep its `.assets` folder together with the project.

Autosave uses the interval configured in Settings (zero disables it), a monotonic clock, and a deferred idle wake-up. An unchanged blank document does not create unnecessary copies. Recovery copies for named projects are separate sidecars; unsaved documents use a temporary location. Autosave is not the same as a manual Save or a durable, independent backup.

On startup, **Recover unsaved work?** offers relevant copies. **Ignore** remembers the decision for that exact recovery identity and state without deleting the project, copy, or assets. A new or modified copy may appear again. **File > Review Recovery Copies…** also shows ignored copies for the active project and retained Untitled projects. To review copies for a different named project, open that project's path first. Multiple copies may be offered one after another after Ignore; Recover loads the currently displayed copy.

A failure to save the Ignore decision is reported and the dialog remains available; **Later (this session)** can postpone it without claiming to remember that choice permanently. Corrupted files are not repaired automatically. Recover replaces the current document only after successful loading and revalidation, and marks the result as modified. Use Save As and reopen the saved result. The previous session's Undo history is not stored in `.ankh`.

Copies older than a validated primary project normally are not offered. Timestamps remain a heuristic. Windows may clean temporary files; Ignore markers can be lost or corrupted, causing a copy to be offered again. There is no guarantee of atomic multi-file transactions, durability after power loss, or recovery of every last keystroke.

## Exporting

The **Export** panel separates **Data / atlas**, **Image sequence**, and **Spritesheet**. Preflight distinguishes errors, fidelity warnings, and informational messages, showing files, collisions, replacements, and orphans before requesting the necessary acknowledgments. Preflight does not reserve the destination folder: paths, locks, permissions, staging, and rollback are checked when publishing the export. Cancel publishes nothing; ordinary failures preserve previous outputs within the documented guarantees.

**Data / atlas** uses the native v2 preset or custom templates with the documented export context. Scalar X/Y curves remain independent. A field being exported (a constraint, physics, a linked mesh, etc.) does not mean the runtime knows how to apply it. A project containing PathPosition is rejected for runtime exports that cannot represent it.

**Image sequence** supports RGBA PNG, JPEG with an explicit opaque RGB background, and lossless RGBA WebP. It reuses the existing Rust CPU renderer and evaluator. Samples occur at `i / fps`, with an exclusive end and common framing: one second at 60 FPS yields 60 images. Optional trimming, padding, and bounds provide reconstruction information. Ankhimate metadata retains the historical `png_sequence_version` key even for JPEG/WebP; use the filename pattern rather than assuming an extension from that key.

**Spritesheet** produces transparent PNG pages and Ankhimate JSON metadata with frame indices, rectangles, padding, and pivots. This JSON is not a native Phaser atlas that can be loaded without an adapter. The pixels can be reconstructed using its metadata. Batch export places multiple clips in separate subfolders; its historical ordering by source index is not the animation manager's new authored order.

**MP4/GIF** use a separately installed compatible FFmpeg executable, which is not bundled. Set its path in Settings or make it available on your PATH. Validated encoding options, cancellation, and errors are shown. No codec is guaranteed without that tool. Ankhimate does not install FFmpeg automatically.

Rendered exports remain restricted to the **default skin** and **Normal blending**, and they refuse **physics**. A constraint that the CPU evaluator can handle is not automatically compatible with the runtime. Do not bypass protective export refusals by editing JSON files.

## Using a game runtime

PixiJS and Phaser belong to a separate runtime repository. Their qualified profiles cover a subset of native v2: transforms, scalar animations, compatible weighted meshes/FFD, skins, static dark tint, and clipping/blending within each adapter's limits. Historical results apply to specific versions and scenarios, not every rig.

This package includes the editor and Jade, **not a game runtime or browser demo**. Advanced constraints, physics, linked meshes, and unqualified modes have no universal compatibility guarantee. **PathPosition is not supported by these runtimes at this checkpoint.** Read [Compatibility](COMPATIBILITY.md) before exporting: successful CPU rendering does not establish GPU parity.

## Optional AI assistance

Open **View > AI Assistant**, then use Settings to configure one of the six implemented provider profiles: **OpenAI, Anthropic, DeepSeek, Ollama, LM Studio, and NInfer**. The latter three target local loopback servers that must already be installed and running. No model is included, downloaded, or started automatically. A chosen model must actually support the provider's protocol, tools, and vision capabilities where required.

Chatting by itself does not require sharing your project. Bounded metadata reads, visual captures, and edit operations have separate permission requirements. Images are not sent automatically. Source paths and complete files are not included in context by default. Cloud credentials remain in the session or in Windows Credential Manager if you explicitly request storage.

The agent can read parts of the rig and propose animation keys, small bone operations, region/slot assembly, and weight corrections for existing owned meshes. Mesh reads are paginated; normalization and smoothing use bounded deterministic operations. Free-form geometry creation, replacing binds, and implicit UV/topology/FFD edits are not supported. Locks, revisions, and target references are checked again before applying changes.

Proposed operations are displayed before approval. **Approve native edits / Refuse changes** and native validation remain authoritative; model-generated text is not permission. An accepted batch becomes one Undo/Redo transaction without automatically running inference again. Stale or invalid batches are refused without partial application. A technically valid weight correction is not a guarantee of artistic quality.

Automated mock-based tests validate transports and contracts; they do not certify Qwen's artistic quality, every real provider, or the resulting character assembly. Corresponding manual validation is separate and still pending. No real AI inference was carried out in T838.

## Everyday workflow

Open Jade through **File > Open…**, then save your own copy with **Ctrl+Shift+S**. **Ctrl+S** saves; **Ctrl+N / Ctrl+O** open the protected New/Open workflows; **Ctrl+Z / Ctrl+Y** use the default native Undo/Redo history. Check the keymap in Settings: shortcuts are configurable, and some text fields capture key presses.

Choose Setup or Animate explicitly, along with the relevant clip, skin, and attachment. Use the hierarchy for selection, the viewport for navigation/manipulation, and the inspector for precise values. Selection tools, zoom/pan, framing, and gizmos are contextual; disabled commands indicate a target that cannot be edited in the current context. Diagnostics help navigate to elements but do not automatically fix every problem or guarantee camera framing.

When reporting an issue, include the candidate version, reproduction steps, work mode, active clip, and error message. A small reproduction that omits private project data and credentials is enough. Physical Windows and artistic checks for this candidate remain **PENDING_MANUAL**: this document describes implemented functionality, not certification of every UI interaction.
