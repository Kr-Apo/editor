# Viewport observation v2

`ankhimate_viewport` takes `{}`: requests Capture/Preview/Consent, NOT pixels.
Ask for a reviewed, authorized new Send. Never invent an image observation.
All six profiles have native image transports; model vision is separate.
Unknown vision requires an explicit untested attempt; incompatible is blocked.
Discovery and HTTP success never certify perception. No automatic inference.

Only the canvas crop: PNG <=1 MiB, longest side 512/768/1024 by quality.
No desktop, files or reuse on later Sends. Consent expires after 2 minutes or
project/destination/model change. Stateless tool continuation may resend this
image up to the explicitly authorized request budget. Stop cannot recall data.

Image origin is the top-left EDGE, X right/Y down. Native mapping gives world
origin and signed world-units/pixel (world Y up); pixel centers add .5.
Current world pose may differ from Setup. Bone transforms are parent-local;
read actual setup hierarchy for placement. Overlays can obscure the art.
Report observations as estimates; approval never bypasses native validation.
