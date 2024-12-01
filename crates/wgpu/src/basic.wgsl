struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = model.uv;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@group(0) @binding(0)
var tex: texture_1d<f32>;
@group(0) @binding(1)
var smplr: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texel_size = 1.0 / f32(textureDimensions(tex));

    let fragment_size = abs(dpdxFine(in.uv.x));

    var texel_pos = in.uv.x - (0.5 * fragment_size);
    let texel_max = in.uv.x + (0.5 * fragment_size);

    var peak = textureSample(tex, smplr, texel_pos).xy;
    texel_pos += texel_size;
    while texel_pos < texel_max {
        let sample = textureSample(tex, smplr, texel_pos);
        peak = vec2<f32>(min(peak.x, sample.x), max(peak.y, sample.y));
        texel_pos += texel_size;
    }

    let v = (in.uv.y * 2) - 1;
    if v < peak.y && v > peak.x {
        return vec4<f32>(0.5, 0.6, 0.8, 1.0);
    } else {
        discard;
    }
}