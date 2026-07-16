// CRT upscale shader for Adrenochrome Ascent (TODO-002).
//
// Samples the 320×200 render target with nearest-neighbor filtering and
// applies the active floor-cluster palette tint. No curvature/scanlines yet
// (those are TODO-034).
//
// This material is a `Material2d`, so Bevy provides the standard mesh2d
// vertex shader. We only override the fragment entry point. The fullscreen
// quad is a `Rectangle` scaled to cover the viewport; its UVs run 0..1.

@group(0) @binding(0) var crt_source: texture_2d<f32>;
@group(0) @binding(1) var crt_sampler: sampler;
// `palette_tint` is bound from `CrtMaterial::palette_tint` (uniform at binding 2).
@group(0) @binding(2) var<uniform> palette_tint: vec4<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Nearest-neighbor sampling is configured on the sampler/texture, so a
    // plain textureSample gives us the crunchy pixel look we want.
    let color = textureSample(crt_source, crt_sampler, in.uv);

    // Palette swap: multiply the scene by the active floor-cluster tint.
    // Tint components are <= 1.0, so this only darkens/shifts — it never
    // blows out to white. Alpha is passed through unchanged.
    return vec4<f32>(color.rgb * palette_tint.rgb, color.a);
}
