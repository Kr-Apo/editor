# T-840B5A — Transport CPU performance

## Method

Same Windows x64 development machine, Rust 1.95 debug/test profile, Node 24.18.0.
Five warmup repetitions then 50 measured repetitions, min/median/p95/max retained.
Same five-bone idle/ABC author rigs (one/three springs) for B4 and B5A. No GUI/GPU.
These are descriptive local CPU measurements, not production latency budgets.

Advance includes fresh construction, priming, B5A play(false), an exact target request
(eight tick budget) and an explicit pump for the remaining ticks. Replay includes
initial construction, a fresh-origin reset/replay command, queueing without numerical
work, and 64-tick pumps until complete. B4 controls perform the same initial/reset and
queue/pump workload via B4's controller; there is no alternative solver. Measurements
include validation, allocation, FK, numerical checks and unchanged observer delivery.

## End-to-end workload and tick costs

| Engine | Rig | Path | Ticks | Median ms | p95 ms | Median us/tick |
|---|---|---|---|---|---|---|
| Rust | idle | B4-advance | 120 | 1.0724 | 1.1166 | 8.937 |
| Rust | idle | B5A-advance | 120 | 1.0767 | 1.1473 | 8.973 |
| Rust | idle | B4-replay | 120 | 1.0866 | 1.1904 | 9.055 |
| Rust | idle | B5A-replay | 120 | 1.0916 | 1.1775 | 9.097 |
| Rust | idle | B4-advance | 600 | 5.2790 | 5.3817 | 8.798 |
| Rust | idle | B5A-advance | 600 | 5.3140 | 5.3918 | 8.857 |
| Rust | idle | B4-replay | 600 | 5.3027 | 5.4589 | 8.838 |
| Rust | idle | B5A-replay | 600 | 5.3556 | 5.4756 | 8.926 |
| Rust | idle | B4-advance | 1200 | 10.4999 | 11.0350 | 8.750 |
| Rust | idle | B5A-advance | 1200 | 10.6676 | 11.3363 | 8.890 |
| Rust | idle | B4-replay | 1200 | 10.6790 | 10.9876 | 8.899 |
| Rust | idle | B5A-replay | 1200 | 10.7374 | 11.1011 | 8.948 |
| Rust | ABC | B4-advance | 120 | 1.4162 | 1.6256 | 11.802 |
| Rust | ABC | B5A-advance | 120 | 1.4282 | 1.5226 | 11.902 |
| Rust | ABC | B4-replay | 120 | 1.4703 | 1.6265 | 12.252 |
| Rust | ABC | B5A-replay | 120 | 1.4520 | 1.6139 | 12.100 |
| Rust | ABC | B4-advance | 600 | 7.0408 | 7.5317 | 11.735 |
| Rust | ABC | B5A-advance | 600 | 7.1248 | 7.4504 | 11.875 |
| Rust | ABC | B4-replay | 600 | 7.0958 | 7.4368 | 11.826 |
| Rust | ABC | B5A-replay | 600 | 7.1128 | 7.6639 | 11.855 |
| Rust | ABC | B4-advance | 1200 | 14.1857 | 22.1166 | 11.821 |
| Rust | ABC | B5A-advance | 1200 | 14.2175 | 14.9126 | 11.848 |
| Rust | ABC | B4-replay | 1200 | 14.1651 | 14.6923 | 11.804 |
| Rust | ABC | B5A-replay | 1200 | 14.2324 | 14.7341 | 11.860 |
| TypeScript | idle | B4-advance | 120 | 2.0995 | 2.5251 | 17.496 |
| TypeScript | idle | B5A-advance | 120 | 2.1237 | 2.6075 | 17.698 |
| TypeScript | idle | B4-replay | 120 | 2.0864 | 2.3529 | 17.387 |
| TypeScript | idle | B5A-replay | 120 | 2.2163 | 2.5968 | 18.469 |
| TypeScript | idle | B4-advance | 600 | 9.4178 | 9.8236 | 15.696 |
| TypeScript | idle | B5A-advance | 600 | 9.4341 | 9.7814 | 15.723 |
| TypeScript | idle | B4-replay | 600 | 9.3836 | 9.9533 | 15.639 |
| TypeScript | idle | B5A-replay | 600 | 9.5241 | 10.1732 | 15.874 |
| TypeScript | idle | B4-advance | 1200 | 18.3969 | 18.7485 | 15.331 |
| TypeScript | idle | B5A-advance | 1200 | 18.5911 | 19.3848 | 15.493 |
| TypeScript | idle | B4-replay | 1200 | 18.8733 | 19.4185 | 15.728 |
| TypeScript | idle | B5A-replay | 1200 | 18.6255 | 19.3681 | 15.521 |
| TypeScript | ABC | B4-advance | 120 | 2.5996 | 5.7241 | 21.663 |
| TypeScript | ABC | B5A-advance | 120 | 2.6001 | 3.2169 | 21.668 |
| TypeScript | ABC | B4-replay | 120 | 2.5686 | 2.8031 | 21.405 |
| TypeScript | ABC | B5A-replay | 120 | 2.7187 | 2.9332 | 22.656 |
| TypeScript | ABC | B4-advance | 600 | 12.1098 | 12.5074 | 20.183 |
| TypeScript | ABC | B5A-advance | 600 | 12.2440 | 12.7586 | 20.407 |
| TypeScript | ABC | B4-replay | 600 | 12.2408 | 12.8217 | 20.401 |
| TypeScript | ABC | B5A-replay | 600 | 12.5661 | 18.0398 | 20.944 |
| TypeScript | ABC | B4-advance | 1200 | 24.0670 | 24.7660 | 20.056 |
| TypeScript | ABC | B5A-advance | 1200 | 24.0252 | 24.6528 | 20.021 |
| TypeScript | ABC | B4-replay | 1200 | 24.0698 | 24.8178 | 20.058 |
| TypeScript | ABC | B5A-replay | 1200 | 24.0808 | 24.6815 | 20.067 |

