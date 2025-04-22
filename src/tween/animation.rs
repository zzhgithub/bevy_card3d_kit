use crate::prelude::Card;
use crate::prelude::event::DeclareDraggingDoneForCard;
use crate::zone::Zone;
use bevy::core::Name;
use bevy::prelude::{Commands, Entity, Transform, Vec3};
use bevy_tween::combinator::{event, event_for, parallel, sequence, tween};
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::{AnimationBuilderExt, IntoTarget, TransformTargetStateExt};
use std::time::Duration;

// 卡片移动回某个地方
pub fn play_card_going_back_to_place_animation(
    card_entity: Entity,
    card: &Card,
    card_transform: &Transform,
    card_name: &Name,
    commands: &mut Commands,
) {
    let animation_target = card_entity.into_target();
    let mut transform_state = animation_target.transform_state(*card_transform);
    commands
        .spawn((Name::new(format!(
            "Go-back-to-origin-after-dragging animation parent for {}",
            card_name
        )),))
        .animation()
        .insert(sequence((
            parallel((
                tween(
                    Duration::from_secs_f32(0.04),
                    EaseKind::ExponentialOut,
                    transform_state.translation_to(card.origin.translation),
                ),
                tween(
                    Duration::from_secs_f32(0.04),
                    EaseKind::ExponentialOut,
                    transform_state.rotation_to(card.origin.rotation),
                ),
                tween(
                    Duration::from_secs_f32(0.04),
                    EaseKind::ExponentialOut,
                    transform_state.scale_to(card.origin.scale),
                ),
            )),
            event(DeclareDraggingDoneForCard {
                card_entity: Some(card_entity),
            }),
        )));
}

/// 卡片回到某个位置
pub fn play_card_going_back_to_trans_animation(
    card_entity: Entity,
    back_to: Transform,
    card_transform: &Transform,
    card_name: &Name,
    commands: &mut Commands,
) {
    let animation_target = card_entity.into_target();
    let mut transform_state = animation_target.transform_state(*card_transform);
    commands
        .spawn((Name::new(format!(
            "Go-back-to-trans-after-dragging animation parent for {}",
            card_name
        )),))
        .animation()
        .insert(sequence((
            parallel((
                tween(
                    Duration::from_secs_f32(0.04),
                    EaseKind::ExponentialOut,
                    transform_state.translation_to(back_to.translation),
                ),
                tween(
                    Duration::from_secs_f32(0.04),
                    EaseKind::ExponentialOut,
                    transform_state.rotation_to(back_to.rotation),
                ),
                tween(
                    Duration::from_secs_f32(0.04),
                    EaseKind::ExponentialOut,
                    transform_state.scale_to(back_to.scale),
                ),
            )),
            event(DeclareDraggingDoneForCard {
                card_entity: Some(card_entity),
            }),
        )));
}

/// 登场动画
pub fn card_set_on_zone_animation(
    card_entity: Entity,
    card: &Card,
    zone: &Zone,
    card_transform: &Transform,
    card_name: &Name,
    commands: &mut Commands,
) {
    let animation_target = card_entity.into_target();
    let mut transform_state = animation_target.transform_state(*card_transform);

    let mut mid = Vec3::ZERO;
    mid.z = card.origin.translation.z;

    let mut mid2 = Vec3::ZERO;
    mid2.z = card.origin.translation.z + 7.0;

    let mut mid_state = animation_target.transform_state(Transform::from_translation(mid));
    let mut mid_state2 = animation_target.transform_state(Transform::from_translation(mid));

    commands
        .spawn((Name::new(format!("card_set_on_zone-{}", card_name)),))
        .animation()
        .insert(sequence((
            tween(
                Duration::from_secs_f32(1.0),
                EaseKind::ExponentialOut,
                transform_state.translation_to(mid),
            ),
            parallel((
                tween(
                    Duration::from_secs_f32(1.0),
                    EaseKind::ExponentialOut,
                    mid_state.translation_to(mid2),
                ),
                sequence((
                    event_for(Duration::from_secs_f32(0.4), "small_boom"),
                    event("boom"),
                )),
            )),
            parallel((
                tween(
                    Duration::from_secs_f32(0.6),
                    EaseKind::ExponentialOut,
                    mid_state2.translation_to(zone.center.translation),
                ),
                event("shark"),
            )),
        )));
}

/// 没有特效的登场动画
pub fn card_set_on_zone_animation_without_boom(
    card_entity: Entity,
    card: &Card,
    zone: &Zone,
    card_transform: &Transform,
    card_name: &Name,
    commands: &mut Commands,
) {
    let animation_target = card_entity.into_target();
    let mut transform_state = animation_target.transform_state(*card_transform);

    let mut mid = Vec3::ZERO;
    mid.z = card.origin.translation.z;

    let mut mid2 = Vec3::ZERO;
    mid2.z = card.origin.translation.z + 7.0;

    let mut mid_state = animation_target.transform_state(Transform::from_translation(mid));
    let mut mid_state2 = animation_target.transform_state(Transform::from_translation(mid));

    commands
        .spawn((Name::new(format!("card_set_on_zone-{}", card_name)),))
        .animation()
        .insert(sequence((
            tween(
                Duration::from_secs_f32(1.0),
                EaseKind::ExponentialOut,
                transform_state.translation_to(mid),
            ),
            parallel((tween(
                Duration::from_secs_f32(1.0),
                EaseKind::ExponentialOut,
                mid_state.translation_to(mid2),
            ),)),
            parallel((tween(
                Duration::from_secs_f32(0.6),
                EaseKind::ExponentialOut,
                mid_state2.translation_to(zone.center.translation),
            ),)),
        )));
}
