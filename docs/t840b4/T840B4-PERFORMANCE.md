# T-840B4 — CPU performance

## Method

Windows x64 on the existing development machine, Rust 1.95 debug/test profile and
Node 24.18.0. Two identical five-bone author rigs in both engines: idle (one spring)
and ABC (three springs). No GPU or graphical rendering. Five warmups then 50 measured
repetitions, each 120 ticks; independent runs include fresh controller construction,
snapshot validation, priming, request/pump overhead and no-op observer delivery.
Pure-clip measures 120 full pure pose queries without controller construction.
Direct B3 uses precomputed author locals; this is not a zero-provider-cost comparison.
Rust clones the per-tick input Vec; TS supplies immutable precomputed local arrays.

These local figures are descriptive, not an alpha latency budget or a universal
speed claim. B3's old smaller-rig figures are retained in its report but are not used
as this experiment's baseline. There is no new Legacy timing claim in this B4 table.

## Uninstrumented results

Median and p95 milliseconds per 120 ticks; raw min/max and repetition counts are in
the corresponding JSON files. Rust's authoritative final runs use the binary from
the qualified nonincremental workspace build; its SHA is recorded in the evidence.

| Engine | Rig | Path | Median ms | p95 ms |
|---|---|---|---|---|
| Rust | idle | B3-direct | 0.8206 | 0.9083 |
| Rust | idle | B4-clip | 1.0972 | 1.2737 |
| Rust | idle | pure-clip | 0.3832 | 0.5877 |
| Rust | ABC | B3-direct | 1.1818 | 1.7783 |
| Rust | ABC | B4-clip | 1.4538 | 1.6694 |
| Rust | ABC | pure-clip | 0.4050 | 0.4968 |
| TypeScript | idle | B3-direct | 1.7732 | 2.1034 |
| TypeScript | idle | B4-clip | 2.0793 | 2.5290 |
| TypeScript | idle | pure-clip | 0.2950 | 0.4202 |
| TypeScript | ABC | B3-direct | 2.3191 | 2.6600 |
| TypeScript | ABC | B4-clip | 2.5817 | 2.8896 |
| TypeScript | ABC | pure-clip | 0.2930 | 0.4177 |

## Localized optimization

Initially the B4 provider evaluated an entire pure pose and discarded its FK/slot
results, then B3 rebuilt the pose again. Now only author locals are produced through
the same native sampling stage. Rust adds a crate-private bone-local seam; TS extracts
its existing setup-bone builder and bone-track loop into shared functions. The full
Legacy evaluator retains the same operations and order. No public pose, context,
finite-value validation, observer isolation copy or spring step was removed.

The control restores only this redundant full-pose work in isolated source/module
copies; it does not modify repository sources. Two before/after campaigns and an
additional direct-binary Rust repetition retain their results. First confirmation
rust-performance-confirmed.json is **excluded**: a shared Cargo artifact alias reused
the control-workload executable. Final measurements use separately retained, hashed
control and qualified executables, with the production binary rebuilt using
CARGO_INCREMENTAL=0. No acceptance decision uses that excluded timing file.

| Engine | Rig | Control median ms | Final median ms | Reduction |
|---|---|---|---|---|
| Rust | idle | 1.2423 | 1.0972 | 11.7% |
| Rust | ABC | 1.5942 | 1.4538 | 8.8% |
| TypeScript | idle | 2.3536 | 2.0793 | 11.7% |
| TypeScript | ABC | 2.8228 | 2.5817 | 8.5% |

The first campaign also measured Rust B4 idle 1.2863→1.0909 ms and ABC
1.6016→1.4512 ms; TS idle 2.2544→2.0424 ms and ABC 2.8184→2.5385 ms.
The full pure-clip query also shows a countertrend: TS idle 0.2393→0.2950 ms and
ABC 0.2584→0.2930 ms across the initial/final runs. The experiment does not
establish its cause (JIT/workload ordering also differs), and makes no general
full-evaluator or Legacy speedup claim. All raw values are retained. This is a
small local replay-provider gain; TS observer copying remains the dominant cost.

## Stage instrumentation

