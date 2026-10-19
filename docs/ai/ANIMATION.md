# Animation principles within native tool limits

Sample times are seconds; key angles are degrees, translation offsets are local
units and scale is a multiplier. Neutral offsets are 0, neutral scale is 1.
Key time may equal duration, even for a looping clip whose playback end wraps.
A single held key can deform the whole clip: explicitly key rest at t=0 and
return to rest at the end when a clean loop is requested.

Use small deliberate changes and inspect existing motion before overwriting it.
For a looping idle, begin/end with equal values and use smooth interpolation.
For an attack, separate anticipation, fast impact and slower recovery. Coordinate
parent and child bones without assuming world-space key values. Preserve existing
clips unless asked to change them. Runtime crossfades are not editable through
these tools; a transition must be authored with supported keys or explained as a
manual/runtime step. No event, constraint, physics or FFD authoring tool is exposed.
