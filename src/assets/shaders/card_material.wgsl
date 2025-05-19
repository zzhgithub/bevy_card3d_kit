#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material: f32;
@group(2) @binding(1) var<uniform> u_time: f32;
@group(2) @binding(2) var base_color_texture: texture_2d<f32>;
@group(2) @binding(3) var base_color_sampler: sampler;
@group(2) @binding(4) var crack_texture: texture_2d<f32>;
@group(2) @binding(5) var crack_sampler: sampler;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let color = textureSample(base_color_texture, base_color_sampler, mesh.uv);

    let gray = dot(color.rgb, vec3(0.299, 0.587, 0.114));
    let gray_color = vec3(gray);

    // 混合彩色与灰色，根据 grayscale_amount 渐变
    let final_color = mix(color.rgb, gray_color, clamp(material, 0.0, 1.0));

    // 碎片化
    let crack_color = textureSample(crack_texture, crack_sampler, mesh.uv);

    let final_color2 = mix(final_color.rgb, crack_color.rgb, clamp(u_time, 0.0, 1.0));

    return vec4(final_color2, color.a);
}