Separate isolated source/module copies contain timing probes. Product source files
were never overwritten by instrumentation. Rust uses scoped Instant probes; TS uses
performance.now. Values below are cumulative stage times averaged over 55 complete
runs, **including** warmups. Inclusive/nested spans overlap and must not be summed
as percentages of end-to-end latency. Probes perturb performance and include their
own timer/map/mutex overhead. B4 production contains no instrumentation clocks.

| Engine | Rig | Probe | Calls across 55 runs | Mean ms/run |
|---|---|---|---|---|
| Rust | idle | FK | 13310 | 0.16419 |
| Rust | idle | curves-inclusive | 13310 | 0.08059 |
| Rust | idle | finite-checks | 19910 | 0.16366 |
| Rust | idle | local-construction | 6655 | 0.06241 |
| Rust | idle | observer | 6600 | 0.00283 |
| Rust | idle | pose-construction-inclusive | 6655 | 0.06674 |
| Rust | idle | prepare-pose-inclusive | 6655 | 0.67601 |
| Rust | idle | record-copies | 6655 | 0.03596 |
| Rust | idle | spring-step | 6600 | 0.01469 |
| Rust | ABC | FK | 13310 | 0.16724 |
| Rust | ABC | curves-inclusive | 13310 | 0.09603 |
| Rust | ABC | finite-checks | 33110 | 0.31174 |
| Rust | ABC | local-construction | 6655 | 0.06278 |
| Rust | ABC | observer | 6600 | 0.00278 |
| Rust | ABC | pose-construction-inclusive | 6655 | 0.06588 |
| Rust | ABC | prepare-pose-inclusive | 6655 | 0.68838 |
| Rust | ABC | record-copies | 6655 | 0.07561 |
| Rust | ABC | spring-step | 19800 | 0.03941 |
| TypeScript | idle | local-construction | 13310 | 0.06534 |
| TypeScript | idle | curves-inclusive | 13310 | 0.09809 |
| TypeScript | idle | pose-construction | 6655 | 0.05706 |
| TypeScript | idle | FK | 13310 | 0.10843 |
| TypeScript | idle | finite-checks | 13255 | 0.21539 |
| TypeScript | idle | prepare-pose-inclusive | 6655 | 0.47293 |
| TypeScript | idle | spring-and-subtree | 6600 | 0.12602 |
| TypeScript | idle | observer-copy | 6600 | 1.19749 |
| TypeScript | idle | observer | 6600 | 0.00885 |
| TypeScript | ABC | local-construction | 13310 | 0.06435 |
| TypeScript | ABC | curves-inclusive | 13310 | 0.09823 |
| TypeScript | ABC | pose-construction | 6655 | 0.04393 |
| TypeScript | ABC | FK | 13310 | 0.09226 |
| TypeScript | ABC | finite-checks | 26455 | 0.39193 |
| TypeScript | ABC | prepare-pose-inclusive | 6655 | 0.37726 |
| TypeScript | ABC | spring-and-subtree | 19800 | 0.22188 |
| TypeScript | ABC | observer-copy | 6600 | 1.41533 |
| TypeScript | ABC | observer | 6600 | 0.00658 |

Rust spring-step excludes world rebuild; TS spring-and-subtree includes it. These
probe definitions are different and are not primitive cross-language benchmarks.
Curve probes include lookup/track dispatch/application; local-construction isolates
the setup transform allocation. Pose construction covers surrounding slot/maps/state.
FK counts include B3's setup FK and override FK; per-spring subtree work is in the
spring/application work, not all in that FK counter. Observer notification and its
TS structuredClone are timed separately; Rust record snapshot copying is timed.

## Allocations and remaining costs

TS raw results include rss/heapUsed/heapTotal before/after each 55-run workload.
These are GC-sensitive process snapshots, not allocation counts or leak proofs.
No Rust allocator-wide measurement was installed; Rust allocation bytes are NOT_RUN.
Identified by code/probe counts: per-tick local arrays/Vec, Pose/maps, B3 reconstruction,
record snapshots, TS observer structuredClone, constructor rig/clip snapshots and
temporary membership/look-up collections. No unbounded cache is introduced.

B3 still does setup FK then override FK, and copies observer frames defensively.
Removing those costs safely requires a separately reviewed prepared-pose/observer
contract. B4 does not weaken isolation or silently replace callbacks with borrowed
mutable state to improve figures. Renderer performance and broad hardware budgets
remain outside this numerical qualification.
