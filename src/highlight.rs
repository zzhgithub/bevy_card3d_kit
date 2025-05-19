use bevy::prelude::*;
use bevy_mod_outline::{OutlinePlugin, OutlineStencil, OutlineVolume};
use bevy_tween::combinator::{parallel, tween};
use bevy_tween::component_tween_system;
use bevy_tween::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Highlight {
    pub color: Color,
}

pub struct HighlightPlugin;

#[derive(Component, Clone, Debug)]
pub struct HighlightTween(pub Entity);

impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OutlinePlugin);
        app.add_systems(
            Update,
            (
                added_highlights_with_one,
                added_highlights,
                remove_highlights,
            ),
        );
        app.add_tween_systems((
            component_tween_system::<HighlightWidth>(),
            component_tween_system::<HighlightColor>(),
        ))
        .register_type::<ComponentTween<HighlightWidth>>()
        .register_type::<ComponentTween<HighlightColor>>();
    }
}

fn added_highlights(
    mut commands: Commands,
    highlights: Query<(Entity, &Highlight), Added<Highlight>>,
) {
    for (entity, highlight) in highlights.iter() {
        let target_component = entity.into_target();
        // 添加高亮的默认动画
        let tween_entity = commands
            .spawn(Name::new(format!("Target{:?}", target_component)))
            .animation()
            .repeat(Repeat::Infinitely)
            .repeat_style(RepeatStyle::PingPong)
            .insert(parallel((
                // Note: 直接改变这个大小 不行！计算出来的 边框会不准！
                // tween(
                //     Duration::from_secs_f32(2.0),
                //     EaseKind::Linear,
                //     target_component.with(highlight_width(10.0, 13.0)),
                // ),
                tween(
                    Duration::from_secs_f32(1.2),
                    EaseKind::ExponentialInOut,
                    target_component.with(highlight_color(
                        highlight.color,
                        Color::WHITE.with_alpha(1.0),
                    )),
                ),
            )))
            .id();

        commands
            .entity(entity)
            .insert(OutlineVolume {
                visible: true,
                colour: highlight.color,
                width: 10.0,
            })
            .insert(OutlineStencil { ..default() })
            .insert(HighlightTween(tween_entity));
    }
}

fn added_highlights_with_one(
    mut highlights: Query<(&Highlight, &mut OutlineVolume), Added<Highlight>>,
) {
    for (highlight, mut outline) in highlights.iter_mut() {
        outline.colour = highlight.color;
        outline.visible = true;
    }
}

fn remove_highlights(
    mut commands: Commands,
    mut highlights: Query<(&mut OutlineVolume, &HighlightTween)>,
    mut removed: RemovedComponents<Highlight>,
) {
    for entity in removed.read() {
        if let Ok((mut outline, highlight_tween)) = highlights.get_mut(entity) {
            outline.visible = false;
            commands.entity(entity).remove::<HighlightTween>();
            commands.entity(highlight_tween.0).despawn();
        }
    }
}

/// 高亮的宽度
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct HighlightWidth {
    #[allow(missing_docs)]
    pub start: f32,
    #[allow(missing_docs)]
    pub end: f32,
}

impl Interpolator for HighlightWidth {
    type Item = OutlineVolume;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.width = self.start.lerp(value, self.end);
    }
}

pub fn highlight_width(start: f32, end: f32) -> HighlightWidth {
    HighlightWidth { start, end }
}

/// 高亮的颜色
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct HighlightColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

impl Interpolator for HighlightColor {
    type Item = OutlineVolume;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.colour = self.start.mix(&self.end, value)
    }
}

pub fn highlight_color(start: Color, end: Color) -> HighlightColor {
    HighlightColor { start, end }
}