Relative medians (negative values are retained as noise/workload outcomes, not claimed
optimizations):

| Engine | Rig | Workload | Ticks | B5A vs B4 |
|---|---|---|---|---|
| Rust | idle | advance | 120 | +0.40% |
| Rust | idle | replay | 120 | +0.46% |
| Rust | idle | advance | 600 | +0.66% |
| Rust | idle | replay | 600 | +1.00% |
| Rust | idle | advance | 1200 | +1.60% |
| Rust | idle | replay | 1200 | +0.55% |
| Rust | ABC | advance | 120 | +0.85% |
| Rust | ABC | replay | 120 | -1.24% |
| Rust | ABC | advance | 600 | +1.19% |
| Rust | ABC | replay | 600 | +0.24% |
| Rust | ABC | advance | 1200 | +0.22% |
| Rust | ABC | replay | 1200 | +0.48% |
| TypeScript | idle | advance | 120 | +1.15% |
| TypeScript | idle | replay | 120 | +6.23% |
| TypeScript | idle | advance | 600 | +0.17% |
| TypeScript | idle | replay | 600 | +1.50% |
| TypeScript | idle | advance | 1200 | +1.06% |
| TypeScript | idle | replay | 1200 | -1.31% |
| TypeScript | ABC | advance | 120 | +0.02% |
| TypeScript | ABC | replay | 120 | +5.84% |
| TypeScript | ABC | advance | 600 | +1.11% |
| TypeScript | ABC | replay | 600 | +2.66% |
| TypeScript | ABC | advance | 1200 | -0.17% |
| TypeScript | ABC | replay | 1200 | +0.05% |

## Commands and observation

Each sample measures 100 repetitions, divided by 100 below. Pause-resume-pair is two
commands, not a single-command estimate. Frame-borrow is Rust's immutable current
reference; frame-owned-copy measures an explicit Pose/record clone. TS frame-copy
includes the required detached copy; frame-owned-copy adds a second caller copy.
Pure-pose-query includes B4's full pure construction/FK; it is not just a lookup.
Restart/stop/seek/reset include constructing and validating a fresh candidate epoch.

