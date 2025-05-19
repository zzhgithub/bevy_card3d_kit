use crate::prelude::Card;
use bevy::prelude::*;
use bevy_tween::combinator::{TransformTargetStateExt, sequence, tween};
use bevy_tween::prelude::{AnimationBuilderExt, EaseKind, IntoTarget};
use std::f32::consts::PI;
use std::time::Duration;

/// 卡片姿态信息
#[derive(Debug, PartialEq, Eq, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct CardState {
    /// 正面朝上
    pub face_up: bool,
    /// 是否竖直
    pub vertical: bool,
}

/// 计算姿态影响下的值
pub fn calculate_transform(trans: Transform, opt_card_state: Option<CardState>) -> Transform {
    let mut res = trans.clone();
    if let Some(card_state) = opt_card_state {
        if !card_state.face_up {
            res.rotate(Quat::from_axis_angle(Vec3::Y, PI));
        }
        if !card_state.vertical {
            res.rotate(Quat::from_axis_angle(Vec3::Z, PI / 2.0));
        }
    }
    res
}

#[derive(Debug, PartialEq, Eq, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct ChangeCardState(pub CardState);

pub struct CardStatePlugin;

impl Plugin for CardStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChangeCardState>();
        app.add_systems(Update, on_update_change_state);
    }
}

fn on_update_change_state(
    mut commands: Commands,
    query: Query<(Entity, &Card, &CardState, &ChangeCardState), Added<ChangeCardState>>,
) {
    query
        .iter()
        .for_each(|(entity, card, card_state, change_card_state)| {
            let start_tr = calculate_transform(card.origin, Some(card_state.clone()));
            let end_tr = calculate_transform(card.origin, Some(change_card_state.0.clone()));

            let target = entity.clone().into_target();
            let mut start = target.transform_state(start_tr);

            commands
                .entity(entity)
                .insert(change_card_state.0.clone())
                .remove::<ChangeCardState>();
            info!("state from {:?} To {:?}", card_state, change_card_state.0);
            if change_card_state.0.face_up != card_state.face_up
                || change_card_state.0.vertical != card_state.vertical
            {
                commands
                    .spawn(Name::new(format!(
                        "change card state {:?}",
                        change_card_state
                    )))
                    .animation()
                    .insert(sequence((
                        tween(
                            Duration::from_secs_f32(0.6),
                            EaseKind::ExponentialOut,
                            start.translation_to(end_tr.translation),
                        ),
                        tween(
                            Duration::from_secs_f32(0.6),
                            EaseKind::ExponentialOut,
                            start.rotation_to(end_tr.rotation),
                        ),
                        tween(
                            Duration::from_secs_f32(0.6),
                            EaseKind::ExponentialOut,
                            start.scale_to(end_tr.scale),
                        ),
                    )));
            }
        })
}
