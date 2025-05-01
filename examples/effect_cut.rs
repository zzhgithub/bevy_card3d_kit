mod helpers;

use bevy::prelude::*;
use bevy_card3d_kit::prelude::{Card, Card3DPlugins, HAND_CARD_LEVEL, SharkCamera};
use bevy_card3d_kit::tween::card_gray::EffectCut;
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
        .add_systems(Update, (spacebar_system, spacebar_system_b))
        .run();
}

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
        CardInfo {
            name: "NAAI-A-001".to_string(),
        },
        Card {
            origin: Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
        },
    ));
}

fn spacebar_system(
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, (With<Card>, Without<EffectCut>)>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Space) {
        for entity in query.iter() {
            info!("Added");
            commands.entity(entity).insert(EffectCut);
        }
    }
}

fn spacebar_system_b(
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, (With<Card>, With<EffectCut>)>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Space) {
        for entity in query.iter() {
            info!("Added");
            commands.entity(entity).remove::<EffectCut>();
        }
    }
}
