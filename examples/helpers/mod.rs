use bevy::asset::{AssetServer, Assets, Handle};
use bevy::color::Color;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy_card3d_kit::prelude::{CardMaterialGetter, bind_card_render};

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

    #[cfg(feature = "image_preview")]
    fn get_id(&self) -> String {
        self.name.clone()
    }
}

pub struct SimplePlugin;

impl Plugin for SimplePlugin {
    fn build(&self, app: &mut App) {
        bind_card_render::<CardInfo>(app);
    }
}
