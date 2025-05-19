use bevy::app::App;
use bevy::asset::{load_internal_asset, weak_handle};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub(crate) const SHADER_HANDLE: Handle<Shader> =
    weak_handle!("f53f6cb3-2e4f-4a79-8244-6c303de83d6d");
pub struct CardMaterialPlugin;

impl Plugin for CardMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SHADER_HANDLE,
            "../assets/shaders/card_material.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(MaterialPlugin::<CardMaterial>::default());
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct CardMaterial {
    #[uniform(0)]
    pub gray_scale: f32,

    #[uniform(1)]
    pub crack_scale: f32,

    #[texture(2)]
    #[sampler(3)]
    pub base_color_texture: Handle<Image>,

    #[texture(4)]
    #[sampler(5)]
    pub crack_texture: Handle<Image>,
}

impl Material for CardMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }
}
