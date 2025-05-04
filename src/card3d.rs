use crate::card::card_material::CardMaterialPlugin;
use crate::highlight::HighlightPlugin;
use crate::prelude::card_namer::CardNamerPlugin;
use crate::prelude::{HandCardPlane, HandCardPlugin};
#[cfg(feature = "image_preview")]
use crate::preview_plugins::PreviewPlugins;
use crate::tween::ExtTweenPlugins;
use crate::zone::ZonePlugin;
use bevy::asset::embedded_asset;
use bevy::prelude::*;
use bevy_tween::DefaultTweenPlugins;
use std::marker::PhantomData;

#[derive(Resource, Copy, Clone)]
pub struct Card3DConfig {
    pub width: f32,
    pub height: f32,
    pub radius: f32,
    pub thick: f32,
}

impl Default for Card3DConfig {
    fn default() -> Self {
        Self {
            width: 3. / 1.4,
            height: 3.,
            radius: 0.05,
            thick: 0.01,
        }
    }
}

/// 主要的插件
pub struct Card3DPlugins;
impl Plugin for Card3DPlugins {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "", "assets/shaders/crack.png");
        app.add_plugins((
            DefaultTweenPlugins,
            CardNamerPlugin,
            CardMaterialPlugin,
            MeshPickingPlugin,
            HandCardPlugin,
            ExtTweenPlugins,
            crate::prelude::MoveCardPlugin::<HandCardPlane> {
                _phantom: PhantomData,
            },
            ZonePlugin,
            HighlightPlugin,
        ))
        .init_resource::<Card3DConfig>();
        #[cfg(feature = "image_preview")]
        app.add_plugins(PreviewPlugins);
    }
}
