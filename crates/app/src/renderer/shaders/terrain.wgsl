/// `ShaderPack::UNIFORM_GROUP`
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

/// `ShaderPack::VERTEX_GROUP`
@group(1) @binding(0)
var<storage, read> vertex_data: array<TerrainVertex>;

/// `ShaderPack::TEXTURE_GROUP`
@group(2) @binding(0)
var texture: texture_2d<f32>;
@group(2) @binding(1)
var texture_sampler: sampler;

struct Uniforms {
    camera: mat4x4f,
};

/// See terrain_vertex.rs
struct TerrainVertex {
    pos: vec3f,
    color: u32, // rgba, packed4x8unorm
}

struct VertexOut {
    @builtin(position) position: vec4f,
    @location(0) tex_coord: vec2f,
    @location(1) color: vec4f,
};

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32) -> VertexOut {
    var out: VertexOut;

    let in = vertex_data[v_idx];
    out.position = uniforms.camera * vec4f(in.pos, 1.0);

    out.tex_coord = vec2f(0.0);
    out.color = unpack4x8unorm(in.color);

    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    //let tex = textureSample(texture, texture_sampler, in.tex_coord);
    //if (tex.a == 0.0) {discard;}
    //let alpha_mul =  tex.a * tex.rgb; // 👈 assumes texture is non-premultiplied
    //return vec4f(mix(alpha_mul, tex.a * in.mix_color.rgb, in.mix_color.a), tex.a);
    return vec4f(in.color.rgb, 1.0);
}
