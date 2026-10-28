# T-840B5B2 — Consumer performance

Windows x64; Rust1.95 debug/unoptimized and Node24.18 are DIFFERENT build targets.
Do not compare their milliseconds as equivalent implementations. Three genuine
5/32/128-bone rigs, three Physics constraints.18 repetitions,3 warmups discarded;
15 measured samples with raw values,min,max,median. TS rotates operation order.
Rust initial series runs operations sequentially; its late128-bone timings show
substantial drift/variance, so they do not establish a skin overhead regression.
No safety copy, context check or last-frame guard is removed. No renderer/GPU work.

Units: unchanged requests/status per1000, snapshot/copy/update-without-Physics per100,
Physics workloads per120 ticks including construction and bounded pumps. Catch-up
per600 ticks with snapshots between pumps. Eight-instance CPU workload is eight
independent120-tick runs; TS holds all eight owners concurrently, Rust initial
benchmark scopes each run sequentially (not an eight-owner retained-memory test).
Both implementations' integration tests keep multiple live owners independent.

| Runtime | bones | snapshot100 ms median [min,max] | B5B1 transport120 ms | bridge120 ms | eight instances120 ms | catch-up600 ms |
|---|---:|---:|---:|---:|---:|---:|
| Rust | 5 | 0.1119 [0.1113,0.1985] | 1.5417 | 1.5592 | 12.5778 | 7.5051 |
| Rust | 32 | 0.3368 [0.3328,0.3688] | 6.2498 | 6.1095 | 49.2235 | 28.9726 |
| Rust | 128 | 1.1188 [1.0880,1.2310] | 21.8675 | 21.1784 | 355.6330 | 215.4880 |
| TS | 5 | 1.1618 [1.1121,1.3850] | 3.0731 | 3.0595 | 24.8211 | 13.8698 |
| TS | 32 | 4.2617 [4.1830,4.5985] | 10.3161 | 10.5236 | 83.2409 | 48.9979 |
| TS | 128 | 15.3041 [14.8748,15.8294] | 38.0947 | 38.1788 | 305.9339 | 183.4750 |

B5B1 comparison is the same-build ReplayTransport.withSkins control, same fixtures
and schedule, not a comparison against old machine timings. TS bridge120 medians
are within ~2.1% of transport controls in these samples, with extra snapshots at
pump boundaries. There is no general speed or real-time budget certification.
Rust late128 visual/eight/catch-up series is not compared as isolated skin cost.
The visual event at120 also preserves two active constraints that membership drops:
that row combines membership/work and sampling differences, not pure overhead.

Snapshot scale: TS100 reads grow1.16->4.26->15.30ms; owned pose copies alone grow
1.01->4.16->15.07ms. Source is B3.frame structuredClone of bone/affine arrays;
metadata/status is cheap (roughly0.08-0.09ms/1000 reads). Rust Vec/name/record copying
also grows with bones (100 snapshots0.112->0.337->1.119ms in debug). Proposal for
adapters: retain one owned committed snapshot per UPDATE and reuse it for every
redraw/mesh, avoiding O(characters*bones*redraws) copies. This is already possible
with current ownership rules. A future read-only view/cache needs lifetime and
consumer-mutation qualification; do not remove detachment for a benchmark gain.

Snapshot count is NOT tick count: update-without-Physics requests exact zero and
copies100 times. Demand scheduling, status, pose-copy, bridge-only and combined
Physics costs are separate raw rows. Skin lookup reuses B5B1's calendar; no rig
rebuild per tick. Local tests measure numerical/instance isolation, not resource
leak freedom on real textures. Rust allocator-byte/GC event counts NOT_RUN.
TS process memory snapshots around all workloads with explicit GC (no retained
allocation/leak inference):
{
  "before": {
    "rss": 67227648,
    "heapTotal": 10461184,
    "heapUsed": 5255856,
    "external": 2297107,
    "arrayBuffers": 338039
  },
  "after": {
    "rss": 284274688,
    "heapTotal": 157786112,
    "heapUsed": 6582776,
    "external": 2332423,
    "arrayBuffers": 338039
  }
}

No cross-language build-speed ranking, large-character-count certification or GPU
performance claim. Larger/deeper rigs, asset-heavy poses, allocations by owner and
real game adapters remain separate qualification work.
