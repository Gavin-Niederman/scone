// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

//    1
//   / \
//  /   \
// 2-----0

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    
    let x = f32(1 - i32(in_vertex_index)) * 0.5; // Set the x of vertex 0 and 2 to 0.5 and -0.5 respectively
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5; // set the y of vertex 1 to 0.5 and 2 and 0 to -0.5

    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}