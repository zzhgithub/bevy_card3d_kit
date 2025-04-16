use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::pbr::PointLight;
use bevy::prelude::{
    AssetServer, Camera3d, Commands, Mesh, Res, ResMut, StandardMaterial, Transform, default,
};
use bevy_card3d_kit::prelude::{
    Card3DConfig, Card3DPlugins, HAND_CARD_LEVEL, HandCard, Moveable, SharkCamera,
    calculate_hand_positions, spawn_card,
};
use std::f32::consts::PI;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    card3d_config: Res<Card3DConfig>,
    asset_server: Res<AssetServer>,
) {
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
    let back_image = asset_server.load(format!("cards/{}.png", "back"));

    // 加载手卡
    hand_positions
        .iter()
        .enumerate()
        .for_each(|(index, hand_position)| {
            spawn_card(
                &mut commands,
                &mut materials,
                &mut meshes,
                asset_server.load(format!("cards/{}.png", card_list.get(index).unwrap())),
                back_image.clone(),
                hand_position.clone(),
                card3d_config.clone(),
                (HandCard, Moveable),
            );
        });
}
