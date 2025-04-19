use crate::prelude::{Card, Dragged, Moveable};
use crate::tween::animation::play_card_going_back_to_place_animation;
use bevy::math::ops::{cos, sin};
use bevy::prelude::*;
use bevy_tween::combinator::AnimationBuilderExt;
use bevy_tween::prelude::{EaseKind, IntoTarget, TransformTargetStateExt};
use bevy_tween::tween::AnimationTarget;
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
            .add_observer(on_hover_cancel)
            .add_systems(Update, change_hand_cards);
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
            debug!("hand card on hovered {}", card_name);
            let target = parent.get().into_target();
            let mut start = target.transform_state(card_transform.clone());
            let mut end = card_transform.clone().translation;
            end.y += 2.0;
            commands.entity(parent.get()).insert(HandOnHover);
            commands
                .spawn((Name::new(format!("hand card on hovered {}", card_name)),))
                .animation()
                .insert_tween_here(
                    Duration::from_secs_f32(0.01),
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
            debug!("hand card on out {}", card_name);
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

///  对手牌进行变化时的处理
pub fn change_hand_cards(
    mut commands: Commands,
    mut hand_card_changed: EventReader<HandCardChanged>,
    mut cards: Query<(Entity, &mut Transform, &mut Card), With<HandCard>>,
    card_plane: Query<&Transform, (With<HandCardPlane>, Without<Card>)>,
) {
    for _ in hand_card_changed.read() {
        let num = cards.iter().len();
        if num > 0 {
            if let Ok(tr) = card_plane.get_single() {
                let hand_positions =
                    calculate_hand_positions(num, 0.0, 200., PI / 4., tr.translation.z, -6.7);
                let mut list: Vec<_> = cards.iter_mut().collect();
                // 排序保障动画流畅
                list.sort_by(|a, b| a.1.translation.x.partial_cmp(&b.1.translation.x).unwrap());

                list.iter_mut().enumerate().for_each(
                    |(index, &mut (ref mut _entity, ref mut transform, ref mut card))| {
                        let target = AnimationTarget.into_target();
                        let mut start = target.transform_state(transform.clone());
                        if let Some(tr_end) = hand_positions.get(index) {
                            commands
                                .spawn((Name::new("hand card changed"),))
                                .animation()
                                .insert_tween_here(
                                    Duration::from_secs_f32(0.2),
                                    EaseKind::ExponentialOut,
                                    start.translation_to(tr_end.clone().translation),
                                );
                            card.origin = tr_end.clone();
                        }
                    },
                )
            }
        }
    }
}
