// CRT upscale shader for Adrenochrome Ascent (TODO-002 / TODO-034).
//
// Samples the 320×200 render target with nearest-neighbor filtering, applies
// the active floor-cluster palette tint, then layers lo-fi horror post:
// scanlines, vignette, Bayer dither, phosphor glow, pain/serum flashes, grain.
//
// Style target: assets/images/style_reference/ (PS1/CRT liminal horror).
//
// Material2d bind layout: group 0 = view, group 1 = mesh, group 2 = material.

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var crt_source: texture_2d<f32>;
@group(2) @binding(1) var crt_sampler: sampler;
// `palette_tint` from `CrtMaterial::palette_tint`.
@group(2) @binding(2) var<uniform> palette_tint: vec4<f32>;
// x = scanline, y = vignette, z = dither, w = time (seconds).
@group(2) @binding(3) var<uniform> crt_params: vec4<f32>;
// x = pain flash, y = serum tint, z = phosphor glow, w = unused.
@group(2) @binding(4) var<uniform> post_fx: vec4<f32>;

fn bayer4(p: vec2<f32>) -> f32 {
    let x = u32(abs(p.x)) % 4u;
    let y = u32(abs(p.y)) % 4u;
    let index = y * 4u + x;
    let bit = (index * 5u + (index / 4u) * 3u) % 16u;
    return f32(bit) / 16.0;
}

fn hash21(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.xyx) * 0.1031);
    p3 = p3 + dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let scanline_strength = crt_params.x;
    let vignette_strength = crt_params.y;
    let dither_strength = crt_params.z;
    let time = crt_params.w;
    let pain = clamp(post_fx.x, 0.0, 1.0);
    let serum = clamp(post_fx.y, 0.0, 1.0);
    let phosphor = clamp(post_fx.z, 0.0, 1.0);

    // Mild barrel distortion toward CRT glass.
    let centered = in.uv * 2.0 - 1.0;
    let barrel = 1.0 + dot(centered, centered) * 0.045;
    let warped_uv = centered * barrel * 0.5 + 0.5;

    // Soft chromatic aberration for that "broken CRT" edge.
    let aberr = 0.0018 * length(centered) + pain * 0.0025;
    var color = textureSample(crt_source, crt_sampler, warped_uv);
    let r = textureSample(crt_source, crt_sampler, warped_uv + vec2<f32>(aberr, 0.0)).r;
    let b = textureSample(crt_source, crt_sampler, warped_uv - vec2<f32>(aberr, 0.0)).b;
    color = vec4<f32>(r, color.g, b, color.a);

    // Floor-cluster palette grade.
    var rgb = color.rgb * palette_tint.rgb;

    // Horizontal scanlines.
    let scan = 0.5 + 0.5 * sin(warped_uv.y * 200.0 * 3.14159265);
    let scan_mod = mix(1.0, scan, scanline_strength);
    rgb = rgb * scan_mod;

    // Edge vignette.
    let vig = 1.0 - dot(centered * vignette_strength, centered * vignette_strength);
    rgb = rgb * clamp(vig, 0.15, 1.0);

    // Ordered dither for crunchy lo-fi gradients.
    let px = warped_uv * vec2<f32>(320.0, 200.0);
    let threshold = bayer4(px) - 0.5;
    rgb = rgb + threshold * dither_strength * 0.08;

    // Phosphor glow — soft green-white bloom on bright pixels.
    let luma = dot(rgb, vec3<f32>(0.299, 0.587, 0.114));
    let glow = vec3<f32>(0.35, 0.85, 0.45) * max(luma - 0.35, 0.0) * phosphor * 0.55;
    rgb = rgb + glow;

    // Pain flash (damage) + serum veil — integrated post, not UI sprites.
    rgb = mix(rgb, vec3<f32>(0.85, 0.08, 0.1), pain * 0.55);
    rgb = mix(rgb, vec3<f32>(0.15, 0.55, 0.75), serum * 0.35);

    // Animated grain.
    let grain = (hash21(px + vec2<f32>(time * 37.0, time * 19.0)) - 0.5) * 0.045;
    rgb = rgb + grain;

    // Slight crush into blacks.
    rgb = max(rgb - vec3<f32>(0.02), vec3<f32>(0.0));
    rgb = clamp(rgb, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(rgb, color.a);
}
