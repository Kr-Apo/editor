# Historical negative fixture

`prototype-minimal.ankh` is copied byte-for-byte from commit
`5c6d435ca72684117abde0c8a293b414b560fca2:formats/tests/fixtures/minimal.ankh`
(blob `af7ded30d2c93c097181a3175398a38f6e8eb4ed`). It is a ZIP containing
project.json with internal version **1**, not v0. The current ANKH v1 reader
rejects it. It is retained to prevent conflating the removed prototype with
current schema version 1. SHA-256 and all supported v1 fixture provenance are
in `../format-versions.json`. No authentic v0 fixture was located; none was invented.
