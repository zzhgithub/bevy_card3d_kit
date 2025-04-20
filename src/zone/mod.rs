pub mod desk_zone;
pub mod events;

use crate::prelude::Card;
use crate::zone::desk_zone::when_added_to_desk;
use crate::zone::events::CardOnZone;
use bevy::app::App;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, when_added_to_desk);
    }
}
pub trait ZoneMaterialGetter {
    fn get_mal(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Handle<StandardMaterial>;
}

/// 场地根节点
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ZoneParent;

/// 场地
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Zone {
    pub center: Transform,
    pub size: Vec2,
}

/// 绑定场地渲染的类
pub fn bind_zone_render<T>(app: &mut App)
where
    T: Component + Clone + ZoneMaterialGetter,
{
    app.add_systems(Update, spawn_zone::<T>);
}

fn spawn_zone<T>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Zone, &T), Added<Zone>>,
) where
    T: Component + Clone + ZoneMaterialGetter,
{
    for (zone_entity, &zone, t) in query.iter() {
        commands
            .entity(zone_entity)
            .insert((
                zone.center.clone(),
                Mesh3d(meshes.add(Rectangle::from_size(zone.size))),
                MeshMaterial3d(t.get_mal(&mut materials, &asset_server)),
            ))
            .observe(deal_drop_card_on_zone);
    }
}

pub fn deal_drop_card_on_zone(
    drag_drop: Trigger<Pointer<DragDrop>>,
    query_card: Query<Entity, With<Card>>,
    query_zone: Query<Entity, (With<Zone>, Without<Card>)>,
    query: Query<&Parent>,
    mut commands: Commands,
) {
    info!("Drag drop: {:?}", drag_drop);
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
