use bevy::prelude::*;
use bevy_card3d_kit::prelude::Card3DPlugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Card3DPlugins))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // 通过某种方法加载资源
}
