use crate::card::card_material::CardMaterial;
use crate::card::card_mesh::gen_card_mesh_list;
use crate::card::card_state::{CardState, calculate_transform};
use crate::card3d::Card3DConfig;
#[cfg(feature = "image_preview")]
use crate::preview_plugins::ImagePreview;
#[cfg(feature = "image_preview")]
use crate::preview_plugins::preview_on_click;
use crate::zone::events::CardOnCard;
use bevy::asset::AssetPath;
use bevy::asset::io::AssetSourceId;
use bevy::prelude::*;
use bevy_mod_outline::{InheritOutline, OutlineStencil, OutlineVolume};
use bevy_tween::tween::AnimationTarget;
use std::path::Path;

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
    fn get_face_mal(&self) -> String;
    /// 背面素材
    fn get_back_mal(&self) -> String;

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
    query_card: Query<(Entity, &Card, &T, Option<&CardState>), Added<T>>,
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
            .insert(Mesh3d::default())
            // 计算新的位置
            .insert(OutlineVolume {
                visible: false,
                width: 10.0,
                ..Default::default()
            })
            .insert(OutlineStencil { ..default() })
            .insert(calculate_transform(card.origin.clone(), opt_state.cloned()))
            .insert(Visibility::default())
            .insert(AnimationTarget)
            .with_children(|parent| {
                // 加载outline的Mesh
                parent.spawn((
                    Mesh3d(mesh_list.clone().3),
                    Transform::default(),
                    InheritOutline,
                ));
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
                            MeshMaterial3d(
                                card_materials.add(CardMaterial {
                                    gray_scale: 0.0,
                                    crack_scale: 0.0,
                                    base_color_texture: asset_server.load(t.get_face_mal()),
                                    crack_texture: asset_server.load(
                                        AssetPath::from(
                                            Path::new("bevy_card3d_kit")
                                                .join("assets/shaders/crack.png"),
                                        )
                                        .with_source(AssetSourceId::from("embedded")),
                                    ),
                                }),
                            ),
                        ))
                        .observe(deal_drop_card_on_zone);
                }
                // 背面
                for (mesh_handle, trans) in mesh_list.clone().2 {
                    parent
                        .spawn((
                            Mesh3d(mesh_handle.clone()),
                            trans.clone(),
                            MeshMaterial3d(materials.add(StandardMaterial {
                                base_color: Color::WHITE,
                                unlit: true,
                                base_color_texture: Some(asset_server.load(t.get_back_mal())),
                                alpha_mode: AlphaMode::Blend,
                                ..Default::default()
                            })),
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
