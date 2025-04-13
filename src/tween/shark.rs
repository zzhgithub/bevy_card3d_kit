use bevy::prelude::*;
use bevy_tween::prelude::Interpolator;
use bevy_tween::tween::ResourceTween;
use bevy_tween::{BevyTweenRegisterSystems, resource_tween_system};
use rand::Rng;

#[derive(Component, Default, Copy, Clone)]
pub struct SharkCamera;

#[derive(Default, Resource)]
pub struct EffectIntensitiy(f32);

pub struct EffectIntensity {
    pub start: f32,
    pub end: f32,
}

impl Interpolator for EffectIntensity {
    type Item = EffectIntensitiy;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.0 = self.start.lerp(self.end, value)
    }
}

pub fn effect_intensity(start: f32, end: f32) -> ResourceTween<EffectIntensity> {
    ResourceTween::new(EffectIntensity { start, end })
}

pub fn custom_interpolators_plugin<C>(app: &mut App)
where
    C: Component,
{
    app.add_tween_systems(resource_tween_system::<EffectIntensity>());
    app.init_resource::<EffectIntensitiy>();
    app.add_systems(Update, shark_do_effect::<C>);
}

fn shark_do_effect<C: Component>(
    effect_intensity: Res<EffectIntensitiy>,
    mut shark_q: Query<&mut Transform, With<C>>,
) {
    let mut rng = rand::rng();
    let dx: f32 = rng.random();
    let dy: f32 = rng.random();
    if let Ok(mut transform) = shark_q.get_single_mut() {
        transform.translation.x = dx * effect_intensity.0;
        transform.translation.y = dy * effect_intensity.0;
    }
}
