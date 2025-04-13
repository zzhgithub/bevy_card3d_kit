use bevy::prelude::*;
use bevy_tween::prelude::Interpolator;

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BaseColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

impl Interpolator for BaseColor {
    type Item = StandardMaterial;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.base_color = self.start.mix(&self.end, value);
    }
}

pub fn basic_color(start: Color, end: Color) -> BaseColor {
    BaseColor { start, end }
}
