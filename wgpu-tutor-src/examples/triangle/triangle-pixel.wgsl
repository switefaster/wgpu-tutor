struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@group(0)
@binding(0)
var<uniform> u_projection: mat4x4<f32>;

@vertex
fn vs_main(@location(0) position: vec2<f32>, @location(1) color: vec3<f32>) -> VertexOut {
    var out: VertexOut;
    out.position = u_projection * vec4<f32>(position, 0.0, 1.0);
    out.color = color;
    return out;
}

@fragment
fn fs_main(pin: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(pin.color, 1.0);
}