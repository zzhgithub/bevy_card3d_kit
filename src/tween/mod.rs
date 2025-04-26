use crate::card::event::CardsEventsPlugin;
use crate::tween::base_color::{BaseColor, basic_color};
use crate::tween::card_gray::CardGrayPlugin;
use crate::tween::clear_on_finish::clear_on_finish_system;
use crate::tween::shark::{SharkCamera, custom_interpolators_plugin, effect_intensity};
use bevy::color::palettes::basic::{WHITE, YELLOW};
use bevy::prelude::*;
use bevy_tween::bevy_time_runner::TimeRunnerEnded;
use bevy_tween::combinator::{AnimationBuilderExt, sequence, tween};
use bevy_tween::interpolate::scale;
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::{AssetTween, IntoTarget, TweenEvent};
use bevy_tween::tween::AnimationTarget;
use bevy_tween::tween_event::TweenEventPlugin;
use bevy_tween::{BevyTweenRegisterSystems, asset_tween_system};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub mod animation;
mod base_color;
pub mod card_gray;
pub mod clear_on_finish;
pub mod shark;

pub struct ExtTweenPlugins;

impl Plugin for ExtTweenPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((CardsEventsPlugin, CardGrayPlugin));
        app.add_plugins(TweenEventPlugin::<DespawnEntityAfterAll>::default());
        // 在动画执行后 删除原来的动画实体
        app.add_systems(Update, (clear_on_finish_system, despawn_done_time_runners));
        // 导入3d的Color变化
        app.add_tween_systems(asset_tween_system::<BaseColor>())
            .register_type::<AssetTween<BaseColor>>();
        // 镜头震动相关
        custom_interpolators_plugin::<SharkCamera>(app);
        // 通用tween 事件效果 和 创建effect 实体删除
        app.add_systems(Update, effect_system);
        app.add_observer(listen_to_despawn_events);
    }
}

#[derive(Component)]
struct Effect;

fn effect_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    _asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event: EventReader<TweenEvent<&'static str>>,
    query: Query<(Entity, &Transform), With<SharkCamera>>,
) {
    event.read().for_each(|event| match event.data {
        "small_boom" => {
            let handle = materials.add(StandardMaterial {
                base_color: WHITE.into(),
                unlit: true,
                ..Default::default()
            });
            let target = handle.clone().into_target();

            let entity_effect = commands
                .spawn((
                    Effect,
                    Mesh3d(meshes.add(Annulus::new(2.6, 3.0))),
                    MeshMaterial3d(handle),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    AnimationTarget,
                ))
                .id();
            let entity = entity_effect.into_target();
            commands
                .spawn(Name::new("small_boom"))
                .animation()
                .insert(sequence((
                    tween(
                        Duration::from_secs_f32(0.2),
                        EaseKind::Linear,
                        (
                            entity.with(scale(Vec3::new(0.6, 0.6, 0.), Vec3::new(3., 3., 0.))),
                            target.with(basic_color(
                                into_color(WHITE.with_alpha(0.5)),
                                into_color(YELLOW.with_alpha(0.)),
                            )),
                        ),
                    ),
                    bevy_tween::combinator::event(DespawnEntityAfterAll {
                        entity: Some(entity_effect),
                    }),
                )));
        }
        "boom" => {
            info!("Boom!");
            let handle = materials.add(StandardMaterial {
                base_color: WHITE.into(),
                unlit: true,
                ..Default::default()
            });
            let target = handle.clone().into_target();

            let entity_effect = commands
                .spawn((
                    Effect,
                    Mesh3d(meshes.add(Annulus::new(2.6, 3.0))),
                    MeshMaterial3d(handle),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    AnimationTarget,
                ))
                .id();
            let entity = entity_effect.into_target();
            commands
                .spawn(Name::new("boom"))
                .animation()
                .insert(sequence((
                    tween(
                        Duration::from_secs_f32(1.0),
                        EaseKind::QuadraticOut,
                        (
                            entity.with(scale(Vec3::new(1., 1., 0.), Vec3::new(10., 10., 0.))),
                            target.with(basic_color(
                                into_color(WHITE.with_alpha(1.)),
                                into_color(YELLOW.with_alpha(0.)),
                            )),
                        ),
                    ),
                    bevy_tween::combinator::event(DespawnEntityAfterAll {
                        entity: Some(entity_effect),
                    }),
                )));
        }
        "shark" => {
            if let Ok((entity, _trans)) = query.single() {
                commands.entity(entity).insert(AnimationTarget);
                commands
                    .spawn(Name::new("shark"))
                    .animation()
                    .insert(sequence((
                        tween(
                            Duration::from_secs_f32(0.2),
                            EaseKind::QuarticIn,
                            effect_intensity(0., 1.),
                        ),
                        tween(
                            Duration::from_secs_f32(1.),
                            EaseKind::QuarticIn,
                            effect_intensity(1., 0.0),
                        ),
                    )));
            }
        }
        _ => {}
    });
}

pub fn despawn_done_time_runners(
    mut time_runner_ended_reader: EventReader<TimeRunnerEnded>,
    mut commands: Commands,
) {
    for event in time_runner_ended_reader.read() {
        if event.is_completed() {
            commands.entity(event.time_runner).despawn();
        }
    }
}

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Default)]
pub struct DespawnEntityAfterAll {
    pub entity: Option<Entity>,
}

fn listen_to_despawn_events(
    trigger: Trigger<TweenEvent<DespawnEntityAfterAll>>,
    mut commands: Commands,
) {
    if let Some(entity) = trigger.data.entity {
        commands.entity(entity).despawn();
    }
}
