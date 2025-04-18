use bevy::prelude::{Entity, Event};

// 卡片进入到场地
#[derive(Event, Clone, Debug)]
pub struct CardOnZone {
    pub card: Entity,
    pub zone: Entity,
}

// 卡片进入到卡片上
#[derive(Event, Clone, Debug)]
pub struct CardOnCard {
    pub bottom_card: Entity,
    pub top_card: Entity,
}
