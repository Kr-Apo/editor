# T-809 — multimodal transport and bounded context

Development branch only; the frozen public Alpha RC2 is unchanged. No new
rigging command, evaluator or format. T-806/T-807/T-808/T-809 artistic, GUI and
live-model qualification remains **pending maintainer acceptance**.

## Transport matrix

| Profile | Image content in a user message | Stream / tools | Qualification |
|---|---|---|---|
| OpenAI | Responses input_text + input_image, PNG data URL, detail auto | SSE / function calls, encrypted reasoning replay | Synthetic HTTP only |
| Claude | Messages image.source: base64, media_type image/png; separate text | SSE / tool_use, thinking/signatures replay | Synthetic HTTP only |
| Ollama | /api/chat message.images: raw base64; content remains text | NDJSON / native JSON tool arguments; thinking retained | Synthetic HTTP only |
| DeepSeek | Chat Completions text + image_url, PNG data URL | SSE / tool deltas, reasoning_content retained | Synthetic HTTP only |
| LM Studio | Compatible text + image_url, PNG data URL | SSE / compatible tool deltas (including fragmented llama.cpp style) | Synthetic HTTP only |
| NInfer | Same compatible transport, not another HTTP stack | SSE / tool deltas, reasoning_content retained | Synthetic HTTP only |

