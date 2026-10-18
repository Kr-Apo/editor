---
title: Export and runtime contracts
description: Distinguish authoring projects from strict template output and external runtime responsibilities.
---

# Export and runtime contracts

Export presets combine atlas settings and one or more strict Handlebars templates.
The [template context reference](/editor/export-context/) is a public compatibility
contract: field names cannot be silently renamed. Missing fields are errors with
locations, never empty strings. Helpers and atlas page/region data are documented
in that reference.

Ankhimate's runtime shape is itself the `ankhimate_runtime` built-in preset, not a
special Rust serializer. This is load-bearing: if the public template engine cannot
express the project's own runtime output, it is too weak for users' engines.

The built-in preset now emits **runtime format v2**, with `format_version: 2`.
Bone translation, scale and shear use independent scalar channels (`translate_x`,
`translate_y`, `scale_x`, `scale_y`, `shear_x`, `shear_y`), each keeping its own
key times and outgoing curve. Bone offsets are already baked into those times;
v2 does not emit a separate `offset`. This is independent of `context_version: 1`
and `preset_version: 1`, whose public contracts have not changed.

The updated TypeScript reader accepts legacy exports with no `format_version`
or version 1 and preserves their paired-track semantics. V2 requires the updated
reader; older readers can silently ignore the scalar fields. Saved user presets
are not automatically replaced: select/copy the current built-in preset and
re-export the authoring project to recover previously omitted axes. See the
[M001a validation record](../milestones/M001a-validation.md) for compatibility
details and the numerical Rust/TypeScript gate.

Built-in JSON templates serialize string values and object keys with `{{json name}}`,
including its surrounding quotes. Names and event payloads retain their original
Unicode, quotation marks, backslashes and control characters after parsing.
This does not change runtime v2, its independent axes or v1 reader compatibility.
Saved/custom templates must use the same helper; they are not silently migrated.

Every export builds a complete plan first. Text, atlas pages and copied images
share path/collision validation and a staged writer with backups and rollback.
Every `.json` output is parsed before any disk changes; malformed template/plugin
JSON is refused with an explicit error, while foreign schemas remain author-owned.
Windows file names, reparse points and replacement behavior are handled explicitly.
Orphans are reported and retained. Multi-file publication is not atomic to readers
or across a crash; see [export safety](../export-safety.md) for precise guarantees
and recovery limitations. Plugin outputs reach this same writer without changing
their API.

Runtime implementations live outside this workspace. A conforming runtime consumes
the documented generated shape, respects animation/constraint semantics it claims
to support, and tests representative exports. The `.ankh` authoring container is
not a runtime contract and includes editor-only and source data by design.

The normative author/runtime field tables and version/migration boundaries are in the
[format specification](../format-spec.md#native-runtime-json-format-version-2). The author
adapter provided by some external runtimes is a separate reader, not the native JSON
contract; it does not make prototype ZIP projects compatible.
