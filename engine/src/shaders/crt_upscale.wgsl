// CRT upscale shader for Adrenochrome Ascent.
//
// Samples the 320×200 render target with nearest-neighbor filtering, applies
// the active floor-cluster palette tint, then layers lo-fi horror post:
// scanlines, vignette, Bayer dither, and subtle film grain.
//
// Style target: assets/images/style_reference/ (PS1/CRT liminal horror).

@group(0) @binding(0) var crt_source: texture_2d<f32>;
@group(0) @binding(1) var crt_sampler: sampler;
// `palette_tint` from `CrtMaterial::palette_tint`.
@group(0) @binding(2) var<uniform> palette_tint: vec4<f32>;
// x = scanline, y = vignette, z = dither, w = time (seconds).
@group(0) @binding(3) var<uniform> crt_params: vec4<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

fn bayer4(p: vec2<f32>) -> f32 {
    // Cheap ordered-dither threshold derived from pixel coords.
    let x = u32(abs(p.x)) % 4u;
    let y = u32(abs(p.y)) % 4u;
    let index = y * 4u + x;
    // Bit-twiddle approximation of a 4×4 Bayer matrix (0..1).
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

    // Mild barrel distortion toward CRT glass (kept subtle so gameplay stays readable).
    let centered = in.uv * 2.0 - 1.0;
    let barrel = 1.0 + dot(centered, centered) * 0.045;
    let warped_uv = centered * barrel * 0.5 + 0.5;

    // Soft chromatic aberration for that "broken CRT" edge.
    let aberr = 0.0018 * length(centered);
    var color = textureSample(crt_source, crt_sampler, warped_uv);
    let r = textureSample(crt_source, crt_sampler, warped_uv + vec2<f32>(aberr, 0.0)).r;
    let b = textureSample(crt_source, crt_sampler, warped_uv - vec2<f32>(aberr, 0.0)).b;
    color = vec4<f32>(r, color.g, b, color.a);

    // Floor-cluster palette grade.
    var rgb = color.rgb * palette_tint.rgb;

    // Horizontal scanlines (reference: blood hall / gun corridor).
    let scan = 0.5 + 0.5 * sin(warped_uv.y * 200.0 * 3.14159265);
    let scan_mod = mix(1.0, scan, scanline_strength);
    rgb = rgb * scan_mod;

    // Edge vignette — darkness-forward horror framing.
    let vig = 1.0 - dot(centered * vignette_strength, centered * vignette_strength);
    rgb = rgb * clamp(vig, 0.15, 1.0);

    // Ordered dither for crunchy lo-fi gradients.
    let px = warped_uv * vec2<f32>(320.0, 200.0);
    let threshold = bayer4(px) - 0.5;
    rgb = rgb + threshold * dither_strength * 0.08;

    // Animated grain so static frames still feel alive.
    let grain = (hash21(px + vec2<f32>(time * 37.0, time * 19.0)) - 0.5) * 0.045;
    rgb = rgb + grain;

    // Slight crush into blacks (asylum / blood halls).
    rgb = max(rgb - vec3<f32>(0.02), vec3<f32>(0.0));
    rgb = clamp(rgb, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(rgb, color.a);
}
