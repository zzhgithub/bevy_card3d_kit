mod helpers;
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::math::Vec3;
use bevy::pbr::PointLight;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::{
    Card, Card3DPlugins, HAND_CARD_LEVEL, HandCard, Moveable, SharkCamera,
    calculate_hand_positions,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::PI;

use helpers::*;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins, SimplePlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // 相机
    commands.spawn((
        SharkCamera,
        Camera3d::default(),
        Transform::from_xyz(0., 0., 25.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 光源
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));
    let card_list = [
        "NAAI-A-001",
        "NAAI-A-001",
        "NAAI-A-001",
        "S001-A-001",
        "S001-A-001",
        // "S001-A-001",
    ];
    let hand_positions =
        calculate_hand_positions(card_list.len(), 0.0, 200., PI / 4., HAND_CARD_LEVEL, -6.7);

    // 加载手卡
    hand_positions
        .iter()
        .enumerate()
        .for_each(|(index, hand_position)| {
            commands.spawn((
                Card {
                    origin: hand_position.clone(),
                },
                CardInfo {
                    name: card_list[index].to_string(),
                },
                Moveable,
                HandCard,
            ));
        });
}
