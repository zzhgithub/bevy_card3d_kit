# Bevy Card3d Kit

# 使用方法
```rust
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // 添加这个插件
            Card3DPlugins))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_entities)
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
    
    // 可以使用这个创建卡片实体了！
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
```
# 示例
|example|desc|
|------|-----|
|simple|基础使用|
|hand_card|手牌|



# 版本

# 鸣谢