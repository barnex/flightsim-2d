
/// `ShaderPack::UNIFORM_GROUP`
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

/// `ShaderPack::INSTANCE_GROUP`
@group(1) @binding(0)
var<storage, read> instance_data: array<InstanceData>;

/// `ShaderPack::TEXTURE_GROUP`
@group(2) @binding(0)
var texture: texture_2d<f32>;
@group(2) @binding(1)
var texture_sampler: sampler;

struct Uniforms {
    camera: mat4x4f,
};

struct InstanceData {
    mix_color: vec4f,
    pos: vec3f,
    _padding: f32,
    tex_coords_off: vec2f,
    tex_coords_size: vec2f,
    scale: vec2f,
    rotation: f32,
    _padding: f32,
}

var<private> v_positions: array<vec2f, 6> = array<vec2f, 6>(
    vec2f(-0.5, -0.5),
    vec2f(0.5, -0.5),
    vec2f(-0.5, 0.5),
    vec2f(0.5, -0.5),
    vec2f(0.5, 0.5),
    vec2f(-0.5, 0.5),
);

var<private> v_texcoords: array<vec2f, 6> = array<vec2f, 6>(
    vec2f(0.0, 0.0),
    vec2f(1.0, 0.0),
    vec2f(0.0, 1.0),
    vec2f(1.0, 0.0),
    vec2f(1.0, 1.0),
    vec2f(0.0, 1.0),
);


struct VertexOut {
    @builtin(position) position: vec4f,
    @location(0) tex_coord: vec2f,
    @location(1) mix_color: vec4f,
};

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32, @builtin(instance_index) i_idx: u32) -> VertexOut {
    var out: VertexOut;

    let instance = instance_data[i_idx];
    let theta = instance.rotation;
    let c = cos(theta);
    let s = sin(theta);
    let rotation_matrix = mat2x2f(c, s, -s, c);

    let v_pos = v_positions[v_idx];
    out.position = vec4f(vec3f((rotation_matrix * (instance.scale * v_pos)), 0.0) + instance.pos, 1.0);
    out.position = uniforms.camera * out.position;

    out.tex_coord = instance.tex_coords_off + instance.tex_coords_size * v_texcoords[v_idx];

    out.mix_color = instance.mix_color;

    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let tex = textureSample(texture, texture_sampler, in.tex_coord);
    if (tex.a == 0.0) {discard;}
    let alpha_mul =  tex.a * tex.rgb; // 👈 assumes texture is non-premultiplied
    return vec4f(mix(alpha_mul, tex.a * in.mix_color.rgb, in.mix_color.a), tex.a);
}
