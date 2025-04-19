use crate::card::card_mesh::gen_card_mesh_list;
use crate::card3d::Card3DConfig;
use crate::zone::Zone;
use crate::zone::events::{CardOnCard, CardOnZone};
use bevy::prelude::*;
use bevy_tween::tween::AnimationTarget;

#[derive(Component, Debug)]
pub struct Card {
    pub origin: Transform,
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub enum Dragged {
    /// The card is being actively dragged
    #[default]
    Actively,
    /// The card is no longer being dragged and is currently going back to its origin place
    GoingBackToPlace,
}

/// A tag added to hovered cards, indicating that they're hovered over
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Hovered;
pub trait CardMaterialGetter {
    /// 正面素材
    fn get_face_mal(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Handle<StandardMaterial>;
    /// 背面素材
    fn get_back_mal(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Handle<StandardMaterial>;
}
// 加载绑定 特定数据类型的卡片
pub fn bind_card_render<T>(app: &mut App)
where
    T: Component + Clone + CardMaterialGetter,
{
    app.add_systems(Update, render_added_card::<T>);
}

fn render_added_card<T>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    card3d_config: Res<Card3DConfig>,
    query_card: Query<(Entity, &Card, &T), Added<Card>>,
    asset_server: Res<AssetServer>,
) where
    T: Component + Clone + CardMaterialGetter,
{
    let mesh_list = gen_card_mesh_list(
        &mut meshes,
        card3d_config.width,
        card3d_config.height,
        card3d_config.radius,
        card3d_config.thick,
    );
    for (card_entity, card, t) in query_card.iter() {
        commands
            .entity(card_entity)
            .insert(card.origin.clone())
            .insert(Visibility::default())
            .insert(AnimationTarget)
            .with_children(|parent| {
                // 加载黑色边框
                for (mesh_handle, trans) in mesh_list.clone().0 {
                    parent.spawn((
                        Mesh3d(mesh_handle.clone()),
                        trans.clone(),
                        MeshMaterial3d(materials.add(Color::BLACK)),
                    ));
                }
                // 加载内容
                for (mesh_handle, trans) in mesh_list.clone().1 {
                    parent
                        .spawn((
                            Mesh3d(mesh_handle.clone()),
                            trans.clone(),
                            MeshMaterial3d(t.get_face_mal(&mut materials, &asset_server)),
                        ))
                        .observe(deal_drop_card_on_zone);
                }
                // 背面
                for (mesh_handle, trans) in mesh_list.clone().2 {
                    parent
                        .spawn((
                            Mesh3d(mesh_handle.clone()),
                            trans.clone(),
                            MeshMaterial3d(t.get_back_mal(&mut materials, &asset_server)),
                        ))
                        .observe(deal_drop_card_on_zone);
                }
            });
    }
}

pub fn deal_drop_card_on_zone(
    drag_drop: Trigger<Pointer<DragDrop>>,
    query_card: Query<Entity, With<Card>>,
    query: Query<&Parent>,
    mut commands: Commands,
) {
    debug!("Drag drop: {:?}", drag_drop);
    if let Ok(card_bottom) = query.get(drag_drop.target) {
        if let Ok(card_bottom_entity) = query_card.get(card_bottom.get()) {
            if let Ok(parent) = query.get(drag_drop.dropped) {
                if let Ok(card_top_entity) = query_card.get(parent.get()) {
                    commands.trigger(CardOnCard {
                        bottom_card: card_bottom_entity,
                        top_card: card_top_entity,
                    });
                }
            }
        }
    }
}
