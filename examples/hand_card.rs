mod helpers;
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::color::palettes::css::{GOLD, LIGHT_SKY_BLUE};
use bevy::math::Vec3;
use bevy::pbr::PointLight;
use bevy::prelude::*;
use bevy_card3d_kit::highlight::Highlight;
use bevy_card3d_kit::prelude::{
    Card, Card3DPlugins, CardLine, HAND_CARD_LEVEL, HandCard, Moveable, SharkCamera,
};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use helpers::*;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins, SimplePlugin))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
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
        .spawn(CardLine {
            transform: Transform::from_xyz(0.0, -6.7, HAND_CARD_LEVEL),
            card_list: vec![],
        })
        .id();

    // 加载手卡
    card_list.iter().enumerate().for_each(|(index, name)| {
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
            Highlight {
                color: if index < 2 {
                    Color::Srgba(GOLD).with_alpha(0.3)
                } else {
                    Color::Srgba(LIGHT_SKY_BLUE).with_alpha(0.3)
                },
            },
        ));
    });
}
