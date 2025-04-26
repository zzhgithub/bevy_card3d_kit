mod helpers;

use crate::helpers::{CardInfo, SimplePlugin};
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::color::palettes::css::{GREEN, RED};
use bevy::prelude::*;
use bevy_card3d_kit::prelude::{
    Card, Card3DPlugins, CardLine, Dragged, HAND_CARD_LEVEL, HandCard, HandCardChanged, Moveable,
    SharkCamera,
};
use bevy_card3d_kit::tween::animation::card_set_on_zone_animation;
use bevy_card3d_kit::zone::events::CardOnZone;
use bevy_card3d_kit::zone::{Zone, ZoneMaterialGetter, bind_zone_render};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::thread::spawn;
use bevy_inspector_egui::bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins, SimplePlugin))
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_observer(card_on_zone)
        .init_resource::<CardLineEntity>()
        .add_plugins(|app: &mut App| {
            bind_zone_render::<ConditionZone>(app);
        })
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

    commands.spawn((
        Zone {
            size: Vec2::new(5.0, 5.0),
            center: Transform::from_xyz(-3.0, 0.0, 0.0),
        },
        ConditionZone::CanSet,
    ));
    commands.spawn((
        Zone {
            size: Vec2::new(5.0, 5.0),
            center: Transform::from_xyz(3.0, 0.0, 0.0),
        },
        ConditionZone::NotCanSet,
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

    commands.insert_resource(CardLineEntity(Some(card_line_entity)));

    // 加载手卡
    card_list.iter().for_each(|name| {
        commands
            .spawn((
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
            ))
            .observe(observer_card_line);
    });
}

fn card_on_zone(
    card_on_zone: Trigger<CardOnZone>,
    mut commands: Commands,
    query: Query<&ConditionZone>,
    query_children: Query<&Children>,
    mut query_card: Query<(&mut Card, &Name, &Transform, &HandCard)>,
    query_zone: Query<&Zone>,
    mut hand_card_event: EventWriter<HandCardChanged>,
) {
    info!("{:?}", card_on_zone.clone());
    if let Ok(zone) = query.get(card_on_zone.zone) {
        match zone {
            ConditionZone::CanSet => {
                commands
                    .entity(card_on_zone.card)
                    .remove::<Dragged>()
                    .remove::<Moveable>();

                if let Ok(children) = query_children.get(card_on_zone.card) {
                    for child in children.iter() {
                        commands.entity(child).remove::<Pickable>();
                    }
                }
                if let Ok((mut card, card_name, card_transform, hand_card)) =
                    query_card.get_mut(card_on_zone.card)
                {
                    hand_card_event.write(HandCardChanged::Remove {
                        card_entity: card_on_zone.card,
                        card_line_entity: hand_card.belong_to_card_line.unwrap(),
                    });

                    if let Ok(zone) = query_zone.get(card_on_zone.zone) {
                        card.origin = zone.center;
                        card_set_on_zone_animation(
                            card_on_zone.card,
                            &card,
                            &zone,
                            card_transform,
                            card_name,
                            &mut commands,
                        );
                    }
                }
            }
            ConditionZone::NotCanSet => {
                info!("Not CanSet");
            }
        }
    };
}

#[derive(Resource, Default)]
pub struct CardLineEntity(Option<Entity>);

fn observer_card_line(
    on_click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    query: Query<Entity, Without<HandCard>>,
    card_line_entity: Res<CardLineEntity>,
) {
    if let Ok(entity) = query.get(on_click.target()) {
        if let Some(belong) = card_line_entity.0 {
            commands
                .entity(entity)
                .insert(HandCard {
                    belong_to_card_line: Some(belong),
                })
                .insert(Moveable);
        }
    }
}

#[derive(Debug, Clone, Default, Component)]
enum ConditionZone {
    #[default]
    CanSet,
    NotCanSet,
}

impl ZoneMaterialGetter for ConditionZone {
    fn get_mal(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        _asset_server: &Res<AssetServer>,
    ) -> Handle<StandardMaterial> {
        match self {
            ConditionZone::CanSet => materials.add(Color::Srgba(GREEN)),
            ConditionZone::NotCanSet => materials.add(Color::Srgba(RED)),
        }
    }
}
