# Project facts and boundaries

Bones form a parent hierarchy; setup transforms are parent-local. World Y is up.
Slots attach visuals to bones and establish draw order. Skins choose attachment
entries; regions use images, meshes may use several weighted bones and FFD.
Read summaries identify available meshes; they do not expose image bytes or
grant topology/skin edits. Weight edits require a targeted guarded read. A visual torso need not be named `torso`.

Animations contain timed tracks. Read real names, duration, looping, units and
current selection before proposing edits. Snapshots are bounded (256 bones,
64 clips, 256 attachments, 64 tracks/clip and 64 keys/track), not the full file.
Treat truncation as incomplete knowledge. Request clarification rather than guess.

The editor supports many manual rigging and rendering operations. The AI tools
expose only the bone, animation and guarded region assembly subset listed in [TOOLS.md](TOOLS.md).
Unsupported runtime features are not made compatible by writing an animation.
