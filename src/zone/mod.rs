use bevy::app::App;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        // 初始化 场地相关代码
    }
}

#[derive(Component, Debug, Clone)]
pub struct ZoneBuilder<T: Component + Clone> {
    // 尺寸
    pub size: Vec2,
    // 中心位置
    pub center: Transform,
    pub zone_type: T,
}

pub trait ZoneMaterialGetter {
    fn get_mal(&self, materials: &mut ResMut<Assets<StandardMaterial>>)
    -> Handle<StandardMaterial>;
}

/// 场地根节点
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ZoneParent;

/// 场地
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Zone {
    pub center: Transform,
}

/// 渲染整个场地
pub fn render_zone<T: Component + Clone + ZoneMaterialGetter>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    center: Transform,
    zone_builder_vec: Vec<ZoneBuilder<T>>,
) {
    commands
        .spawn((
            ZoneParent,
            center,
            Visibility::default(),
            Name::new(format!("Zone Parent on {:?}", center.clone())),
        ))
        .with_children(|parent| {
            for zone_builder in zone_builder_vec {
                parent.spawn((
                    Zone {
                        center: zone_builder.center.clone(),
                    },
                    zone_builder.clone().zone_type,
                    Mesh3d(meshes.add(Rectangle::from_size(zone_builder.size))),
                    zone_builder.center,
                    MeshMaterial3d(zone_builder.clone().zone_type.get_mal(&mut materials)),
                ));
            }
        });
}
