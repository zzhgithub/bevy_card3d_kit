use crate::card3d::Card3DConfig;
use crate::prelude::{Card, HandCard};
use crate::tween::animation::play_card_going_back_to_place_animation;
use crate::zone::Zone;
use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct DeskCard {
    pub belongs_to_desk: Option<Entity>,
}

#[derive(Component, Clone, Debug, Default)]
pub struct DeskZone {
    pub card_list: Vec<Entity>,
}

pub fn when_added_to_desk(
    mut commands: Commands,
    card3d_config: Res<Card3DConfig>,
    mut added_query: Query<(Entity, &mut Card, &DeskCard, &Name, &Transform), Added<DeskCard>>,
    mut zone_query: Query<(&Zone, Option<&mut DeskZone>)>,
) {
    for (card_entity, mut card, desk_card, card_name, card_transform) in added_query.iter_mut() {
        if let Some(desk_entity) = desk_card.belongs_to_desk {
            if let Ok((zone, desk_zone)) = zone_query.get_mut(desk_entity) {
                let mut end = zone.center.clone();
                if let Some(mut desk_zone) = desk_zone {
                    desk_zone.card_list.push(card_entity);
                    // 卡片移动到位置上
                    end.translation.z =
                        (desk_zone.card_list.len() + 1) as f32 * card3d_config.thick;
                    //
                } else {
                    end.translation.z = card3d_config.thick;
                    commands.entity(desk_entity).insert(DeskZone {
                        card_list: vec![card_entity],
                    });
                }
                card.origin = end;
                // 去除手牌 如果是手牌的话
                commands.entity(desk_entity).remove::<HandCard>();
                // 动画移动
                play_card_going_back_to_place_animation(
                    card_entity,
                    &card,
                    card_transform,
                    card_name,
                    &mut commands,
                );
            }
        }
    }
}