Wire contracts checked against [OpenAI vision](https://developers.openai.com/api/docs/guides/images-vision),
[Responses reasoning](https://developers.openai.com/api/docs/guides/reasoning),
[Claude vision](https://platform.claude.com/docs/en/build-with-claude/vision),
[Ollama REST vision](https://docs.ollama.com/capabilities/vision) and
[DeepSeek vision](https://api-docs.deepseek.com/guides/vision/).
The latter documents deepseek-flash image input; this is documentation evidence,
not a live test of that model. No provider name, model name or successful HTTP
response enables a vision capability automatically. ChatGPT subscriptions do
not provide an OpenAI API key or API billing credit.

## Compatibility and consent

Settings → Model capabilities records user declarations for text, images, tools,
streaming and separate reasoning: Unknown / untested, Confirmed by user, or
Incompatible. The declaration is bound to the exact provider, actual canonical
endpoint and model ID. Changing destination makes it unknown; discovery never
qualifies it and sends no image. Only the most recently configured destination's
declarations are retained, not a database of model certifications.

All six transports remain text-compatible. Declared incompatible text/streaming
blocks Send; incompatible tools requires disabling tools. Unknown tools can be
attempted by explicitly enabling Model tools. Reasoning is informational: no
reasoning mode or temperature parameter is invented. Opaque output is retained
only as required by the actual response protocol, not shown as model thoughts.

Viewport preview is still mandatory. Unknown images additionally require
“Attempt this image with an untested model”. Then authorize **THIS** image for
the next Send. Metadata permission is independent. Image role must be user;
only bounded, validated inline viewport PNG is accepted. No file path, external
URL, arbitrary image upload or automatic capture is added. Debug formatting
redacts binary image data. Image bytes are skipped by session-message
serialization and never saved in projects, exports or preferences.

The exact PNG preview states dimensions and base64 size, destination, revision,
two-minute expiry and maximum HTTP image requests. Changing destination in
Settings clears the capture, even when changing back. New document/revision
requires new permission. Changing capture quality requires a new preview.
Send consumes the permission, also on error/Stop. A later Send contains no old
pixels. Previously visible conversation and the submitted draft survive an API
refusal; remove the image to send text only. No automatic paid retry occurs.

## Images and continuation cost

Light/Balanced/Detailed deterministically downsample longest side to at most
512/768/1024 (never upscale), PNG <=1 MiB, preserving mapping and alpha. Aspect
ratio remains fixed; nearest sampling may alias. These are input-size profiles,
not equivalent perception, cost estimates or GPU memory guarantees. Hard PNG
validation checks header, dimensions, allocation bounds, decoding and mapping.

A Send may have multiple model requests around native tool results. Current
protocols are stateless: Responses uses store:false and encrypted reasoning;
Claude/Chat/Ollama replay required messages. Each such continuation must carry
the image to preserve visual context. Default consent permits at most **two**
image requests; advanced control allows 1–17, bounded also by 1–16 native calls
and at most two invalid-proposal corrections. Once exhausted there is a clear
error, no edit or silent extra request. Increase the budget deliberately or
continue text only with the native data. No automatic file upload, provider
storage, previous_response_id chain or extra paid continuation is introduced.
Stop cancels the client future; it cannot recall bytes or guarantee that a
provider has stopped processing/billing an already received request.

Repeated exact read/read_mesh calls inside the immutable Send snapshot return
a compact cache_hit/source_call_id/revision result pointing at the first native
result already retained in wire history. No project revision is hidden by this
cache: it ends with the Send; editor revision changes cancel the active job and
live approval still revalidates. This avoids repeated native payloads, not the
required image replay. Mesh pagination remains bounded. Other array reads can
use {"section":"bones","offset":0,"limit":16}; legacy unpaged reads remain
supported within the existing bounded snapshot. Overview/selection are not arrays.
Metadata truncation remains explicit in counts/limits; oversized snapshots refuse.

## Budgets and local model profile

| Budget | Balanced (default) | Small local model | Advanced range |
|---|---|---|---|
| Whole-Send timeout | 60 s | 120 s | 1–600 s |
| Output / response | 2048 tokens | 1024 tokens | 128–32768 |
| Native calls / Send | 6 | 4 | 1–16 |
| Context bytes excluding PNG base64 | 256 KiB | 96 KiB | 32–2048 KiB |
| Embedded manual | 6144 bytes | 3072 bytes | 2048–6144 |
| Image longest side | 768 | 512 | 512 / 768 / 1024 |
| HTTP requests carrying image | 2 | 2 | 1–17 |

The context budget counts instructions, exact schemas, native results and all
required message/reasoning history. The full wire request is additionally <=4
MiB, each streamed response <=2 MiB, project metadata <=64 KiB and user input
<=16 KiB. These byte ceilings are **not token-window predictions**. No schema or
required continuation is silently discarded to favor history. Required context
that cannot fit produces an explicit error before that HTTP request.
Small local omits redundant tool prose before relevant business guidance; the
core safety manual and exact enabled tool schemas stay intact. Guides are
selected deterministically, not all injected at once. Manual length may be
lower than its ceiling because whole optional sections are included or omitted.

For modest local hardware start with Small local model, one precise question,
a Light image, and paged mesh reads. Clear unneeded chat explicitly. Increase
output/timeout/context only if the installed server/model supports it. No 8 GB
hardware or inference-speed qualification is claimed.

## Errors and manual acceptance

401/403 identifies authentication; 429 identifies quota/rate limit without retry.
Known context_length_exceeded/context_window_exceeded codes identify a provider
context refusal. Unknown 400/413/422 image failures state that the cause is not
certified: model vision, parameters or context may be responsible. Error bodies
are never echoed into chat/logs. Output-limit, content-filter, incomplete stream,
malformed JSON/UTF-8, disconnect, timeout and Stop remain distinct. No partial
or truncated response becomes a native edit proposal. A server silently ignoring
pixels is not detected or certified by HTTP success.

Synthetic tests cover six emitted HTTP envelopes, image→read→proposal,
Approve/Refuse, receipts on the next image-free Send, three Undo/Redo cycles,
opaque reasoning continuation, bounded duplicates, refusal/privacy/limits and
Stop/timeout. Historical rig/assembly/weights/save/export tests remain required.
These mocks do not replace the maintainer's manual A–F acceptance (Qwen/NInfer,
Ollama, voluntary cloud, tools after image, missing consent, nonvisual model).
