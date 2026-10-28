# T-840B6 — Fault matrix

All numerical equality below is exact inside the language: Rust raw f32 bits, TS
strict deep equality (including negative zero). Committed private map, anchor fields,
active identities and public frame are compared, not merely `faulted` or a visual pose.
Private injection hooks exist only in Rust cfg(test) and an outside-repository TS
instrumented sidecar. Production has no fault-arm or state-restore API.

| Point / test | Before / attempted mutation | Required and observed after | Clock / skin / public frame | Result |
|---|---|---|---|---|
| RED second constraint | Tick0, A integrates, B parent singular | Before fix: A private state changes while frame stays old; after fix exact original records/anchors | completed0, requested1, backlog1; frame0 | RED then GREEN both languages |
| Invalid context, old epoch, regressive time | Accepted prefix, rejected public request | No physical work or mutation | Existing B2/B5A/B5B2 request rejection rules retained; no invented global-command rollback | PASS historical and full suites |
| Unknown bone / nonfinite input / provider error | Prefix8, candidate input rejects before constraints | All private state and public frame equal prefix8; another owner unchanged | requested60/completed8/backlog52, faulted; committed skin remains on | PASS cross-language |
| Later solver / singular parent | A succeeds, B cannot invert actual parent | No partial offset/velocity/anchor, dependent FK or frame survives | rejected tick1/9/120/240 excluded; prior prefix retained | PASS cross-language |
| Native arithmetic overflow | Finite maximum force and small valid mass on B after A | Existing finite checks reject true integration overflow; no nonfinite publication | completed0, frame0, faulted | PASS Rust and TS negative input test |
| Stable membership private fields | Prefix8, post-integration fault at each of A/B/C | Inject position NaN, velocity Inf, rotation NaN, angular velocity -Inf, anchor NaN or nested local/world corruption; exact old state | requested60/completed8/backlog52, membership unchanged | PASS Rust returned error + unwind, TS exception |
| Skin remove | Tick9 changes on to off, deletes A/C candidates, remaining B integrates then fails | Deleted records restored by retaining original committed map; all anchors/active IDs exact | old on skin/frame8, backlog52 | PASS private injections |
| Skin prime | Tick9 off to on, initializes missing A/C in authored order, fails after each solver | New candidate records discarded; common old records untouched | old off skin/frame8, backlog52 | PASS private injections |
| Real calendar boundaries | Ticks120 on to off and240 off to on; observer or later B fails | Rejected membership and numerical creation/deletion never accepted | committed skin at N-1; no candidate frame published | PASS cross-language |
| Precommit observer rejects | All constraints computed; audit errors/panics | Whole tick candidate discarded, accepted private numerics/frame unchanged | completed remains N-1, request retained, faulted | PASS Rust unwind and TS exception |
| Nested observer mutation then throw | Candidate records spliced, pose array appended, world set Inf | Detached nested observer view cannot mutate rig, accepted map or pose | old completed/frame/skin | PASS TS sidecar |
| Nested observer mutation then success | Same detached view mutations without throw | Accepted frame equals unmodified control, observer aliases cannot poison it | coherent successful new tick | PASS TS sidecar |
| Observer reads during skin transition | Candidate9/off shown to audit;100 public reads inside audit | Public records/frame/skin stay8/on, with already accepted intent9 | completed8/backlog1 during audit; after return9/off, completed9 | PASS consumer unit |
| Cancellation offered before integration | Prefix8, queueTime offer rejects tick9 | No candidate or numerical mutation; prefix intact | requested60, completed8, backlog52, not faulted | PASS cross-language and B5B2 |
| Late fault within one request | Request60, ticks1..3 accepted,4 fails | Exactly the clean prefix3, not origin; never roll back successful ticks | requested60/completed3/backlog57/faulted | PASS both languages |
| Faulted pump / reset / stale context | Any faulted case | Pump refused; explicit epoch reset required, old context rejected;360 reconstructed ticks match fresh control | Rebuilt trajectory is deterministic; no hidden resume | PASS cross-language |
| Isolation and source integrity | Two owners from same author/native rig | Second owner, native/author bytes, rig definition and unrelated source remain unchanged | No global numerical state | PASS corpus/private tests and preservation receipt |

Rust private fault loop: (3 stable +1 removal +3 priming points) x6 fields x2
(returned Err / caught unwind) =84. TS:42 corresponding post-step faults plus
4 provider/observer/mutation cases x3 transitions =54. A test returning Err after
injection proves discard; genuine natural overflow separately proves existing finite
checks. These are complementary, not a claim that injected errors are all detected
by a new validator. No validator or numerical threshold is weakened.

33 native cross-language cases: single7, independent11, dependent11, medium2, many2.
Fault at9 after8 committed ticks gives backlog52 of60; faults at1/120/240 retain the
corresponding successful prefix. Successful partial request test gives3/60 and57due.
B3 reset selects fixed-default as before; fresh controls use that exact context.
Transport/consumer reset calendar persistence remains separately checked by B5B2.

No asynchronous cancellation while a synchronous solver runs. External observer
side effects cannot be reversed. Rust panic abort/OOM/host kill and TS fatal VM failure
are excluded; caught Rust unwind and synchronous JS exceptions are covered.
