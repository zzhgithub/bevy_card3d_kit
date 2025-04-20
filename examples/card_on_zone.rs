mod helpers;

use crate::helpers::{CardInfo, SimplePlugin};
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::color::palettes::css::{GREEN, RED};
use bevy::prelude::*;
use bevy_card3d_kit::prelude::{
    Card, Card3DPlugins, Dragged, HAND_CARD_LEVEL, Moveable, SharkCamera,
};
use bevy_card3d_kit::tween::animation::card_set_on_zone_animation;
use bevy_card3d_kit::zone::events::CardOnZone;
use bevy_card3d_kit::zone::{Zone, ZoneMaterialGetter, bind_zone_render};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::thread::spawn;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins, SimplePlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_observer(card_on_zone)
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

    commands.spawn((
        Card {
            origin: Transform::from_xyz(0.0, -4.5, HAND_CARD_LEVEL),
        },
        CardInfo {
            name: "S001-A-001".to_string(),
        },
        Moveable,
    ));
}

fn card_on_zone(
    card_on_zone: Trigger<CardOnZone>,
    mut commands: Commands,
    query: Query<&ConditionZone>,
    query_children: Query<&Children>,
    mut query_card: Query<(&mut Card, &Name, &Transform)>,
    query_zone: Query<&Zone>,
) {
    info!("{:?}", card_on_zone.clone());
    if let Ok(zone) = query.get(card_on_zone.zone) {
        match zone {
            ConditionZone::CanSet => {
                commands.entity(card_on_zone.card).remove::<Dragged>();
                if let Ok(children) = query_children.get(card_on_zone.card) {
                    for &child in children.iter() {
                        commands.entity(child).remove::<PickingBehavior>();
                    }
                }
                if let Ok((card, card_name, card_transform)) = query_card.get_mut(card_on_zone.card)
                {
                    if let Ok(zone) = query_zone.get(card_on_zone.zone) {
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
