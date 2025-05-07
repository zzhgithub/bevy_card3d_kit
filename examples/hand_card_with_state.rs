mod helpers;
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::math::Vec3;
use bevy::pbr::PointLight;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_card3d_kit::prelude::card_state::CardState;
use bevy_card3d_kit::prelude::{
    Card, Card3DPlugins, CardLine, HAND_CARD_LEVEL, HandCard, Moveable, SharkCamera,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use helpers::*;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins, SimplePlugin))
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
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
    let card_line_entity = commands
        .spawn((
            CardLine {
                transform: Transform::from_xyz(0.0, -6.7, HAND_CARD_LEVEL),
                card_list: vec![],
            },
            // CardState {
            //     face_up: false,
            //     vertical: true,
            // },
        ))
        .id();

    // 加载手卡
    card_list.iter().for_each(|name| {
        commands.spawn((
            Card {
                origin: Transform::default(),
            },
            CardInfo {
                name: name.to_string(),
            },
            Moveable,
            HandCard {
                belong_to_card_line: Some(card_line_entity),
            },
        ));
    });

    // 对手卡牌
    let card_list2 = [
        "NAAI-A-001",
        "NAAI-A-001",
        "NAAI-A-001",
        "S001-A-001",
        "S001-A-001",
        // "S001-A-001",
    ];

    let opponent_card_line_entity = commands
        .spawn((
            CardLine {
                transform: Transform::from_xyz(0.0, 6.7, HAND_CARD_LEVEL),
                card_list: vec![],
            },
            CardState {
                face_up: false,
                vertical: true,
            },
        ))
        .id();

    card_list2.iter().for_each(|name| {
        commands.spawn((
            Card {
                origin: Transform::default(),
            },
            CardInfo {
                name: name.to_string(),
            },
            HandCard {
                belong_to_card_line: Some(opponent_card_line_entity),
            },
        ));
    });
}
