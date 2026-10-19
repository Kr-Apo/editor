# Native AI assistant — candidate guide

The Windows candidate includes T-805 through T-809. Open the **AI Assistant**
pane from View and configure the exact provider, endpoint and model. Supported
transport profiles are OpenAI, Anthropic, DeepSeek, Ollama, LM Studio and NInfer.
They are transport implementations, not certification of every hosted/local model.
No model, credential or backend is bundled. Nothing launches an inference server.

Use native reads before a small correction. Rigging and assembly operate in Setup;
animation tools follow Animate/keying rules. Proposed edits are typed, validated
on an isolated document, shown before/after and require explicit approval. Refusal
or stale revisions must not modify the document. Approved batches are native
atomic commands with Undo/Redo and save/reload support. Linked meshes, bind changes,
mesh topology/UV changes and FFD writes remain restricted as documented by each tool.

Viewport images require a preview and explicit consent bound to the destination
and actual model; native metadata reads do not silently send image bytes or local
project paths. Capabilities can be Unknown: opt in deliberately instead of assuming
vision/tool support. Start with the small local profile and targeted/paged reads.
Long conversations can exceed context budgets; shorten them and narrow requests.
No autonomous capture/inference/correction loop is provided.

## Evidence levels

- Automated Rust tests cover native tools, approval/atomicity, read pagination,
  privacy, six synthetic HTTP image envelopes, failures, budgets and Undo/Redo.
- The maintainer reports a Qwen/NInfer Jade test: consented viewport read, native
  bone inspection; proposal/approval for `left_upper_arm` and `left_forearm`;
  `left_arm` reassignment preserving Setup geometry; `qwen_left_arm_test` with
  five keys over two seconds; visible motion; mesh `default/right_arm/art`
  inspection and smoothing vertices 4/5; normalized re-read, Undo/Redo and Save/Reload.
- That modified project was not saved. It is **not included**. The sample remains
  the accepted RC2 project. This evidence qualifies those reported gestures, not
  every rig, model, GPU or artistic result. T-811 ran no real inference.

Detailed manuals: [assistant](native-ai.md), [vision](native-ai-vision.md),
[assembly](native-ai-assembly.md), [weights](native-ai-weights.md),
[multimodal budgets](native-ai-multimodal.md), [tool manual](ai/ANKHIMATE-AI.md).
Never attach credentials or private projects to public bug reports.
