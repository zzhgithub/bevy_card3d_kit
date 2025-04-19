# Bevy Card3d Kit

# 使用方法

1. 引入插件

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

```

2. 建立数据类型 并实现正反面的方法

```rust
#[derive(Component, Clone)]
pub struct CardInfo {
    pub name: String,
}

impl CardMaterialGetter for CardInfo {
    fn get_face_mal(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            base_color_texture: Some(asset_server.load(format!("cards/{}.png", self.name))),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        })
    }

    fn get_back_mal(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            base_color_texture: Some(asset_server.load(format!("cards/{}.png", "back"))),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        })
    }
}


pub struct SimplePlugin;

impl Plugin for SimplePlugin {
    fn build(&self, app: &mut App) {
        bind_card_render::<CardInfo>(app);
    }
}
```

3. 之后即可使用

```rust
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
        Rotating,
        CardInfo {
            name: "NAAI-A-001".to_string(),
        },
        Card {
            origin: Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
        },
    ));
}
```

# 示例

| example      | desc        |
|--------------|-------------|
| simple       | 基础使用        |
| hand_card    | 手牌          |
| zone         | 场地基础        |
| card_on_zone | 卡片放置到不同的场地上 |

simple.rs

https://github.com/user-attachments/assets/892158c5-2788-4906-a22a-7fca0f820ec9

hand_card.rs

https://github.com/user-attachments/assets/124294e4-1d45-40a7-8c49-8dcf29eaf198

zone.rs
![](doc/zone.png)

card_on_zone.rs 简单的实例 拖拽到不同的位置进行不同的处理。

https://github.com/user-attachments/assets/4490abbf-29ee-4af9-824a-74af213052c3

# 版本

# 鸣谢