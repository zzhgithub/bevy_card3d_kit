use bevy::asset::{Assets, Handle};
use bevy::prelude::*;
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
    let center = Transform::from_xyz(0.0, 0.0, thick / 2.);
    let back = Transform::from_xyz(0.0, 0.0, -thick / 2.)
        .with_rotation(Quat::from_axis_angle(Vec3::Y, PI));
    // 加载一组的shape

    let frames = [
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            right_top,
        ),
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            right_bottom,
        ),
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            left_top,
        ),
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            left_bottom,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(a, radius)),
                thick,
            )),
            top,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(a, radius)),
                thick,
            )),
            bottom,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(radius, b)),
                thick,
            )),
            left,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(radius, b)),
                thick,
            )),
            right,
        ),
    ];

    // 正面主要
    let content = [(meshes.add(Rectangle::from_size(Vec2::new(a, b))), center)];
    let back_side = [(meshes.add(Rectangle::from_size(Vec2::new(a, b))), back)];

    (frames, content, back_side)
}
