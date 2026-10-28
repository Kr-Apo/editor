# T-840B6 — Performance and memory

## Method and limits

Windows x64, Rust1.95, Node24.18.0, existing TypeScript5.9.3; no new dependency.
Same five genuine native rigs:6bones/1spring,6/3independent,6/3dependent,32/3,128/64.
Exact original B5B2 core sources plus the read-only audit probe were measured BEFORE
transaction changes. Their recovered hashes match pre-measurement records. The baseline
was not reconstructed from an approximate algorithm. No performance gate is invented.

Rust debug and release are distinct processes/profiles. Each workload13repetitions,
3warmups discarded,10samples retained with median/min/max. TS rotating operation order,
then same13/3/10schedule; explicit GC only before/after the whole measurement process.
One tick and failure cases use B3 directly. Advance120/1200 uses actual ConsumerBridge
with B4 sampler; eight simultaneous live owners each advance120. Long TS case advances
36,000 ticks (300logical seconds), not a five-minute real-time graphical soak. Constructor
placement differs by workload, so compare like-for-like, never Rust vs TS speed rankings.
Rust observer-failure timing includes caught panic and the panic hook's stderr work.

Raw separate-phase timing is subject to machine/JIT/environment drift. In particular,
roughly50% TS and32% debug-many apparent improvements cannot be attributed to candidate
copying. They are retained, not presented as a B6 speedup. A same-process alternating
TS baseline/control run was added to isolate transaction cost with identical output
assertions. It uses direct B3 and a constant setup provider, hence is not a replacement
for the B4/Consumer full workload or real animation rendering.

## Release Rust advance1200, median milliseconds

| Rig | B5B2 | B6 | Delta |
|---|---:|---:|---:|
|single|2.1809|2.2941|+5.19%|
|independent|2.4458|2.5176|+2.94%|
|dependent|2.4515|2.5271|+3.08%|
|medium|5.9451|5.9075|-0.63%|
|many|56.8422|56.9485|+0.19%|

## Same-process alternating TypeScript (direct B3)

|Rig|Ticks|B5B2 ms|B6 ms|Delta|
|---|---:|---:|---:|---:|
|single|120|2.1066|2.1597|+2.52%|
|single|1200|19.4571|19.7379|+1.44%|
|independent|120|2.5514|2.5938|+1.66%|
|independent|1200|24.9822|25.4053|+1.69%|
|medium|120|8.2856|8.2792|-0.08%|
|medium|1200|89.1816|87.9287|-1.40%|
|many|120|180.4665|179.7043|-0.42%|
|many|1200|1816.1277|1836.5668|+1.13%|

Small differences overlap timing variance; negative deltas are not an optimization claim.
Full raw one-tick, failure-second, failure-observer, snapshot100, advance120/1200,
eight-owner and long-session measurements, samples/min/max are in `baseline-*.json`
and `b6-*.json`. These expose variance instead of hiding it behind a single percentage.

## Component costs (milliseconds per1000 operations)

Measured actual private candidate/accept functions:23repeats,3warmups,20samples.
Rust candidate timing also drops each temporary map; it is preparation+temporary release,
not a pure allocator-only measurement. Accept includes replacing/dropping old records
and frame; discard drops preprepared candidates/poses. TS discard measures only release
of reachability, not synchronous destruction or the deferred GC cost. Microbenchmarks
can be nonmonotonic for tiny map sizes; do not extrapolate from one row.

|Language/profile|Springs|Operation|Median ms|Min ms|Max ms|
|---|---:|---|---:|---:|---:|
|Rust debug|1|candidate|0.30760|0.30430|0.34760|
|Rust debug|1|accept|0.33670|0.29080|0.44060|
|Rust debug|1|discard|0.28770|0.24720|0.69790|
|Rust debug|3|candidate|0.17330|0.17230|0.27160|
|Rust debug|3|accept|0.28960|0.27180|0.38630|
|Rust debug|3|discard|0.28490|0.25440|0.33670|
|Rust debug|64|candidate|1.12870|1.10270|1.16530|
|Rust debug|64|accept|1.03250|0.91720|1.14280|
|Rust debug|64|discard|0.84650|0.74520|1.10350|
|Rust release|1|candidate|0.19950|0.19910|0.20540|
|Rust release|1|accept|0.21470|0.20100|0.23090|
|Rust release|1|discard|0.19620|0.19370|0.20620|
|Rust release|3|candidate|0.04720|0.04700|0.10450|
|Rust release|3|accept|0.21510|0.20000|0.24330|
|Rust release|3|discard|0.20810|0.19580|0.22020|
|Rust release|64|candidate|0.11400|0.11090|0.12550|
|Rust release|64|accept|0.76470|0.69060|0.96130|
|Rust release|64|discard|0.62040|0.54440|0.70710|
|TS|1|candidate|0.11000|0.09530|0.25240|
|TS|1|accept|0.03720|0.02130|0.07980|
|TS|1|discard|0.00020|0.00010|0.00060|
|TS|3|candidate|0.15600|0.14590|0.27750|
|TS|3|accept|0.02270|0.02070|0.02980|
|TS|3|discard|0.00010|0.00010|0.00030|
|TS|64|candidate|3.28550|3.21620|3.47390|
|TS|64|accept|0.08120|0.02880|0.12310|
|TS|64|discard|0.00070|0.00030|0.00130|

## Copy/allocation ownership and observed process memory

B6 adds a map with N owned numerical values (8 scalars each), its entries/container,
an active-ID array and (in TS cloneRecords) a temporary mapped entry array. No immutable
rig/bones/geometry/clip clone is added per tick. Fresh pose/FK, returned frame records
and detached observer structuredClone existed already. Rust owns Vec2/scalars; TS's
explicit eight-field copy prevents mutable Map-value aliasing. Calendar definitions
are immutable. An error releases only candidates; there is no reverse integration.

|Process|Stage|RSS bytes|Heap used bytes|ArrayBuffers bytes|
|---|---|---:|---:|---:|
|baseline|before|67411968|5253856|338039|
|baseline|after|267067392|6865048|338039|
|B6|before|67469312|5256376|338039|
|B6|after|268955648|6878864|338039|

These are explicit-GC process snapshots, not comparable allocation totals: process
RSS includes retained V8 arenas/JIT. No GC event count, allocator-byte instrumentation,
peak residency or leak certification was executed. Number/shape of added allocations
is established by source ownership; memory/GC conclusions are deliberately limited.
No OOM recovery guarantee or performance SLA. Consider reusable double-buffer numeric
workspace only in a separate measured change with the complete fault matrix unchanged.
Do not remove finite checks or isolation to optimize a timing sample.
