pub mod frame_config;
pub mod frame_config_data;

use crate::card::card_state::CardState;
use crate::card3d::Card3DConfig;
use crate::editor::frame_config::FrameConfig;
use crate::editor::frame_config_data::FrameConfigData;
use crate::prelude::card_mesh::gen_card_mesh_list;
use crate::prelude::card_state::calculate_transform;
use crate::prelude::*;
use bevy::app::App;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

/// 编辑器相关代码

#[derive(Resource, Clone)]
pub struct FrameConfigResource(pub Handle<FrameConfig>);

#[derive(Resource, Clone)]
pub struct FrameConfigPath(String);

impl Default for FrameConfigPath {
    fn default() -> Self {
        Self("assets/default.frame.ron".to_string())
    }
}

/// 表示使用这个配置文件的方式渲染
#[derive(Component, Clone, Default)]
pub struct CardByConfig(pub Option<Handle<FrameConfig>>);

pub struct CardEditor;

impl Plugin for CardEditor {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::new(&["frame.ron"]));
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    frame_config_path: Res<FrameConfigPath>,
    asset_server: Res<AssetServer>,
) {
    // 获取默认配置
    commands.insert_resource(FrameConfigResource(
        asset_server.load(frame_config_path.clone().0),
    ));
}

// 可以从外部加载的Config
// 通过配置来处理数据

fn render_by_card_frame(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &Card,
            &CardByConfig,
            &FrameConfigData,
            Option<&CardState>,
        ),
        Added<Card>,
    >,
    asset_server: Res<AssetServer>,
    frame_config_resource: Res<FrameConfigResource>,
    frame_config_assets: Res<Assets<FrameConfig>>,
    card3d_config: Res<Card3DConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_list = gen_card_mesh_list(
        &mut meshes,
        card3d_config.width,
        card3d_config.height,
        card3d_config.radius,
        card3d_config.thick,
    );

    for (entity, card, card_by_config, frame_config_data, opt) in query.iter() {
        // 如果不存在获取默认逻辑
        let handle = if let Some(frame_config_handle) = card_by_config.clone().0 {
            frame_config_handle
        } else {
            frame_config_resource.clone().0
        };

        if let Some(frame_config) = frame_config_assets.get(handle.id()) {
            // 找到配置项
            commands
                .entity(entity)
                .insert(calculate_transform(card.origin.clone(), opt.cloned()))
                .insert(Visibility::default())
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
                                MeshMaterial3d(materials.add(Color::WHITE)),
                            ))
                            .observe(deal_drop_card_on_card);
                    }
                    //TODO 怎么定义背面的数据呢？
                    for (mesh_handle, trans) in mesh_list.clone().2 {
                        parent
                            .spawn((
                                Mesh3d(mesh_handle.clone()),
                                trans.clone(),
                                MeshMaterial3d(materials.add(Color::WHITE)),
                            ))
                            .observe(deal_drop_card_on_card);
                    }

                    // 处理贴花！
                });
        }
    }
}
