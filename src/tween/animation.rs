use crate::prelude::event::DeclareDraggingDoneForCard;
use crate::prelude::{Card, ClearOnFinishExt};
use bevy::core::Name;
use bevy::prelude::{Commands, Entity, Transform};
use bevy_tween::combinator::{event, parallel, sequence, tween};
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
        .clear_on_finish()
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
