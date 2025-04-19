use crate::prelude::card_namer::CardNamerPlugin;
use crate::prelude::{HandCardPlane, HandCardPlugin};
use crate::tween::ExtTweenPlugins;
use crate::zone::ZonePlugin;
use bevy::prelude::*;
use bevy_tween::DefaultTweenPlugins;
use std::marker::PhantomData;
#[cfg(feature = "image_preview")]
use crate::preview_plugins::PreviewPlugins;

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
        app.add_plugins((
            DefaultTweenPlugins,
            CardNamerPlugin,
            MeshPickingPlugin,
            HandCardPlugin,
            ExtTweenPlugins,
            crate::prelude::MoveCardPlugin::<HandCardPlane> {
                _phantom: PhantomData,
            },
            ZonePlugin,
        ))
        .init_resource::<Card3DConfig>();
        #[cfg(feature = "image_preview")]
        app.add_plugins(PreviewPlugins);
    }
}
