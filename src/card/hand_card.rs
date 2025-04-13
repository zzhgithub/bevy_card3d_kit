use bevy::prelude::*;
use std::f32::consts::PI;

pub const HAND_CARD_LEVEL:f32 = 10.0;


/// 手牌
#[derive(Component, Copy, Clone)]
pub struct HandCard;

/// 手牌操作的平台
#[derive(Component, Copy, Clone)]
pub struct HandCardPlane;

#[derive(Resource, Copy, Clone)]
pub struct HandPlaneConfig(Transform);

impl Default for HandPlaneConfig {
    fn default() -> Self {
        Self(
            Transform::from_xyz(0.0, 0.0, HAND_CARD_LEVEL)
                .with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0)),
        )
    }
}

pub struct HandCardPlugin;

impl Plugin for HandCardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HandPlaneConfig>()
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, hand_plane_config: Res<HandPlaneConfig>) {
    commands.spawn((HandCardPlane, hand_plane_config.0));
}
