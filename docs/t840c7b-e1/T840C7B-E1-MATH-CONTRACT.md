# T-840C7B-E1 — Math contract

## Native frames and source of truth

Sources: core/src/attachment.rs (bind_to_pose, skin_vertex_with_deform), core/src/transforms.rs (Affine2::invert), formats schema/conversion, export/src/context.rs (setup_worlds, packed_weights, flat_vertices), the native preset and docs/format-spec.md. NUMERIC-AUDIT.txt inventories the relevant inverse/determinant/fallback sites. No global utility is refactored.

A mesh stores one setup vertex v in its slot bone's local space. Weighted mesh attachment data has no region offset transform; do not apply region offsets to it. H is the slot host's setup WORLD affine; B_j is an influencing bone's setup WORLD affine. setup_worlds reuses native pose evaluation, including its setup constraint convention. Core bind_to_pose stores inverse(B_j) * H in f32; native skinning evaluates B_j(t) * inverseBind_j * (v + deform_j). E1 does not change this code or FFD.

Native influence x/y must therefore represent l_j = inverse(B_j) * H * v. Runtime placement is the weighted sum of B_j(t) * l_j, divided by total usable weight. Coefficients/order/counts are preserved, including zero and nonunit sums. Empty/nonpositive-total rows retain the native rigid placement convention; they are not normalized by export. Influence coordinates are also present in flat_vertices with bone indices and weights; vertices/UVs/triangles remain unchanged.

For B=[a c tx; b d ty], solve with widened f32 coefficients:

    dx = (H.tx - B.tx) + H.a*v.x + H.c*v.y
    dy = (H.ty - B.ty) + H.b*v.x + H.d*v.y
    determinant64 = a*d - b*c
    local = ((d*dx-c*dy)/determinant64, (a*dy-b*dx)/determinant64)

Relative translations are subtracted before adding the vertex terms, avoiding the old world-f32 quantization followed by cancellation. Independently check B*local against H*v, with max absolute component error <= 0.01. Native source WORLD must also be finite.

## Eligibility, precision and conditioning

Core's f32 determinant eligibility is abs(det)>=1e-12; exact threshold and adjacent f32 values are tested. Core's invert result must exist and have finite coefficients. Source affine coefficients/vertices, f64 determinant/local/reconstruction must be finite. A tiny uniform scale can have a tiny determinant but condition number near 1; tested diagonal (1e-18,1e18) has condition about 1e36 yet its chosen points reconstruct accurately. Neither determinant nor a condition-number cutoff alone decides validity. Private test logs include infinity-norm estimates and residuals; this is not a universal conditioning certification.

Authored fields remain f32. Derived LOCAL coordinates are finite f64 JSON numbers in the existing number fields, including flat_vertices. Weighted bone parameters use full authored precision, since four-decimal rounding can turn a valid 1e-6 scale into zero. Nonweighted output retains the old rounding. No schema/version or reader change is introduced.

After rendering a recognized native template, a second check reads the actual serialized bone parameters and weights. It reconstructs setup through the unchanged native core and checks weighted WORLD against source WORLD. This detects legacy-template rounding loss. It is not a new JavaScript trigonometry/renderer implementation or a certificate for general animated constraints. Source-f32 quantization and native f32 inverse multiplication explain small differences between ideal affine, source point and native skinning; report all three instead of hiding them by f32 rounding.

## Explicit failure policy

Context inspection stays infallible: invalid derived points are null plus a root binding_errors list; valid contexts omit that list. skeleton.precise_bind_numbers is an additive template-context hint, never a native-v2 member. run::plan fails before template publication when binding_errors exists. Native rendering fails if serialized reconstruction loses fidelity. No failed inverse becomes identity/WORLD; previous files remain untouched by planning. Singular/native-rejected projects now refuse misleading exports; old templates are retained but may need the current built-in preset. Custom templates keep their own format semantics and do not gain the native post-render precision check.

An encountered accepted matrix that cannot be represented within the unchanged contract/tolerance requires explicit evidence and HOLD; E1 is not permission to increase tolerance or arbitrarily clamp a matrix. None of the accepted historical fixtures or bounded tested cases requires such a migration. Extreme untested floats and downcasting consumers remain unqualified.
