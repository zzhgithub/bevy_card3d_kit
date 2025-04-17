use bevy::app::{App, Startup};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::color::Color;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::{Card3DPlugins, SharkCamera};
use bevy_card3d_kit::zone::{ZoneBuilder, ZoneMaterialGetter, render_zone};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机
    commands.spawn((
        SharkCamera,
        Camera3d::default(),
        Transform::from_xyz(0., 0., 15.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 光源
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    render_zone(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(0.0, 0.0, 0.0),
        gen_zone_builder_ver(10, 10, 1.2),
    );
}

#[derive(Clone, Debug, Component)]
enum CardZone {
    TypeA,
    TypeB,
}

impl ZoneMaterialGetter for CardZone {
    fn get_mal(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Handle<StandardMaterial> {
        match self {
            CardZone::TypeA => materials.add(Color::BLACK),
            CardZone::TypeB => materials.add(Color::WHITE),
        }
    }
}

/// 行数 列数 和 正方形的宽 生成列表
fn gen_zone_builder_ver(row: usize, col: usize, a: f32) -> Vec<ZoneBuilder<CardZone>> {
    let mut res = Vec::with_capacity((row * col) as usize);
    for r in 0..row {
        for c in 0..col {
            let center = Transform::from_xyz(
                c as f32 * a - (col - 1) as f32 * a / 2.0,
                r as f32 * a - (row - 1) as f32 * a / 2.0,
                0.0,
            );
            res.push(ZoneBuilder {
                size: Vec2::new(a, a),
                center,
                zone_type: if ((r + c * a.floor() as usize) % 2) == 0 {
                    CardZone::TypeB
                } else {
                    CardZone::TypeA
                },
            });
        }
    }

    res
}
