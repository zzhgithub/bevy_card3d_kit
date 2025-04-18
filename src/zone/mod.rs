pub mod events;

use crate::prelude::Card;
use crate::zone::events::CardOnZone;
use bevy::app::App;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        // todo
    }
}

#[derive(Component, Debug, Clone)]
pub struct ZoneBuilder<T: Component + Clone> {
    // 尺寸
    pub size: Vec2,
    // 中心位置
    pub center: Transform,
    pub zone_type: T,
}

pub trait ZoneMaterialGetter {
    fn get_mal(&self, materials: &mut ResMut<Assets<StandardMaterial>>)
    -> Handle<StandardMaterial>;
}

/// 场地根节点
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ZoneParent;

/// 场地
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Zone {
    pub center: Transform,
}

/// 渲染整个场地
pub fn render_zone<T: Component + Clone + ZoneMaterialGetter>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    center: Transform,
    zone_builder_vec: Vec<ZoneBuilder<T>>,
) {
    commands
        .spawn((
            ZoneParent,
            center,
            Visibility::default(),
            Name::new(format!("Zone Parent on {:?}", center.clone())),
        ))
        .with_children(|parent| {
            for zone_builder in zone_builder_vec {
                parent
                    .spawn((
                        Zone {
                            center: zone_builder.center.clone(),
                        },
                        zone_builder.clone().zone_type,
                        Mesh3d(meshes.add(Rectangle::from_size(zone_builder.size))),
                        zone_builder.center,
                        MeshMaterial3d(zone_builder.clone().zone_type.get_mal(&mut materials)),
                    ))
                    .observe(deal_drop_card_on_zone);
            }
        });
}

pub fn deal_drop_card_on_zone(
    drag_drop: Trigger<Pointer<DragDrop>>,
    query_card: Query<Entity, With<Card>>,
    query_zone: Query<Entity, (With<Zone>, Without<Card>)>,
    query: Query<&Parent>,
    mut commands: Commands,
) {
    debug!("Drag drop: {:?}", drag_drop);
    if let Ok(zone_entity) = query_zone.get(drag_drop.target) {
        if let Ok(parent) = query.get(drag_drop.dropped) {
            if let Ok(card_entity) = query_card.get(parent.get()) {
                commands.trigger(CardOnZone {
                    card: card_entity,
                    zone: zone_entity,
                });
            }
        }
    }
}
