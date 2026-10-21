# Local build and preparation tools

Use [current build instructions](../docs/CANDIDATE_BUILD.md) and [pre-alpha workflow](../docs/prealpha-workflow.md). T836 produces a separate local Windows-Test-Candidate and never republishes an existing archive.

The former `package-alpha.ps1` is retired to [historical source text](history/package-m0072-alpha.ps1.txt). Its M006/M007 runtime/package/showcase pins are obsolete. It is retained byte-identically for restoration, not offered as a current executable packager. No tests invoke it. No global cleanup/push/release tool is introduced.
