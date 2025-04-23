mod helpers;

use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::card_state::CardState;
use bevy_card3d_kit::prelude::{Card, Card3DPlugins, HAND_CARD_LEVEL, SharkCamera};
use bevy_card3d_kit::zone::desk_zone::{DeskCard, DeskZone};
use bevy_card3d_kit::zone::{Zone, ZoneMaterialGetter, bind_zone_render};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use helpers::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins, SimplePlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, spacebar_system)
        .add_plugins(|app: &mut App| {
            bind_zone_render::<ZoneInfo>(app);
        })
        .init_resource::<DeskEntity>()
        .run();
}

#[derive(Resource, Default, Clone)]
pub struct DeskEntity(Option<Entity>);

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
        Text::new("Press `Space` to create a Card\nAnd Click to back to Desk Zone."),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    let desk_entity = commands
        .spawn((
            Zone {
                center: Transform::from_xyz(15., -5., 0.0),
                size: Vec2::new(3.7, 5.),
            },
            DeskZone::default(),
            ZoneInfo::Desk,
            CardState {
                face_up: false,
                vertical: true,
            },
        ))
        .id();
    // 设置值
    commands.insert_resource(DeskEntity(Some(desk_entity)));

    commands
        .spawn((
            CardInfo {
                name: "NAAI-A-001".to_string(),
            },
            Card {
                origin: Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
            },
            DeskCard {
                belongs_to_desk: Some(desk_entity),
            },
        ))
        .observe(observer_click);
}

fn spacebar_system(input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if input.just_pressed(KeyCode::Space) {
        commands
            .spawn((
                CardInfo {
                    name: "NAAI-A-001".to_string(),
                },
                Card {
                    origin: Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL),
                },
            ))
            .observe(observer_click);
    }
}

fn observer_click(
    click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    desk_entity: Res<DeskEntity>,
) {
    if let Some(entity) = desk_entity.0 {
        commands.entity(click.entity()).insert(DeskCard {
            belongs_to_desk: Some(entity),
        });
    }
}

#[derive(Component, Clone)]
enum ZoneInfo {
    Desk,
}

impl ZoneMaterialGetter for ZoneInfo {
    fn get_mal(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        _asset_server: &Res<AssetServer>,
    ) -> Handle<StandardMaterial> {
        materials.add(Color::Srgba(RED))
    }
}
