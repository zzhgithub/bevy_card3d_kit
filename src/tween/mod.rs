use crate::card::event::CardsEventsPlugin;
use crate::tween::base_color::{BaseColor, basic_color};
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
use bevy_tween::{BevyTweenRegisterSystems, asset_tween_system};
use std::time::Duration;

mod base_color;
mod clear_on_finish;
pub mod shark;

pub struct ExtTweenPlugins;

impl Plugin for ExtTweenPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(CardsEventsPlugin);
        // 在动画执行后 删除原来的动画实体
        app.add_systems(Update, clear_on_finish_system);
        // 导入3d的Color变化
        app.add_tween_systems(asset_tween_system::<BaseColor>())
            .register_type::<AssetTween<BaseColor>>();
        // 镜头震动相关
        custom_interpolators_plugin::<SharkCamera>(app);
        // 通用tween 事件效果 和 创建effect 实体删除
        app.add_systems(Update, (effect_system, despawn_effect_system));
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

            let entity = AnimationTarget.into_target();
            commands
                .spawn((
                    Effect,
                    Mesh3d(meshes.add(Annulus::new(2.6, 3.0))),
                    MeshMaterial3d(handle),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    AnimationTarget,
                ))
                .animation()
                .insert_tween_here(
                    Duration::from_secs_f32(0.2),
                    EaseKind::Linear,
                    (
                        entity.with(scale(Vec3::new(0.6, 0.6, 0.), Vec3::new(3., 3., 0.))),
                        target.with(basic_color(
                            into_color(WHITE.with_alpha(0.5)),
                            into_color(YELLOW.with_alpha(0.)),
                        )),
                    ),
                );
        }
        "boom" => {
            info!("Boom!");
            let handle = materials.add(StandardMaterial {
                base_color: WHITE.into(),
                unlit: true,
                ..Default::default()
            });
            let mut target = handle.clone().into_target();

            let entity = AnimationTarget.into_target();
            commands
                .spawn((
                    Effect,
                    Mesh3d(meshes.add(Annulus::new(2.6, 3.0))),
                    MeshMaterial3d(handle),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    AnimationTarget,
                ))
                .animation()
                .insert_tween_here(
                    Duration::from_secs_f32(1.0),
                    EaseKind::QuadraticOut,
                    (
                        entity.with(scale(Vec3::new(1., 1., 0.), Vec3::new(10., 10., 0.))),
                        target.with(basic_color(
                            into_color(WHITE.with_alpha(1.)),
                            into_color(YELLOW.with_alpha(0.)),
                        )),
                    ),
                );
        }
        "shark" => {
            if let Ok((entity, _trans)) = query.get_single() {
                commands
                    .entity(entity)
                    .insert(AnimationTarget)
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

fn despawn_effect_system(
    mut commands: Commands,
    q_effect: Query<(), With<Effect>>,
    mut ended: EventReader<TimeRunnerEnded>,
) {
    ended.read().for_each(|ended| {
        if ended.is_completed() && q_effect.contains(ended.time_runner) {
            commands.entity(ended.time_runner).despawn_recursive();
        }
    });
}

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}
