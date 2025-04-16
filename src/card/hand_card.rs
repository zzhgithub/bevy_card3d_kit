use crate::prelude::{Card, Dragged, Moveable};
use crate::tween::animation::play_card_going_back_to_place_animation;
use crate::tween::clear_on_finish::ClearOnFinishExt;
use bevy::math::ops::{cos, sin};
use bevy::prelude::*;
use bevy_tween::combinator::AnimationBuilderExt;
use bevy_tween::prelude::{EaseKind, IntoTarget, TransformTargetStateExt};
use std::f32::consts::PI;
use std::time::Duration;

pub const HAND_CARD_LEVEL: f32 = 10.0;

/// 手牌
#[derive(Component, Copy, Clone)]
pub struct HandCard;

/// 手牌操作的平台
#[derive(Component, Copy, Clone)]
pub struct HandCardPlane;

/// 手牌发生变化事件
#[derive(Event, Debug)]
pub struct HandCardChanged;

#[derive(Resource, Copy, Clone)]
pub struct HandPlaneConfig(Transform);

#[derive(Component, Copy, Clone)]
pub struct HandOnHover;

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
            .add_event::<HandCardChanged>()
            .add_systems(Startup, setup)
            .add_observer(on_hover)
            .add_observer(on_hover_cancel);
    }
}

fn setup(mut commands: Commands, hand_plane_config: Res<HandPlaneConfig>) {
    commands.spawn((HandCardPlane, hand_plane_config.0));
}

/// 计算手牌位置
pub fn calculate_hand_positions(
    card_count: usize,
    center_x: f32,
    base_radius: f32,
    max_angle: f32,
    z: f32,
    base_y: f32,
) -> Vec<Transform> {
    // 不处理card_count == 0 的情况
    let mut vec = Vec::new();

    let radius = base_radius + (card_count - 1) as f32 * 10.0;
    let total_angle = max_angle.min(10.0 * card_count as f32);
    let angle_step = total_angle / 1.0_f32.max((card_count - 1) as f32);
    let start_angle = -total_angle / 2.;

    for i in 0..card_count {
        let angle = start_angle + i as f32 * angle_step;
        let radian = angle.to_radians();
        let x = center_x + radius * sin(radian);
        let mut y = base_y + radius * (1.0_f32 - cos(radian));
        y += radius * 0.1 * (1. - cos(2. * radian));
        vec.push(Transform::from_xyz(x, y, z - 0.001 * i as f32));
    }
    vec
}

/// 对手牌的数据进行 onHover的处理
pub fn on_hover(
    over_trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    query: Query<&Parent>,
    query_transform: Query<
        (&Transform, &Card, &Name),
        (With<HandCard>, With<Moveable>, Without<Dragged>),
    >,
) {
    if let Ok(parent) = query.get(over_trigger.target) {
        if let Ok((card_transform, _card, card_name)) = query_transform.get(parent.get()) {
            let target = parent.get().into_target();
            let mut start = target.transform_state(card_transform.clone());
            let mut end = card_transform.clone().translation;
            end.y += 2.0;
            commands.entity(parent.get()).insert(HandOnHover);
            commands
                .spawn((Name::new(format!("hand card on hovered {}", card_name)),))
                .animation()
                .insert_tween_here(
                    Duration::from_secs_f32(1.1),
                    EaseKind::ExponentialOut,
                    start.translation_to(end),
                );
        }
    }
}
/// 对手牌的数据进行 onHoverCancel的处理
pub fn on_hover_cancel(
    out_trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    query: Query<&Parent>,
    query_transform: Query<
        (&Transform, &Card, &Name),
        (With<HandCard>, With<Moveable>, Without<Dragged>),
    >,
) {
    if let Ok(parent) = query.get(out_trigger.target) {
        if let Ok((card_transform, card, card_name)) = query_transform.get(parent.get()) {
            play_card_going_back_to_place_animation(
                parent.get(),
                card,
                &card_transform,
                card_name,
                &mut commands,
            );
            commands.entity(parent.get()).remove::<HandOnHover>();
        }
    }
}

// TODO 对手牌进行变化时的处理
