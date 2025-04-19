mod helpers;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_card3d_kit::prelude::{
    Card, Card3DPlugins, CardMaterialGetter, Dragged, HAND_CARD_LEVEL, Moveable, SharkCamera,
    bind_card_render,
};
use bevy_card3d_kit::zone::events::{CardOnCard, CardOnZone};
use helpers::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins, SimplePlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_observer(card_on_zone)
        .run();
}

// 初始化方法
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
    commands.spawn((
        CardInfo {
            name: "NAAI-A-001".to_string(),
        },
        Card {
            origin: Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
        },
    ));

    commands.spawn((
        CardInfo {
            name: "NAAI-A-001".to_string(),
        },
        Card {
            origin: Transform::from_xyz(0.0, -4.0, HAND_CARD_LEVEL),
        },
        Moveable,
    ));
}

fn card_on_zone(card_on_card: Trigger<CardOnCard>) {
    info!("{:?}", card_on_card.clone());
    // do your own logic here
}
