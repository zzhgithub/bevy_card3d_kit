use bevy::asset::{Assets, Handle};
use bevy::prelude::*;
use bevy_mod_outline::{GenerateOutlineNormalsSettings, OutlineMeshExt};
use std::f32::consts::PI;

/// gen_card_mesh_list 生成3d卡片的mesh列表的方法
pub fn gen_card_mesh_list(
    meshes: &mut ResMut<Assets<Mesh>>,
    width: f32,
    height: f32,
    radius: f32,
    thick: f32,
) -> (
    [(Handle<Mesh>, Transform); 8],
    [(Handle<Mesh>, Transform); 1],
    [(Handle<Mesh>, Transform); 1],
    Handle<Mesh>,
) {
    // 四个 扇形 四个长方形  一个中央的部分
    let a: f32 = width - 2.0 * radius;
    let b: f32 = height - 2.0 * radius;

    // 四个角的坐标
    let right_top = Transform::from_xyz(a / 2.0, b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, -PI / 4.));
    let right_bottom = Transform::from_xyz(a / 2.0, -b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, -PI / 4. - PI / 2.));

    let left_top = Transform::from_xyz(-a / 2.0, b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, PI / 4.));

    let left_bottom = Transform::from_xyz(-a / 2.0, -b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, PI / 4. + PI / 2.));

    // 四个边框的坐标
    let right = Transform::from_xyz((a + radius) / 2.0, 0.0, 0.0);
    let left = Transform::from_xyz(-(a + radius) / 2.0, 0.0, 0.0);
    let top = Transform::from_xyz(0.0, (b + radius) / 2.0, 0.0);
    let bottom = Transform::from_xyz(0.0, -(b + radius) / 2.0, 0.0);
    // 中心的坐标
    let center =
        Transform::from_xyz(0.0, 0.0, thick / 2.).with_rotation(Quat::from_axis_angle(Vec3::Y, PI));
    let back = Transform::from_xyz(0.0, 0.0, -thick / 2.);
    // 加载一组的shape

    let circular_sector = Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)
        .mesh()
        .build();

    let top_mesh = Extrusion::new(Rectangle::from_size(Vec2::new(a, radius)), thick)
        .mesh()
        .build();
    let bottom_mesh = Extrusion::new(Rectangle::from_size(Vec2::new(a, radius)), thick)
        .mesh()
        .build();

    let left_mesh = Extrusion::new(Rectangle::from_size(Vec2::new(radius, b)), thick)
        .mesh()
        .build();

    let right_mesh = Extrusion::new(Rectangle::from_size(Vec2::new(radius, b)), thick)
        .mesh()
        .build();
    let frames = [
        (meshes.add(circular_sector.clone()), right_top),
        (meshes.add(circular_sector.clone()), right_bottom),
        (meshes.add(circular_sector.clone()), left_top),
        (meshes.add(circular_sector.clone()), left_bottom),
        (meshes.add(top_mesh), top),
        (meshes.add(bottom_mesh), bottom),
        (meshes.add(left_mesh), left),
        (meshes.add(right_mesh), right),
    ];

    let content_mesh = Cuboid::new(a, b, thick / 2.0).mesh().build();
    // 正面主要
    let content = [(meshes.add(content_mesh.clone()), center)];
    let back_side = [(meshes.add(content_mesh.clone()), back)];

    let mut outline_mesh = Cuboid::new(a + 2.0 * radius, b + 2.0 * radius, thick)
        .mesh()
        .build();
    outline_mesh
        .generate_outline_normals(&GenerateOutlineNormalsSettings::default())
        .unwrap();

    (frames, content, back_side, meshes.add(outline_mesh))
}
