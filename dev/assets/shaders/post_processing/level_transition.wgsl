const CELLS = 300;
const STEPS = 50.0;
const COLOR1 = vec3(0.0, 0.0, 0.0);
const COLOR2 = vec3(0.01, 0.01, 0.01);

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct LevelTransitionShaderSettings {
    time: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: LevelTransitionShaderSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let cellSize: f32 = 1.0 / CELLS;

    let uv: vec2<f32> = in.uv;

    let tex: vec4<f32> = textureSample(screen_texture, texture_sampler, uv);

    let progress: f32 = 1.0 - (settings.time);

    let sUv: vec2<f32> = fract(uv * cellSize);

    let distToEdge: f32 = min(min(sUv.x, 1.0 - sUv.x), min(sUv.y, 1.0 - sUv.y));

    var pUv: vec2<f32> = uv;

    pUv -= pUv % cellSize;

    let stepProgress: f32 = progress % (1.0 / cellSize);
    var fadeProgress: f32 = abs(pUv.y - 0.5) + stepProgress;
    fadeProgress = pow(fadeProgress, 5.0);
    let fadeProgress2: f32 = pow(fadeProgress, 2.0);
    let fadeProgress3: f32 = pow(fadeProgress2, 2.0);
    let fadeProgress4: f32 = pow(fadeProgress3, 2.0);

    // pattern
    let r: f32 = max(0.07, random(pUv));
    let p: f32 = 1.0 - step(fadeProgress, r);
    let p2: f32 = 1.0 - step(fadeProgress2, r);
    let p3: f32 = 1.0 - step(fadeProgress3, r);
    let p4: f32 = 1.0 - step(fadeProgress4, r);

    let rt: f32 = mix(p2, p3, stepProgress);
    let pColor: vec4<f32> = vec4<f32>(mix(COLOR2, COLOR1, p3), 1.0);
    let rgb = vec4<f32>(tex.rgb, 1.0);

    // Output to screen
    return vec4<f32>(mix(rgb, pColor, p2));
}

fn random(st: vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy,
                         vec2(12.9898,78.233)))*
        43758.5453123);
}
