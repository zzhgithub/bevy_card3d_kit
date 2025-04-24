use bevy::app::App;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub struct CardMaterialPlugin;

impl Plugin for CardMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CardMaterial>::default());
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct CardMaterial {
    #[uniform(0)]
    pub grayscale: f32,

    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Handle<Image>,
}

impl Material for CardMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/card_material.wgsl".into()
    }
}
