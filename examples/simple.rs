mod helpers;

use bevy::prelude::*;
use bevy_card3d_kit::prelude::{Card, Card3DPlugins, HAND_CARD_LEVEL, SharkCamera};
use helpers::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins, SimplePlugin))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_entities)
        .run();
}

#[derive(Component, Clone)]
pub struct Rotating;

// 初始化方法
fn setup(mut commands: Commands) {
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
    commands.spawn((
        Rotating,
        CardInfo {
            name: "NAAI-A-001".to_string(),
        },
        Card {
            origin: Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
        },
    ));
}

fn rotate_entities(time: Res<Time>, mut query: Query<&mut Transform, With<Rotating>>) {
    for mut transform in &mut query {
        // 每秒旋转 90 度（PI/2 弧度）
        let rotation_speed = std::f32::consts::PI / 2.0;
        transform.rotate_y(rotation_speed * time.delta().as_secs_f32());
    }
}
