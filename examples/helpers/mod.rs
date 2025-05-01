use bevy::prelude::*;
use bevy_card3d_kit::prelude::{CardMaterialGetter, bind_card_render};

#[derive(Component, Clone)]
pub struct CardInfo {
    pub name: String,
}

impl CardMaterialGetter for CardInfo {
    fn get_face_mal(&self) -> String {
        format!("cards/{}.png", self.name)
    }

    fn get_back_mal(&self) -> String {
        format!("cards/{}.png", "back")
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
