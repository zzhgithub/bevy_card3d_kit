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
        app.add_systems(Update, (added_highlights, remove_highlights));
    }
}

fn added_highlights(
    mut commands: Commands,
    highlights: Query<(Entity, &Highlight), Added<Highlight>>,
) {
    for (entity, highlight) in highlights.iter() {
        commands
            .entity(entity)
            .insert(Mesh3d::default())
            .insert(OutlineVolume {
                visible: true,
                colour: highlight.color,
                width: 10.0,
            })
            .insert(OutlineStencil { ..default() });
    }
}

fn remove_highlights(mut commands: Commands, mut removed: RemovedComponents<Highlight>) {
    for entity in removed.read() {
        commands.entity(entity).remove::<OutlineVolume>();
    }
}
