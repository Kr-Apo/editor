# Native AI assistant — T-805 / T-805.1 / T-806

This source-only development feature is **not included in the frozen Alpha RC2
ZIP**. Build this branch with the pinned Rust toolchain. The assistant is a
session-local editor panel; it does not change the project or runtime formats.

T-806 adds separately authorized [viewport images and Setup bone rigging](native-ai-vision.md).
The following animation guide remains applicable. [T-809 multimodal transport and budgets](native-ai-multimodal.md)
cover all six profiles; model vision and maintainer visual qualification remain separate.

## Quick start

1. Open a working copy of a project. Choose **View → AI Assistant**, or use
   **View → Open in a window → AI Assistant** for a larger conversation area.
2. Open **Settings**, select a provider and enter its exact model ID. **Fetch models / test
   connection** lists available IDs; it does not certify their tool support.
3. For cloud APIs, enter a key under **API key** in Settings. It is masked and
   session-only until **Store key securely** is clicked. Windows Credential
   Manager stores it per user, scoped to the provider and endpoint. **Remove
   stored key** removes that entry. Other platforms support session keys only.
4. Conversation alone needs no project access. To inspect the rig, explicitly
   allow metadata sharing and enable model tools. Review **Exact project metadata
   available to tools** before sending. For editing, also enable **Allow edits
   with confirmation**. These choices do not start a request.
5. Close Settings to keep the conversation area clear. Enter a multiline draft;
   click **Send** or press **Ctrl+Enter while the composer has focus**. Enter
   alone inserts a newline. Ask “Examine this rig.” Then ask for a small, gentle looping breathing
   animation on the actual chest bone. Review every proposed action and choose
   **Approve native edits** or **Refuse changes**. A chat message saying “apply”
   does not bypass this button.
6. Approved keys appear in the ordinary Dopesheet/Graph Editor and remain
   manually editable. One **Edit → Undo AI Animation Edit** reverses the whole
   document edit; Redo restores it without contacting a model. Save is manual.

**Stop** cancels an active network future. Errors, refusal, cancellation and
stale proposals do not apply partial edits. Changing documents cancels the
request and resets metadata permission. Chat remains visible locally during the
session. Changing provider, model or permissions clears the outbound conversation
before the next request, retaining earlier visible messages locally. Chat is not
saved in `.ankh`, autosaves, exports or preferences.

The header shows provider/model, connection information, real execution phase
and elapsed request time. “Generating / receiving” does not claim access to a
model's internal thoughts. The composer and Approve/Refuse controls remain below
the scrolling conversation. Messages are selectable and have Copy buttons;
scrolling upward keeps the reading position when new text arrives. Technical
errors and native events have distinct cards. Pending proposals list clip,
duration, looping, bones and key operations; **Inspect proposed actions** opens
the exact typed batch. In a short dock, the header collapses to two lines and
Approve/Refuse replace the disabled Send/Stop row while a proposal is pending.
Cancelled/failed requests restore the submitted draft if
you have not already typed a new one.

Native results have locally unique proposal IDs in addition to provider call
IDs. At most two invalid-proposal correction rounds run inside the existing
Send/tool budget. Errors include the operation, field, cause, allowed correction
and unchanged revision. There is no silent set-to-create conversion. Approval
records an authoritative native event and updates only the matching proposal's
tool results. The next Send includes the latest verified result if sharing/tools
are still enabled; later Undo/edits are signalled by revision changes. Approval
itself never starts another model request. Model prose can still be wrong:
the native event, actual timeline and current revision remain the evidence.

## Providers

| Provider | Protocol | Default endpoint | Model selection |
|---|---|---|---|
| OpenAI | Responses API, SSE, function calls | `https://api.openai.com/v1` | Manual ID or `/models` |
| Anthropic | Messages API, SSE, tool-use blocks | `https://api.anthropic.com/v1` | Manual ID or `/models` |
| DeepSeek | Chat Completions, SSE, tool-call deltas | `https://api.deepseek.com` | Manual ID or `/models` |
| Ollama | Native `/api/chat`, NDJSON, JSON tool arguments | `http://127.0.0.1:11434` | Manual ID or `/api/tags` |
| LM Studio | OpenAI-compatible Chat Completions, SSE | `http://127.0.0.1:1234/v1` | Manual ID or `/models` |
| NInfer profile | The same local OpenAI-compatible adapter, SSE/tool calls | `http://127.0.0.1:8080/v1` | Manual ID or `/models` |

No model-name allowlist, cloud account requirement for local use, model download,
automatic retry, or AI request at startup. Qwen and other installed families can
be selected by ID, but a specific model/server must support the chosen protocol
and tools. Disable model tools for conversation-only models. An HTTP rejection
explains this fallback; syntactically invalid tool results are rejected locally.
Cloud endpoints are fixed. Local URLs must be HTTP(S) on loopback, without URL
credentials, query or fragment. LAN-hosted inference is outside this MVP.
TLS certificate validation remains enabled and redirects are refused.

Implemented protocol details include split UTF-8 streaming, tool-argument
fragments, OpenAI reasoning output replay, Anthropic thinking/signature replay,
and DeepSeek/NInfer reasoning-content continuation. NInfer's observed Qwen
server uses `reasoning_content`; no universal backend identity is assumed.
Model-specific capability and API
policy changes still require live qualification; listing an ID is insufficient.

## Data and native tools

