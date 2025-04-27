use bevy::prelude::{Color, Component};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Clone, Serialize, Deserialize, Debug,Component)]
pub struct FrameConfigData {
    pub image_config: HashMap<usize, ItemData>,
    pub attrs: HashMap<String, String>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub enum ItemData {
    #[default]
    Empty,
    Image {
        path: String,
    },
    Text {
        content: String,
        color: Color,
    },
}
