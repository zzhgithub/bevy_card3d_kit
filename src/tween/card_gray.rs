use crate::card::card_material::CardMaterial;
use crate::prelude::Card;
use bevy::app::App;
use bevy::prelude::*;
use bevy_tween::combinator::{AnimationBuilderExt, sequence, tween};
use bevy_tween::interpolate::{Interpolator, scale};
use bevy_tween::prelude::{AssetTween, EaseKind, IntoTarget};
use bevy_tween::{BevyTweenRegisterSystems, asset_tween_system};
use std::time::Duration;

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct CardGray {
    #[allow(missing_docs)]
    pub start: f32,
    #[allow(missing_docs)]
    pub end: f32,
}

impl Interpolator for CardGray {
    type Item = CardMaterial;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.gray_scale = self.start.lerp(self.end, value);
    }
}

pub fn card_gray(start: f32, end: f32) -> CardGray {
    CardGray { start, end }
}

pub struct CardGrayPlugin;

impl Plugin for CardGrayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CardEffectCutAnimationEvent>();
        // 卡片灰度相关
        app.add_tween_systems(asset_tween_system::<CardGray>())
            .register_type::<AssetTween<CardGray>>();
        app.add_systems(
            Update,
            (
                add_effect_cut,
                deal_card_effect_cut_animation_event,
                remove_effect_cut,
            ),
        );
    }
}

#[derive(Component, Clone)]
pub struct EffectCut;

fn add_effect_cut(
    query: Query<Entity, Added<EffectCut>>,
    mut event_writer: EventWriter<CardEffectCutAnimationEvent>,
) {
    for card_entity in query.iter() {
        info!("Added effect cut");
        event_writer.write(CardEffectCutAnimationEvent { card_entity });
    }
}

#[derive(Debug, Clone, Event)]
/// 卡片效果无效动画
pub struct CardEffectCutAnimationEvent {
    pub card_entity: Entity,
}

fn deal_card_effect_cut_animation_event(
    mut commands: Commands,
    mut events: EventReader<CardEffectCutAnimationEvent>,
    query: Query<(&Name, &Children), With<Card>>,
    query_mal: Query<&MeshMaterial3d<CardMaterial>>,
) {
    for event in events.read() {
        if let Ok((card_name, children)) = query.get(event.card_entity.clone()) {
            for inner_entity in children.iter() {
                if let Ok(material) = query_mal.get(inner_entity) {
                    let gray_target = material.clone().0.into_target();
                    let animation_target = event.card_entity.clone().into_target();
                    // 创建动画
                    commands
                        .spawn(Name::new(format!("card gray {}", card_name)))
                        .animation()
                        .insert(sequence((
                            tween(
                                Duration::from_secs_f32(1.0),
                                EaseKind::ExponentialOut,
                                animation_target.with(scale(Vec3::splat(1.0), Vec3::splat(1.2))),
                            ),
                            tween(
                                Duration::from_secs_f32(0.4),
                                EaseKind::ExponentialOut,
                                gray_target.with(card_gray(0.0, 1.0)),
                            ),
                            tween(
                                Duration::from_secs_f32(0.4),
                                EaseKind::ExponentialOut,
                                gray_target.with(card_gray(1.0, 0.0)),
                            ),
                            tween(
                                Duration::from_secs_f32(0.4),
                                EaseKind::ExponentialOut,
                                gray_target.with(card_gray(0.0, 1.0)),
                            ),
                            tween(
                                Duration::from_secs_f32(0.4),
                                EaseKind::ExponentialOut,
                                gray_target.with(card_gray(1.0, 0.0)),
                            ),
                            tween(
                                Duration::from_secs_f32(0.2),
                                EaseKind::ExponentialOut,
                                gray_target.with(card_gray(0.0, 1.0)),
                            ),
                            tween(
                                Duration::from_secs_f32(0.8),
                                EaseKind::ExponentialOut,
                                animation_target.with(scale(Vec3::splat(1.2), Vec3::splat(1.0))),
                            ),
                        )));
                }
            }
        }
    }
}

fn remove_effect_cut(
    mut commands: Commands,
    mut removed: RemovedComponents<EffectCut>,
    query: Query<(&Name, &Children), With<Card>>,
    query_mal: Query<&MeshMaterial3d<CardMaterial>>,
) {
    for entity in removed.read() {
        if let Ok((card_name, children)) = query.get(entity) {
            for inner_entity in children.iter() {
                if let Ok(material) = query_mal.get(inner_entity) {
                    let gray_target = material.clone().0.into_target();
                    commands
                        .spawn(Name::new(format!("card gray {}", card_name)))
                        .animation()
                        .insert(tween(
                            Duration::from_secs_f32(0.6),
                            EaseKind::ExponentialOut,
                            gray_target.with(card_gray(1.0, 0.0)),
                        ));
                }
            }
        }
    }
}
