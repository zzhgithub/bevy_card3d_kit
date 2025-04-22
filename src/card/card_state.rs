use bevy::prelude::*;
use std::f32::consts::PI;

/// 卡片姿态信息
#[derive(Debug, PartialEq, Eq, Clone, Component)]
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
