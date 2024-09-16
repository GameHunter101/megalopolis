struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@fragment
fn main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4f(in.tex_coords/ 30.0, 0.0, 1.0);

    /* let x_coord = u32(floor(in.tex_coords.x * 128.0));
    let y_coord = u32(floor(in.tex_coords.y * 128.0));
    let vec_index = y_coord / 4u;
    let rem = y_coord % 4u;

    let value = grid[x_coord][vec_index][rem] / 10.0;

    return vec4f(value, value, value, 1.0); */

}
