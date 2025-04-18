use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::prelude::*;
use bevy_card3d_kit::prelude::{Card3DPlugins, SharkCamera};

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

    // todo
}

