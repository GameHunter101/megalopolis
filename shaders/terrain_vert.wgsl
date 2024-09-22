struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
}

struct ModelData {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) vert_pos: vec3<f32>,
}

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: Camera;

@group(0) @binding(0)
var height_map: texture_2d<f32>;

@group(0) @binding(2)
var river_map: texture_2d<f32>;


fn convolve(coords: vec2i) -> vec2f {
    var sobel_y = mat3x3(1.0,2.0,1.0, 0.0,0.0,0.0, -1.0, -2.0, -1.0);
    var sobel_x = mat3x3(1.0,0.0,-1.0, 2.0,0.0,-2.0, 1.0, 0.0, -1.0);
    var grad = vec2f(0.0);

    for (var i = -1; i < 2; i++) {
        for (var j = -1; j < 2; j++) {
            let new_coords = coords + vec2i(i, j);
            grad.x += sobel_x[j+1][i+1] * textureLoad(height_map, vec2i(new_coords), 0).y;
            grad.y += sobel_y[j+1][i+1] * textureLoad(height_map, vec2i(new_coords), 0).y;
        }
    }

    return grad;
}

const TERRAIN_AMPLITUDE: f32 = 2.0;

fn sample(coords: vec2f) -> f32 {
    return TERRAIN_AMPLITUDE * textureLoad(height_map, vec2i(coords), 0).y;
}

@vertex
fn main(model: VertexInput, model_data: ModelData) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        model_data.model_matrix_0,
        model_data.model_matrix_1,
        model_data.model_matrix_2,
        model_data.model_matrix_3,
    );

    let tex_coords = model.tex_coords + vec2f(1.0, 1.0);

    var out:VertexOutput;

    let height_map_offset = vec4f(0.0, sample(tex_coords), 0.0, 0.0);

    let world_position = model_matrix * (vec4<f32>(model.position,1.0) + height_map_offset);
    let vert_pos = camera.view_proj * world_position;

    let vertex_spacing = 20.0 / (f32(textureDimensions(height_map).x) - 2.0);

    out.clip_position = vert_pos;
    out.tex_coords = model.tex_coords;
    /* out.normal = normalize(cross(
        vec3f(vert_pos.x, sample(tex_coords + vec2f(0.0, 1.0)), vert_pos.z + vertex_spacing) - vert_pos.xyz,
        vec3f(vert_pos.x + vertex_spacing, sample(tex_coords + vec2f(1.0, 0.0)), vert_pos.z) - vert_pos.xyz,
    )); */

    let north_vertex = vec3f(world_position.x, sample(tex_coords + vec2f(0.0,1.0)), world_position.z + vertex_spacing);
    let east_vertex = vec3f(world_position.x + vertex_spacing, sample(tex_coords + vec2f(1.0,0.0)), world_position.z);

    out.normal = normalize(cross(north_vertex - world_position.xyz, east_vertex - world_position.xyz));

    out.vert_pos = world_position.xyz;

    return out;
}
