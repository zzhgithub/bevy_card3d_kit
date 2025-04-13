use bevy::hierarchy::Children;
use bevy::prelude::{Commands, Component, Entity, EventReader, Has, Query};

#[derive(Component)]
pub struct ClearOnFinish;

trait ClearOnFinishExt {
    fn clear_on_finish(self) -> Self;
}

impl ClearOnFinishExt for bevy_tween::combinator::AnimationBuilder<'_> {
    fn clear_on_finish(mut self) -> Self {
        self.entity_commands().insert(ClearOnFinish);
        self
    }
}

pub fn clear_on_finish_system(
    mut commands: Commands,
    mut time_runner_finished: EventReader<bevy_tween::bevy_time_runner::TimeRunnerEnded>,
    has_clear_on_finish: Query<Has<ClearOnFinish>>,
    q_children: Query<&Children>,
    q_tween: Query<(Entity, Has<bevy_tween::bevy_time_runner::TimeSpan>)>,
) {
    for time_runner in time_runner_finished.read() {
        if has_clear_on_finish
            .get(time_runner.time_runner)
            .unwrap_or(false)
        {
            let Ok(children) = q_children.get(time_runner.time_runner) else {
                continue;
            };
            for (entity, is_tween) in q_tween.iter_many(children) {
                if is_tween {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
