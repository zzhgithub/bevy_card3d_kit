use crate::card::card_state::{CardState, calculate_transform};
use crate::prelude::{Card, Dragged, Moveable};
use crate::tween::animation::{
    play_card_going_back_to_place_animation, play_card_going_back_to_trans_animation,
};
use bevy::math::ops::{cos, sin};
use bevy::prelude::*;
use bevy_tween::combinator::{AnimationBuilderExt, parallel, sequence, tween};
use bevy_tween::prelude::{EaseKind, IntoTarget, TransformTargetStateExt};
use bevy_tween::tween::AnimationTarget;
use std::f32::consts::PI;
use std::time::Duration;

pub const HAND_CARD_LEVEL: f32 = 10.0;

#[derive(Component, Clone, Default)]
pub struct CardLine {
    pub transform: Transform,
    pub card_list: Vec<Entity>,
}

/// 手牌
#[derive(Component, Copy, Clone, Default)]
pub struct HandCard {
    pub belong_to_card_line: Option<Entity>,
}

/// 手牌操作的平台
#[derive(Component, Copy, Clone)]
pub struct HandCardPlane;

/// 手牌发生变化事件
#[derive(Event, Debug)]
pub enum HandCardChanged {
    Added {
        card_entity: Entity,
        card_line_entity: Entity,
    },
    Remove {
        card_entity: Entity,
        card_line_entity: Entity,
    },
}

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
            .add_systems(Update, (added_hand_card, change_hand_cards_event));
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
        vec.push(Transform::from_xyz(x, y, z - 0.01 * i as f32));
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
        (&Transform, &Card, &Name, Option<&CardState>),
        (With<HandCard>, With<Moveable>, Without<Dragged>),
    >,
) {
    if let Ok(parent) = query.get(out_trigger.target) {
        if let Ok((card_transform, card, card_name, opt_state)) = query_transform.get(parent.get())
        {
            debug!("hand card on out {}", card_name);
            play_card_going_back_to_trans_animation(
                parent.get(),
                calculate_transform(card.origin, opt_state.cloned()),
                &card_transform,
                card_name,
                &mut commands,
            );
            commands.entity(parent.get()).remove::<HandOnHover>();
        }
    }
}

pub fn added_hand_card(
    mut hand_card_event: EventWriter<HandCardChanged>,
    query: Query<(Entity, &HandCard), (With<Card>, Added<HandCard>)>,
) {
    for (card_entity, hand_card) in query.iter() {
        if let Some(belong_to) = hand_card.belong_to_card_line {
            hand_card_event.send(HandCardChanged::Added {
                card_entity,
                card_line_entity: belong_to,
            });
        }
    }
}

pub fn change_hand_cards_event(
    mut commands: Commands,
    mut hand_card_changed: EventReader<HandCardChanged>,
    mut query_card_line: Query<(&mut CardLine, Option<&CardState>)>,
    mut query_card: Query<(&mut Card, &mut Transform)>,
) {
    for event in hand_card_changed.read() {
        match event {
            HandCardChanged::Added {
                card_entity,
                card_line_entity,
            } => {
                // 给CardLine添加新成员
                if let Ok((mut card_line, opt_state)) = query_card_line.get_mut(*card_line_entity) {
                    info!("add hand card to card line {:?}", card_line_entity);
                    card_line.card_list.push(*card_entity);
                    change_all_cards(
                        &card_line,
                        &mut commands,
                        &mut query_card,
                        opt_state.cloned(),
                    );
                }
            }
            HandCardChanged::Remove {
                card_entity,
                card_line_entity,
            } => {
                // 删除CardLine中的数据
                if let Ok((mut card_line, opt_state)) = query_card_line.get_mut(*card_line_entity) {
                    card_line.card_list.retain(|x| *x != *card_entity);
                    commands.entity(*card_entity).remove::<HandCard>();
                    change_all_cards(
                        &card_line,
                        &mut commands,
                        &mut query_card,
                        opt_state.cloned(),
                    );
                }
            }
        }
    }
}

fn change_all_cards(
    card_line: &CardLine,
    commands: &mut Commands,
    query_card: &mut Query<(&mut Card, &mut Transform)>,
    opt_state: Option<CardState>,
) {
    if card_line.card_list.len() == 0 {
        return;
    }
    // 计算所有卡的位置
    let hand_positions = calculate_hand_positions(
        card_line.card_list.len(),
        card_line.transform.translation.x,
        200.,
        PI / 4.,
        card_line.transform.translation.z,
        card_line.transform.translation.y,
    );
    // 修改动画 和 Card内数据
    card_line
        .card_list
        .iter()
        .enumerate()
        .for_each(|(index, card_entity)| {
            // 更改Card的姿态信息
            if let Some(state) = opt_state.clone() {
                commands.entity(*card_entity).insert(state.clone());
            }
            if let Ok((mut card, card_transform)) = query_card.get_mut(*card_entity) {
                let target = card_entity.clone().into_target();
                let mut start = target.transform_state(*card_transform);
                if let Some(tr_end) = hand_positions.get(index) {
                    let calculated_end = calculate_transform(tr_end.clone(), opt_state.clone());
                    // 修改这里的值
                    card.origin = Transform::from_translation(tr_end.translation);

                    info!(
                        "change from {:?} to {:?} state{:?}",
                        card.origin, calculated_end, opt_state
                    );
                    commands
                        .spawn(Name::new(format!("hand card at index {}", index)))
                        .animation()
                        .insert(sequence((parallel((
                            tween(
                                Duration::from_secs_f32(0.1),
                                EaseKind::ExponentialOut,
                                start.translation_to(calculated_end.translation),
                            ),
                            tween(
                                Duration::from_secs_f32(0.1),
                                EaseKind::ExponentialOut,
                                start.rotation_to(calculated_end.rotation),
                            ),
                            tween(
                                Duration::from_secs_f32(0.1),
                                EaseKind::ExponentialOut,
                                start.scale_to(calculated_end.scale),
                            ),
                        )),)));
                }
            }
        })
}
