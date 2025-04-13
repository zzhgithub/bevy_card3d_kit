use bevy::picking::prelude::MeshPickingPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use card3d_kit::prelude::{
    Card3DConfig, Card3DPlugins, HAND_CARD_LEVEL, HandCard, Moveable, SharkCamera, spawn_card,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, Card3DPlugins))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

// 初始化方法
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

    let face_image = asset_server.load(format!("cards/{}.png", "NAAI-A-001"));

    spawn_card(
        &mut commands,
        &mut materials,
        &mut meshes,
        face_image.clone(),
        face_image.clone(),
        Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
        card3d_config.clone(),
        (HandCard, Moveable),
    );
}