| Engine | Rig | Operation | Median us/op or pair | p95 us/op or pair |
|---|---|---|---|---|
| Rust | idle | play-same | 0.0340 | 0.0340 |
| Rust | idle | pause-resume-pair | 0.0370 | 0.0380 |
| Rust | idle | snapshot | 0.1040 | 0.1050 |
| Rust | idle | frame-borrow | 0.0290 | 0.0300 |
| Rust | idle | frame-owned-copy | 1.1680 | 1.8460 |
| Rust | idle | pure-pose-query | 3.4030 | 4.1300 |
| Rust | idle | restart | 21.2620 | 23.1570 |
| Rust | idle | stop | 21.1960 | 24.1230 |
| Rust | idle | seek | 25.0740 | 27.2260 |
| Rust | idle | reset | 21.1330 | 23.5880 |
| Rust | ABC | play-same | 0.0340 | 0.0340 |
| Rust | ABC | pause-resume-pair | 0.0360 | 0.0530 |
| Rust | ABC | snapshot | 0.1040 | 0.1050 |
| Rust | ABC | frame-borrow | 0.0280 | 0.0290 |
| Rust | ABC | frame-owned-copy | 0.9830 | 1.3900 |
| Rust | ABC | pure-pose-query | 3.6960 | 5.2500 |
| Rust | ABC | restart | 26.0360 | 29.1880 |
| Rust | ABC | stop | 26.2680 | 29.0180 |
| Rust | ABC | seek | 29.8030 | 32.1050 |
| Rust | ABC | reset | 26.1130 | 29.0600 |
| TypeScript | idle | play-same | 0.0130 | 0.1550 |
| TypeScript | idle | pause-resume-pair | 0.0500 | 0.1520 |
| TypeScript | idle | snapshot | 0.1170 | 0.1330 |
| TypeScript | idle | frame-copy | 8.9160 | 9.9690 |
| TypeScript | idle | frame-owned-copy | 18.4220 | 20.7560 |
| TypeScript | idle | pure-pose-query | 2.4450 | 3.5560 |
| TypeScript | idle | restart | 128.3800 | 136.7510 |
| TypeScript | idle | stop | 127.4810 | 135.8650 |
| TypeScript | idle | seek | 130.2580 | 136.8940 |
| TypeScript | idle | reset | 127.0360 | 131.3370 |
| TypeScript | ABC | play-same | 0.0280 | 0.0280 |
| TypeScript | ABC | pause-resume-pair | 0.0480 | 0.0530 |
| TypeScript | ABC | snapshot | 0.1080 | 0.3900 |
| TypeScript | ABC | frame-copy | 10.4610 | 11.4610 |
| TypeScript | ABC | frame-owned-copy | 21.5860 | 23.2430 |
| TypeScript | ABC | pure-pose-query | 2.0230 | 2.5370 |
| TypeScript | ABC | restart | 166.4700 | 177.2580 |
| TypeScript | ABC | stop | 165.0630 | 173.5150 |
| TypeScript | ABC | seek | 174.1380 | 188.2100 |
| TypeScript | ABC | reset | 169.2580 | 180.6440 |

## Interpretation and costs

There is no new per-tick immutable rig reconstruction in B5A. One old pending context
is discarded only by an explicit replacing command. Rust reuses its immutable source
Arc; TS retains defensive source snapshots and constructs a new private candidate.
That extra TS source clone is visible in short replay/restart costs. Long replays are
dominated by the inherited B4/B3 pose construction, FK, finite checks, spring application
and detached observer copies. No validation or isolation copy was removed for speed.

The smallest 120-tick TS replay samples show roughly six percent overhead, predominantly
candidate construction/reset work; longer workloads are close to their B4 controls.
These five-bone measurements do not certify large-rig restart cost. If large sources
prove problematic, consider a separately measured immutable-source sharing seam; do
not weaken source isolation. No architecture/performance rewrite is implemented here.

Rust pause/resume and snapshot work is tiny in this local fixture; TS current-frame
copies are measurably more expensive than snapshots. Redraw/observation still executes
zero Physics ticks. Snapshot and frame-copy latency must not be confused with simulation
latency. Existing B4 stage-instrumentation evidence remains available unchanged; B5A
does not claim a new decomposition of spring equations or renderer costs.

TS JSON retains rss/heapTotal/heapUsed/external/arrayBuffer snapshots before/after each
55-run workload. They are GC-sensitive process snapshots, not allocation counts or
leak proofs. Rust allocation bytes/counts are NOT_RUN. Identified allocations include
candidate snapshots, per-tick poses inherited from B4, source/context metadata, frame
copies and optional caller-owned copies. Production code reads no performance clock.
No speedup beyond B4 is claimed, and B4's former 9–12% gain is not a general runtime claim.
