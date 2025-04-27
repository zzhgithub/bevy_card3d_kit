use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 框架配置类

#[derive(Default, Clone, Serialize, Deserialize, Debug, Asset, TypePath)]
pub struct FrameConfig {
    pub image_config: HashMap<usize, ItemConfig>,
    pub projected: HashMap<String, String>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub enum ItemType {
    #[default]
    Image,
    Text,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ItemConfig {
    pub size: Vec2,
    pub position: Vec2,
    pub scale: Vec2,
    pub level: Vec2,
    pub name: String,
    pub default: Option<String>,
}
