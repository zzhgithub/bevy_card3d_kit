use crate::card::card_mesh::gen_card_mesh_list;
use crate::card::card_state::{CardState, calculate_transform};
use crate::card3d::Card3DConfig;
#[cfg(feature = "image_preview")]
use crate::preview_plugins::ImagePreview;
#[cfg(feature = "image_preview")]
use crate::preview_plugins::preview_on_click;
use crate::zone::events::CardOnCard;
use bevy::prelude::*;
use bevy_tween::tween::AnimationTarget;
use crate::card::card_material::CardMaterial;

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
        materials: &mut ResMut<Assets<CardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Handle<CardMaterial>;
    /// 背面素材
    fn get_back_mal(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Handle<StandardMaterial>;

    #[cfg(feature = "image_preview")]
    fn get_id(&self) -> String;
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
    mut card_materials: ResMut<Assets<CardMaterial>>,
    card3d_config: Res<Card3DConfig>,
    query_card: Query<(Entity, &Card, &T, Option<&CardState>), Added<Card>>,
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
    for (card_entity, card, t, opt_state) in query_card.iter() {
        commands
            .entity(card_entity)
            // 计算新的位置
            .insert(calculate_transform(card.origin.clone(), opt_state.cloned()))
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
                            MeshMaterial3d(t.get_face_mal(&mut card_materials, &asset_server)),
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
        #[cfg(feature = "image_preview")]
        commands
            .entity(card_entity)
            .insert(ImagePreview(t.get_id()))
            .observe(preview_on_click);
    }
}

pub fn deal_drop_card_on_zone(
    drag_drop: Trigger<Pointer<DragDrop>>,
    query_card: Query<Entity, With<Card>>,
    query: Query<&ChildOf>,
    mut commands: Commands,
) {
    debug!("Drag drop: {:?}", drag_drop);
    if let Ok(card_bottom) = query.get(drag_drop.target) {
        if let Ok(card_bottom_entity) = query_card.get(card_bottom.parent()) {
            if let Ok(parent) = query.get(drag_drop.dropped) {
                if let Ok(card_top_entity) = query_card.get(parent.parent()) {
                    commands.trigger(CardOnCard {
                        bottom_card: card_bottom_entity,
                        top_card: card_top_entity,
                    });
                }
            }
        }
    }
}
