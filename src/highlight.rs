use bevy::prelude::*;
use bevy_mod_outline::{OutlinePlugin, OutlineStencil, OutlineVolume};

#[derive(Component, Clone, Debug)]
pub struct Highlight {
    pub color: Color,
}

pub struct HighlightPlugin;

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
    }
}

fn added_highlights(
    mut commands: Commands,
    highlights: Query<(Entity, &Highlight), Added<Highlight>>,
) {
    for (entity, highlight) in highlights.iter() {
        commands
            .entity(entity)
            .insert(OutlineVolume {
                visible: true,
                colour: highlight.color,
                width: 1.0,
            })
            .insert(OutlineStencil { ..default() });
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
    mut highlights: Query<&mut OutlineVolume>,
    mut removed: RemovedComponents<Highlight>,
) {
    for entity in removed.read() {
        if let Ok(mut outline) = highlights.get_mut(entity) {
            outline.visible = false;
        }
    }
}
