use crate::card::card_state::CardState;
use crate::prelude::event::DeclareDraggingDoneForCard;
use crate::prelude::{Card, Dragged};
use crate::tween::animation::play_card_going_back_to_place_animation;
use bevy::app::App;
use bevy::prelude::*;
use bevy_tween::prelude::TweenEvent;
use std::marker::PhantomData;

/// 可以被移动的
#[derive(Component, Copy, Clone)]
pub struct Moveable;

pub struct MoveCardPlugin<P>
where
    P: Component,
{
    pub(crate) _phantom: PhantomData<P>,
}

impl<P> Plugin for MoveCardPlugin<P>
where
    P: Send + Sync + 'static + Component,
{
    fn build(&self, app: &mut App) {
        app.add_observer(on_drag_start)
            .add_observer(move_on_drag::<P>())
            .add_observer(back_to_origin_when_unused)
            .add_observer(listen_to_dragging_done_for_card);
    }
}

/// 在3d的某个平面上一移动observer
pub fn move_on_drag<P>() -> impl Fn(
    Trigger<Pointer<Drag>>,
    Query<&mut Transform, (With<Card>, With<Moveable>)>,
    Single<(&Camera, &GlobalTransform)>,
    Single<&Window>,
    Single<(&GlobalTransform, &Transform), (Without<Card>, With<P>)>,
)
where
    P: Component,
{
    move |drag, mut transforms, camera_query, windows, ground| {
        // 这个是需要修改的值
        if let Ok(mut transform) = transforms.get_mut(drag.target()) {
            let (camera, camera_transform) = *camera_query;

            let Some(cursor_position) = windows.cursor_position() else {
                return;
            };

            // Calculate a ray pointing from the camera into the world based on the cursor's position.
            let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
                return;
            };

            let (ground_transform, ground_tr) = *ground;

            // Calculate if and where the ray is hitting the ground plane.
            let Some(distance) = ray.intersect_plane(
                ground_transform.translation(),
                InfinitePlane3d::new(ground_transform.up()),
            ) else {
                return;
            };
            let point = ray.get_point(distance);
            transform.translation.x = point.x;
            transform.translation.y = point.y;
            transform.translation.z = ground_tr.translation.z;
        }
    }
}

fn on_drag_start(
    drag_start: Trigger<Pointer<DragStart>>,
    // 可以被‘移动’的‘卡片’
    mut card_transforms: Query<(&mut Transform, &Card), (With<Card>, With<Moveable>)>,
    mut commands: Commands,
    query: Query<&Children>,
) {
    if let Ok((_card_transform, _card)) = card_transforms.get_mut(drag_start.target()) {
        if let Ok(mut entity_commands) = commands.get_entity(drag_start.target()) {
            // info!("drag start");
            entity_commands.insert(Dragged::Actively);
        }

        if let Ok(children) = query.get(drag_start.target()) {
            for child in children.iter() {
                commands.entity(child).insert(Pickable::IGNORE);
            }
        }
    }
}

fn back_to_origin_when_unused(
    drag_end: Trigger<Pointer<DragEnd>>,
    mut dragged_cards: Query<(
        &mut Transform,
        Entity,
        &Card,
        &mut Dragged,
        &Name,
        Option<&CardState>,
    )>,
    mut commands: Commands,
    query: Query<&Children>,
) {
    debug!("drag end {:?}", drag_end.target);
    debug!("drag end {:?}", drag_end.target());
    if let Ok((
        card_transform,
        card_entity,
        card,
        mut card_dragged_component,
        card_name,
        opt_state,
    )) = dragged_cards.get_mut(drag_end.target())
    {
        debug!("drag end!!!");
        if let Ok(children) = query.get(drag_end.target()) {
            for child in children.iter() {
                commands.entity(child).remove::<Pickable>();
            }
        }
        *card_dragged_component = Dragged::GoingBackToPlace;
        // 进行动画
        play_card_going_back_to_place_animation(
            card_entity,
            card,
            &card_transform,
            card_name,
            opt_state.cloned(),
            &mut commands,
        );
    }
}

fn listen_to_dragging_done_for_card(
    trigger: Trigger<TweenEvent<DeclareDraggingDoneForCard>>,
    cards: Query<&Dragged, With<Card>>,
    mut commands: Commands,
) {
    if let Some(entity) = trigger.data.card_entity {
        if let Ok(dragged) = cards.get(entity) {
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                match dragged {
                    Dragged::Actively => {
                        // do nothing!
                    }
                    Dragged::GoingBackToPlace => {
                        entity_commands.remove::<Dragged>();
                    }
                }
            }
        }
    }
}