Only explicitly enabled, bounded metadata is available to `ankhimate_read`:
bone hierarchy/setup transforms/locks, attachment and mesh summaries, clip
properties and bone key tracks, selection and playhead. The provider receives
the requested section through a tool result, not the whole project automatically.
Limits: 256 bones, 64 animations, 256 attachments, 64 tracks per clip and 64 keys
per track, with totals/truncation limits included. Metadata over 64 KiB is refused.
No image bytes, source paths, PSD provenance, export presets or raw `.ankh` files
are sent. Names and any text you type are still data sent to the chosen provider;
review them for private information. No automatic project-file uploads.

`ankhimate_animate` proposes an explicitly typed batch:

- `animation.create`: unique name, duration and looping flag.
- `animation.select`: an existing animation by exact name.
- `animation.set`: duration and looping, preserving existing keys.
- `key.add`: add/update a bone transform key at a specified clip time.

Supported channels are translation X/Y, rotation, scale X/Y and shear X/Y.
Translation is an additive setup offset; rotation/shear keys use **degrees**;
scale is a **multiplier**, with 1 at rest. Interpolation is Linear, Stepped or
Smooth (native normalized Bézier handles 0.42/0 and 0.58/1). A new breathing loop
should include rest keys at time 0 and its end. There is no new evaluator.

Proposals are checked on an asset-free scratch document using existing native
commands. Approval rechecks revision, locks and references, then dispatches one
undoable animation transaction. Names must resolve uniquely. A batch allows
1–128 actions; duration is 0.01–3600 seconds; key time must be within the clip;
values must be finite and bounded; scale magnitude is 0.0001–100. No mesh, weight, FFD, constraint, event, script, shell or file-writing tool is
exposed. Setup bone creation/transform/rename use the separate T-806 rigging
contract; a viewport observation request opens consent rather than sending an image. Selection/playhead/mode remain session navigation: selecting alone does
not dirty the document or create an Undo entry, following existing editor rules.

Timeout is 1–600 seconds for the entire Send, output is 128–32768 tokens per
provider response, and the tool-call budget is 1–16 per Send (defaults: 60 s,
2048 tokens, 6 calls). Text input is limited to 16 KiB. The configurable context
budget is 32–2048 KiB (default 256), including schemas, instructions and required
history but excluding independently bounded PNG base64. A 4 MiB complete request
and 2 MiB response transport hard ceiling remain. Provider token accounting may
include reasoning; a multi-round Send can consume several response allowances.
Visible history keeps up to 100 messages. Clear conversation to start afresh.

## Qualification and limitations

The six profiles (five existing adapters plus NInfer reusing local compatibility)
are exercised with local deterministic mock servers;
these tests **do not establish a successful live cloud/model connection**.
No authorized cloud credential was supplied. The original T-805 qualification
had no default Ollama/LM Studio server. During T-805.1 the maintainer's existing
local endpoint at `127.0.0.1:8080/v1` advertised `qwen3.8-27b` and was used for
real inference on an isolated Jade copy. This qualifies that observed
model/server configuration, not every NInfer, llama.cpp or Qwen version. No
server was launched/reconfigured and no model was downloaded.

The original T-805 Windows Jade demonstration used an explicitly labelled
simulated Ollama protocol server. T-805.1 additionally exercised real NInfer/Qwen
streaming, metadata reading, a three-key breathing proposal, native approval and
manual Save in the Windows interface. A separate opt-in real-model test checks
three Undo/Redo cycles, Save/reopening, unchanged assets and native v2 export.
These tests use isolated Jade copies; the original sample remains unchanged.
Windows Credential Manager is tested with a disposable fake credential only.

## Embedded expert manual

[AI manual v1](ai/ANKHIMATE-AI.md) links exact tool semantics, project concepts,
animation/rigging principles and recipes. Core instructions and the enabled
schemas are always available; a deterministic French/English keyword selector
adds relevant sections, not the entire library. Everything is compiled into the
binary using `include_str!`; no working-directory or internet dependency.
Public manual text is capped at **6144 UTF-8 bytes** (a conservative ceiling of
6144 byte-level tokens; actual token counts depend on the provider). This is
separate from the 64 KiB project-metadata ceiling and configurable context budget.
T-809 permits a 2048–6144 byte manual budget. Small local profile prioritizes
business sections over redundant tool prose; exact enabled schemas are always
transmitted in full. Oversized required context fails instead of silently dropping history. Native project strings
are never inserted as instructions. [JSON examples](ai/examples.json) are checked
against Rust action fields, the exported schemas and actual native outcomes.

This MVP is not autonomous rigging, image generation, a general-purpose scripting
agent, an artistic-quality guarantee or certification of every model ID. Stop
cannot recall information already sent. Credential Manager protects persistence,
not a compromised Windows user session or a provider's own data policy. No
automatic save is initiated by the assistant; the existing editor autosave
policy still applies after any approved native edit.

Protocol references: [OpenAI streaming](https://developers.openai.com/api/docs/guides/streaming-responses),
[OpenAI function calling](https://developers.openai.com/api/docs/guides/function-calling),
[Anthropic streaming](https://platform.claude.com/docs/en/build-with-claude/streaming),
[DeepSeek tool calls](https://api-docs.deepseek.com/guides/tool_calls/),
[Ollama chat](https://docs.ollama.com/api/chat),
[LM Studio chat](https://lmstudio.ai/docs/developer/openai-compat/chat-completions),
[Windows credentials](https://learn.microsoft.com/en-us/windows/win32/api/wincred/nf-wincred-credwritew).
