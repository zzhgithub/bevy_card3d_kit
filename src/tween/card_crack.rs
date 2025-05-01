use crate::card::card_material::CardMaterial;
use crate::prelude::Card;
use crate::tween::card_gray::EffectCut;
use bevy::prelude::*;
use bevy_tween::combinator::tween;
use bevy_tween::interpolate::Interpolator;
use bevy_tween::prelude::{AnimationBuilderExt, AssetTween, EaseKind, IntoTarget};
use bevy_tween::{BevyTweenRegisterSystems, asset_tween_system};
use std::time::Duration;

#[derive(Debug, Copy, Clone, Component)]
pub struct CardCrack;

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct CrackScale {
    #[allow(missing_docs)]
    pub start: f32,
    #[allow(missing_docs)]
    pub end: f32,
}

impl Interpolator for CrackScale {
    type Item = CardMaterial;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.crack_scale = self.start.lerp(self.end, value);
    }
}

pub fn crack_scale(start: f32, end: f32) -> CrackScale {
    CrackScale { start, end }
}

pub struct CardCrackPlugin;

impl Plugin for CardCrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_tween_systems(asset_tween_system::<CrackScale>())
            .register_type::<AssetTween<CrackScale>>();
        app.add_systems(Update, (added_card_crack, remove_card_crack));
    }
}

fn added_card_crack(
    mut commands: Commands,
    query: Query<(&Name, &Children), (With<Card>, Added<CardCrack>)>,
    query_mal: Query<&MeshMaterial3d<CardMaterial>>,
) {
    for (card_name, children) in query.iter() {
        for inner_entity in children.iter() {
            if let Ok(material) = query_mal.get(inner_entity) {
                let mal_target = material.clone().0.into_target();
                commands
                    .spawn(Name::new(format!("card crack {}", card_name)))
                    .animation()
                    .insert(tween(
                        Duration::from_secs_f32(0.6),
                        EaseKind::ExponentialOut,
                        mal_target.with(crack_scale(0.0, 0.5)),
                    ));
            }
        }
    }
}

fn remove_card_crack(
    mut commands: Commands,
    mut removed: RemovedComponents<CardCrack>,
    query: Query<(&Name, &Children), With<Card>>,
    query_mal: Query<&MeshMaterial3d<CardMaterial>>,
) {
    for entity in removed.read() {
        if let Ok((card_name, children)) = query.get(entity) {
            for inner_entity in children.iter() {
                if let Ok(material) = query_mal.get(inner_entity) {
                    let mal_target = material.clone().0.into_target();
                    commands
                        .spawn(Name::new(format!("card de crack {}", card_name)))
                        .animation()
                        .insert(tween(
                            Duration::from_secs_f32(0.6),
                            EaseKind::ExponentialOut,
                            mal_target.with(crack_scale(0.5, 0.)),
                        ));
                }
            }
        }
    }
}
