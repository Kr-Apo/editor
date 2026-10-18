// One lazily created pipeline per renderer, owned/destroyed by Phaser's manager.
// Recreating rigs does not allocate additional pipelines or shader programs.
const pipelines = new WeakMap();
const name = "AnkhimateStaticDarkTint";
const fragment = `
precision highp float;
uniform sampler2D uMainSampler;
uniform vec3 uLight;
uniform vec4 uDark;
varying vec2 outTexCoord;
varying vec4 outTint;
void main () {
    vec4 texel = texture2D(uMainSampler, outTexCoord);
    // PNG textures are uploaded premultiplied by Phaser. Algebraically equal to
    // (T.rgb*L.rgb + (1-T.rgb)*D.rgb*D.a) * T.a * L.a without dividing by T.a.
    vec3 rgb = texel.rgb * uLight + (vec3(texel.a) - texel.rgb) * uDark.rgb * uDark.a;
    gl_FragColor = vec4(rgb * outTint.a, texel.a * outTint.a);
}`;
function pipelineFor(scene) {
    const renderer = scene.sys.game.renderer;
    const cached = pipelines.get(renderer);
    if (cached)
        return cached;
    const manager = renderer.pipelines;
    const single = manager.get("SinglePipeline");
    if (!single || manager.has(name))
        throw new Error("Ankhimate dark tint requires an available Phaser SinglePipeline");
    // Obtain the public pipeline class from the active Phaser instance. Keep the
    // adapter's type-only Phaser import: no global or second Phaser copy required.
    const Single = single.constructor;
    class DarkTintPipeline extends Single {
        onBind(gameObject) {
            const data = gameObject?.pipelineData;
            const tint = data?.ankhimateDarkTint;
            if (!tint)
                return;
            // Uniforms belong to a draw batch, not a mesh. Flush the preceding mesh
            // before replacing its values, even when this pipeline is already active.
            this.flush();
            this.set3f("uLight", tint.light[0], tint.light[1], tint.light[2]);
            this.set4f("uDark", tint.dark[0], tint.dark[1], tint.dark[2], tint.dark[3]);
        }
    }
    const pipeline = new DarkTintPipeline({ game: scene.sys.game, fragShader: fragment });
    manager.add(name, pipeline);
    pipelines.set(renderer, pipeline);
    return pipeline;
}
export function applyDarkTint(scene, mesh, light, dark) {
    if (dark && dark[3] !== 0 && dark.slice(0, 3).some(channel => channel !== 0)) {
        if (!mesh.frame.source.glTexture?.pma)
            throw new Error("Ankhimate dark tint requires premultiplied atlas textures (Phaser PNG loading)");
        const pipeline = pipelineFor(scene);
        mesh.setPipelineData("ankhimateDarkTint", { light, dark });
        mesh.setPipeline(pipeline);
    }
    else if (mesh.pipeline === pipelines.get(scene.sys.game.renderer)) {
        mesh.resetPipeline();
        delete mesh.pipelineData.ankhimateDarkTint;
    }
}
//# sourceMappingURL=dark-tint.js.map