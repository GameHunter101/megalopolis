struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) vert_pos: vec3<f32>,
}

@group(0) @binding(0)
var height_map: texture_2d<f32>;

@group(0) @binding(1)
var height_sampler: sampler;

@group(0) @binding(2)
var river_map: texture_2d<f32>;

@group(0) @binding(3)
var river_sampler: sampler;

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: Camera;

@fragment
fn main(in: VertexOutput) -> @location(0) vec4<f32> {
    let river_val = textureSample(river_map, river_sampler, in.tex_coords / (f32(textureDimensions(height_map).x) - 2.0)).x;
    let light_position = vec3f(-5.0, 5.0, 7.0);

    let highlight_color = vec3f(1.0);
    let surface_color = mix(vec3f(0.0, 1.0, 0.0), vec3f(0.0, 0.0, 1.0), river_val);
    let cool_color = vec3f(0.0, 0.0, 0.55) + 0.25 * surface_color;
    let warm_color = vec3f(0.3, 0.3, 0.0) + 0.25 * surface_color;

    let vector_to_light = normalize(light_position - in.vert_pos);
    let vector_to_camera = normalize(camera.view_pos.xyz - in.vert_pos);
    let light_contribution = dot(in.normal, vector_to_light);
    let t = (light_contribution + 1.0) / 2.0;
    let r = 2.0 * light_contribution * in.normal - vector_to_light;
    let s = clamp(100.0 * dot(r,vector_to_camera) - 97.0, 0.0, 1.0);

    return vec4f(mix(highlight_color, mix(warm_color, cool_color, 1.0 - t), 1.0 - s), 1.0);
}
