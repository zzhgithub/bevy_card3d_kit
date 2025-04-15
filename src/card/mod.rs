use crate::card::card_mesh::gen_card_mesh_list;
use crate::card3d::Card3DConfig;
use crate::prelude::Card;
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::image::Image;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;
use bevy_tween::tween::AnimationTarget;

mod card_mesh;
pub mod core;
pub mod event;
pub mod hand_card;
pub mod move_card;
pub mod card_namer;

/// 生成一个Card实体
pub fn spawn_card<T: Bundle>(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    face_image: Handle<Image>,
    back_image: Handle<Image>,
    transform: Transform,
    config: Card3DConfig,
    bundle: T,
) -> Entity {
    let mesh_list = gen_card_mesh_list(
        meshes,
        config.width,
        config.height,
        config.radius,
        config.thick,
    );

    commands
        .spawn(bundle)
        .insert(Card {
            origin: transform.clone(),
        })
        .insert(Visibility::Inherited)
        .insert(AnimationTarget)
        .insert(transform)
        .with_children(|parent| {
            // 加载黑色边框
            for (mesh_handle, trans) in mesh_list.0 {
                parent.spawn((
                    Mesh3d(mesh_handle.clone()),
                    trans.clone(),
                    MeshMaterial3d(materials.add(Color::BLACK)),
                ));
            }
            // 加载内容
            for (mesh_handle, trans) in mesh_list.1 {
                parent.spawn((
                    Mesh3d(mesh_handle.clone()),
                    trans.clone(),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::WHITE,
                        base_color_texture: Some(face_image.clone()),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..Default::default()
                    })),
                ));
            }
            // 背面
            for (mesh_handle, trans) in mesh_list.2 {
                parent.spawn((
                    Mesh3d(mesh_handle.clone()),
                    trans.clone(),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::WHITE,
                        unlit: true,
                        base_color_texture: Some(back_image.clone()),
                        alpha_mode: AlphaMode::Blend,
                        ..Default::default()
                    })),
                ));
            }
        })
        .id()
}
