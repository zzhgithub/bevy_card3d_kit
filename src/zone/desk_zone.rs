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

#[derive(Component, Clone, Debug, Default)]
pub struct DeskZone {
    pub card_list: Vec<Entity>,
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
    query: Query<(Entity, &DeskCard), (With<Card>, Added<DeskCard>)>,
) {
    for (card_entity, desk_card) in query.iter() {
        if let Some(belongs_to_desk) = desk_card.belongs_to_desk {
            desk_card_event.send(DeskZoneChangedEvent::Added {
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
    mut query_card: Query<(&mut Card, &mut Transform)>,
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
    query_card: &mut Query<(&mut Card, &mut Transform)>,
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
            // 更改卡片的姿态信息
            if let Some(state) = &opt_state {
                commands.entity(*card_entity).insert(state.clone());
            }
            if let Ok((mut card, card_transform)) = query_card.get_mut(*card_entity) {
                let target = card_entity.clone().into_target();
                let mut start = target.transform_state(*card_transform);
                let mut end = zone.center.clone();
                end.translation.z = (index + 1) as f32 * card3d_config.thick;
                let calculated_end = calculate_transform(end.clone(), opt_state.clone());
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

#[deprecated]
pub fn when_added_to_desk(
    mut commands: Commands,
    card3d_config: Res<Card3DConfig>,
    mut added_query: Query<(Entity, &mut Card, &DeskCard, &Name, &Transform), Added<DeskCard>>,
    mut zone_query: Query<(&Zone, Option<&mut DeskZone>, Option<&CardState>)>,
) {
    for (card_entity, mut card, desk_card, card_name, card_transform) in added_query.iter_mut() {
        if let Some(desk_entity) = desk_card.belongs_to_desk {
            if let Ok((zone, desk_zone, opt_state)) = zone_query.get_mut(desk_entity) {
                let mut end = zone.center.clone();
                if let Some(mut desk_zone) = desk_zone {
                    desk_zone.card_list.push(card_entity);
                    // 卡片移动到位置上
                    end.translation.z =
                        (desk_zone.card_list.len() + 1) as f32 * card3d_config.thick;
                } else {
                    end.translation.z = card3d_config.thick;
                    commands.entity(desk_entity).insert(DeskZone {
                        card_list: vec![card_entity],
                    });
                }
                card.origin = end;
                // 去除手牌 如果是手牌的话
                commands
                    .entity(card_entity)
                    .remove::<HandCard>()
                    .remove::<Moveable>();
                if let Some(card_state) = opt_state {
                    commands.entity(card_entity).insert(card_state.clone());
                }
                // 动画移动
                play_card_going_back_to_trans_animation(
                    card_entity,
                    calculate_transform(end, opt_state.cloned()),
                    card_transform,
                    card_name,
                    &mut commands,
                );
            }
        }
    }
}
