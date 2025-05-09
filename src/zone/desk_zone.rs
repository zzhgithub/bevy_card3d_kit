use crate::card::card_state::{CardState, calculate_transform};
use crate::card3d::Card3DConfig;
use crate::prelude::{Card, HandCard, Moveable};
use crate::tween::animation::play_card_going_back_to_trans_animation;
use crate::zone::Zone;
use bevy::prelude::*;
use bevy_tween::combinator::{
    AnimationBuilderExt, TransformTargetStateExt, parallel, sequence, tween,
};
use bevy_tween::prelude::{EaseKind, IntoTarget};
use std::time::Duration;

#[derive(Component, Clone, Debug)]
pub struct DeskCard {
    pub belongs_to_desk: Option<Entity>,
}

#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DeskZone {
    pub card_list: Vec<Entity>,
    // 平铺容量
    pub opt_capacity: Option<usize>,
}

#[derive(Event, Clone, Debug)]
pub enum DeskZoneChangedEvent {
    Added { desk: Entity, card: Entity },
    Removed { desk: Entity, card: Entity },
}

pub struct DeskZonePlugin;

impl Plugin for DeskZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeskZoneChangedEvent>();
        app.add_systems(Update, (added_desk_card, change_desk_cards_event));
    }
}

fn added_desk_card(
    mut desk_card_event: EventWriter<DeskZoneChangedEvent>,
    mut commands: Commands,
    query: Query<(Entity, &DeskCard), (With<Card>, Added<DeskCard>)>,
) {
    for (card_entity, desk_card) in query.iter() {
        if let Some(belongs_to_desk) = desk_card.belongs_to_desk {
            commands.entity(card_entity).remove::<CardState>();
            desk_card_event.write(DeskZoneChangedEvent::Added {
                desk: belongs_to_desk,
                card: card_entity,
            });
        }
    }
}

pub fn change_desk_cards_event(
    mut commands: Commands,
    mut desk_card_changed: EventReader<DeskZoneChangedEvent>,
    mut query_desk_zone: Query<(&Zone, &mut DeskZone, Option<&CardState>)>,
    mut query_card: Query<(&mut Card, Option<&mut Transform>, Option<&CardState>)>,
    card3d_config: Res<Card3DConfig>,
) {
    for event in desk_card_changed.read() {
        match event {
            DeskZoneChangedEvent::Added { desk, card } => {
                // 添加新成员
                if let Ok((zone, mut desk_zone, opt_state)) = query_desk_zone.get_mut(*desk) {
                    desk_zone.card_list.push(*card);

                    change_desk_cards_transform(
                        zone,
                        &desk_zone,
                        &mut commands,
                        &mut query_card,
                        opt_state.cloned(),
                        card3d_config.clone(),
                    );
                }
            }
            DeskZoneChangedEvent::Removed { desk, card } => {
                if let Ok((zone, mut desk_zone, opt_state)) = query_desk_zone.get_mut(*desk) {
                    desk_zone.card_list.retain(|x| *x != *card);
                    commands.entity(*card).remove::<DeskCard>();
                    change_desk_cards_transform(
                        zone,
                        &desk_zone,
                        &mut commands,
                        &mut query_card,
                        opt_state.cloned(),
                        card3d_config.clone(),
                    );
                }
            }
        }
    }
}

fn change_desk_cards_transform(
    zone: &Zone,
    desk_zone: &DeskZone,
    commands: &mut Commands,
    query_card: &mut Query<(&mut Card, Option<&mut Transform>, Option<&CardState>)>,
    opt_state: Option<CardState>,
    card3d_config: Card3DConfig,
) {
    if desk_zone.card_list.len() == 0 {
        return;
    }
    desk_zone
        .card_list
        .iter()
        .enumerate()
        .for_each(|(index, card_entity)| {
            if let Ok((mut card, opt_card_transform, opt_card_state)) =
                query_card.get_mut(*card_entity)
            {
                let target = card_entity.clone().into_target();
                let mut start = if let Some(card_transform) = opt_card_transform {
                    target.transform_state(*card_transform)
                } else {
                    target.transform_state(card.origin)
                };

                let mut end = zone.center.clone();
                end.translation.z = (index + 1) as f32 * card3d_config.thick;
                // 进行平铺！
                if let Some(capacity) = desk_zone.opt_capacity {
                    let height = zone.size.y;
                    let half_height = height / 2.0;
                    let per = height / capacity as f32;
                    let half_per = per / 2.0;
                    end.translation.y =
                        end.translation.y - half_height + half_per + index as f32 * per;
                }
                let calculated_end = if opt_card_state.is_some() {
                    calculate_transform(end.clone(), opt_card_state.cloned())
                } else {
                    if let Some(zone_state) = opt_state.clone() {
                        info!("Init state in zone");
                        commands.entity(*card_entity).insert(zone_state.clone());
                    }
                    calculate_transform(end.clone(), opt_state.clone())
                };

                // 修改这里的值
                card.origin = Transform::from_translation(end.translation);
                info!(
                    "change from {:?} to {:?} state{:?}",
                    card.origin, calculated_end, opt_state
                );
                commands
                    .spawn(Name::new(format!("desk card at index {}", index)))
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
        });
}
