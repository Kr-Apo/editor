# T-840B5B1 — Performance observations

Windows x64, Rust 1.95 debug/unoptimized, Node 24.18, existing five-bone/three-spring
rig, same environment as B5A. No GPU, wall-time bridge or production timer.
35 repetitions, first five discarded, median of 30. Raw samples retained. Workloads
120/600/1200 ticks include construction, sampling/FK, full instance stepping and
observer ownership costs. TS rotates the four profile orders each repetition.
B5A default and the fixed opt-in calendar select identical constraints and their
complete 360-tick output frames are exactly equal. Membership performs less work
while constraints are absent, so it is not a pure overhead or speedup comparison.

| 1200 ticks, median ms | B5A fixed default | B5B1 fixed calendar | visual calendar | membership calendar |
|---|---:|---:|---:|---:|
| Rust | 15.2061 | 15.1168 | 15.567599999999999 | 16.170299999999997 |
| TypeScript | 52.02419999999984 | 52.6140000000014 | 52.418799999999464 | 51.5662999999995 |

At 1200 ticks the alternating TS fixed-calendar control is ~1.1% slower than B5A;
Rust fixed-calendar is within ~0.6% of its control (noise, not a general speedup).
The earlier sequential TS run showed 32 vs 53 ms depending on run order; it is
retained as evidence, not hidden or used as proof of 65% implementation overhead.
Balanced 52.0 vs 52.6 ms controls show the environment/order sensitivity. There is
no large-rig performance certification and no regression budget is inferred here.

Measured additional operations (1000 queries/copies; ms):
- Rust skin-resolution: 0.2354
- Rust pure-sampling: 22.502100000000002
- Rust frame-clone: 2.2196
- TS skin-resolution/1000: 0.057800000000497676
- TS pure-sampling/1000: 6.0666999999994005
- TS frame-copy/1000: 18.671000000000276

Boundary measurements (ms) include the full single tick: event lookup/set comparison,
removal or priming, author sample/FK and resulting active simulation. These are NOT
isolated allocator/delete/prime timings; active work differs. Raw samples separate
no change, visual-only, deletion and reactivation. No per-tick rig reconstruction.
Resolution uses binary search in a prevalidated calendar; sets are precomputed once.
Common records survive; only changed sets allocate/update membership and records.
Rust owned skin query clones its name/vector; TS returns frozen calendar values.
Rust pure diagnostics currently perform both existing full sample and skinned full
evaluation; this measured cost is outside the advancing animation-locals path.

TS process memory before/after forced GC (not total allocation count):
{
  "before": {
    "rss": 67616768,
    "heapTotal": 10723328,
    "heapUsed": 5465536,
    "external": 2451309,
    "arrayBuffers": 338039
  },
  "after": {
    "rss": 141832192,
    "heapTotal": 78356480,
    "heapUsed": 6763688,
    "external": 2474305,
    "arrayBuffers": 338039
  }
}

These process snapshots include JIT/heap growth and all benchmark instances; they
are neither retained-rig-size nor leak proof. Rust allocator-byte instrumentation:
NOT_RUN. Record counts are bounded by the three active identities in these fixtures;
removal/fresh-state tests prove the required lifecycle, not a broad memory claim.
B5B2 can assess larger calendars/rigs and caching only under unchanged safety rules.
