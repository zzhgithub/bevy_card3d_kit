use bevy::picking::prelude::MeshPickingPlugin;
use bevy::prelude::*;
use card3d_kit::prelude::{Card3DConfig, Card3DPlugins, HAND_CARD_LEVEL, SharkCamera, spawn_card};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, Card3DPlugins))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_entities)
        .run();
}

#[derive(Component, Clone)]
pub struct Rotating;

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

    let face_image = asset_server.load(format!("cards/{}.png", "NAAI-A-001"));
    let back_image = asset_server.load(format!("cards/{}.png", "back"));
    spawn_card(
        &mut commands,
        &mut materials,
        &mut meshes,
        face_image.clone(),
        back_image.clone(),
        Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
        card3d_config.clone(),
        Rotating,
    );
}

fn rotate_entities(time: Res<Time>, mut query: Query<&mut Transform, With<Rotating>>) {
    for mut transform in &mut query {
        // 每秒旋转 90 度（PI/2 弧度）
        let rotation_speed = std::f32::consts::PI / 2.0;
        transform.rotate_y(rotation_speed * time.delta().as_secs_f32());
    }
}
