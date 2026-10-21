# Animation manager and saved author order

Open the **Animations** pane. Existing New, Duplicate, Rename, Delete, Duration and Loop controls remain available. The existing manager uses a row context menu (right-click), replacing the older documented Timeline `⋯` menu; no new global shortcut is required.

1. Select the desired clip, or right-click any clip row.
2. Choose **Move Up** or **Move Down**. It swaps with exactly one neighbor.
3. Verify the immediate order in the manager and Timeline picker.
4. Save or Save As. Reopening preserves this order and all clip data.

The first Up and last Down actions are disabled. Empty/singleton lists cannot move. A stale menu after a document/revision/active-clip/mode change or hidden frame closes without editing. Open a fresh menu to retry. A valid click produces one independent Undo; boundaries/refusals produce none and preserve Redo. Selection remains the same logical animation even if another clip is moved; intentional later selection is session state and is not undone.

Manager commands, including order, are legal in Setup and Animate. Existing row selection keeps its mode; the Timeline picker enters Animate. Duration currently displays seconds in the dedicated pane. Original T-208 asks for frames and selection entering Animate, so its complete historical task remains PARTIAL even though ordering is implemented.

The source of truth is Document author order keyed by AnimationId, represented on disk by the existing animation array in all three author profiles. No version/tag/schema changes. Imported/legacy clips initially use occupied-slot order; native New/Duplicate append explicitly, even if SlotMap reuses an old slot. Rename never sorts the list. Create/Duplicate/Delete preserve ID generations through collection-only history snapshots; this costs memory proportional to all clip data for those operations, not for Move. No assets are copied into those snapshots.

Native runtime-v2/template context follows the project array; JSON object enumeration is not a universal playback-order promise. Existing batch export continues to enumerate original occupied storage slots/source indices for its folder names. Single-clip PNG/spritesheet selects the named animation and remains pixel-identical after reorder. Unsupported PathPosition runtime export still refuses. Physics/default-skin/Normal blending restrictions remain unchanged. Old editors are not certified to preserve a new author order on resave. No Windows usability qualification is claimed.
