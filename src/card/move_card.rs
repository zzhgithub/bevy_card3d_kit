use bevy::app::App;
use bevy::prelude::*;
use std::marker::PhantomData;

/// 可以被移动的
#[derive(Component, Copy, Clone)]
pub struct Moveable;

pub struct MoveCardPlugin<C, P>
where
    C: Component,
    P: Component,
{
    pub(crate) _phantom: PhantomData<(C, P)>,
}

impl<C, P> Plugin for MoveCardPlugin<C, P>
where
    C: Send + Sync + 'static + Component,
    P: Send + Sync + 'static + Component,
{
    fn build(&self, app: &mut App) {
        app.add_observer(move_on_drag::<C, P>());
    }
}

/// 在3d的某个平面上一移动observer
pub fn move_on_drag<C, P>() -> impl Fn(
    Trigger<Pointer<Drag>>,
    Query<&mut Transform, (With<C>, With<Moveable>)>,
    Single<(&Camera, &GlobalTransform)>,
    Single<&Window>,
    Single<&GlobalTransform, With<P>>,
)
where
    C: Component,
    P: Component,
{
    move |drag, mut transforms, camera_query, windows, ground| {
        // 这个是需要修改的值
        if let Ok(mut transform) = transforms.get_mut(drag.entity()) {
            let (camera, camera_transform) = *camera_query;

            let Some(cursor_position) = windows.cursor_position() else {
                return;
            };

            // Calculate a ray pointing from the camera into the world based on the cursor's position.
            let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
                return;
            };

            // Calculate if and where the ray is hitting the ground plane.
            let Some(distance) =
                ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
            else {
                return;
            };
            let point = ray.get_point(distance);
            transform.translation.x = point.x;
            transform.translation.y = point.y;
        }
    }
}

// todo drag start
// todo drag end
