use crate::card::card_state::CardState;
use crate::prelude::Card;
use bevy::prelude::*;

pub struct PreviewPlugins;

#[derive(Component)]
pub struct ImagePreview(pub String);

/// 控制预览的时长
#[derive(Resource)]
pub struct LongPressTimer(Timer);

#[derive(Resource)]
pub struct ImageStage(pub Option<Handle<Image>>);

#[derive(Component)]
struct QuitPreview;

#[derive(Resource)]
struct PreviewPress(pub bool);

#[derive(PartialEq, Clone, Eq, Copy, Default, Debug, Hash, States)]
pub enum PreviewState {
    #[default]
    Disable,
    Show,
}

impl Plugin for PreviewPlugins {
    fn build(&self, app: &mut App) {
        app.init_state::<PreviewState>();
        app.enable_state_scoped_entities::<PreviewState>();
        app.add_systems(Startup, setup);
        app.add_systems(OnEnter(PreviewState::Show), show_preview);
        app.add_systems(
            Update,
            (handle_mouse_input, check_long_press).run_if(in_state(PreviewState::Disable)),
        );
        app.add_systems(
            Update,
            check_quit_preview.run_if(in_state(PreviewState::Show)),
        );
    }
}

fn setup(mut commands: Commands) {
    // 初始化长按计时器
    commands.insert_resource(LongPressTimer(Timer::from_seconds(
        0.6,
        TimerMode::Repeating,
    )));
    // 初始化预览图片
    commands.insert_resource(ImageStage(None));
    commands.insert_resource(PreviewPress(false));
}

fn show_preview(mut commands: Commands, image_stage: ResMut<ImageStage>) {
    if let Some(handle) = &image_stage.0 {
        // 创建页面显示结果
        commands
            .spawn((
                Node {
                    height: Val::Vh(100.0),
                    width: Val::Vw(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                GlobalZIndex(1),
                Button,
                QuitPreview,
                StateScoped(PreviewState::Show),
                BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.5)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Node {
                        height: Val::Percent(80.0),
                        ..default()
                    },
                    ImageNode::new(handle.clone()),
                ));
            });
    }
}

fn handle_mouse_input(
    mut timer: ResMut<LongPressTimer>,
    input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&Interaction, &mut ImagePreview), With<Button>>,
    mut preview_press: ResMut<PreviewPress>,
) {
    for (_, _preview) in query.iter_mut() {
        if input.just_pressed(MouseButton::Left) {
            if !preview_press.0 {
                debug!("MouseButton::Left pressed start");
                timer.0.reset();
                preview_press.0 = true;
            }
        }
        if input.just_released(MouseButton::Left) {
            // info!("MouseButton::Left just_released");
            preview_press.0 = false;
            timer.0.reset();
        }
    }
}

fn check_long_press(
    time: Res<Time>,
    mut timer: ResMut<LongPressTimer>,
    mut query: Query<(&Interaction, &mut ImagePreview), With<Button>>,
    mut image_stage: ResMut<ImageStage>,
    mut next_state: ResMut<NextState<PreviewState>>,
    asset_server: Res<AssetServer>,
) {
    // 当前状态是啥状态啊？
    if timer.0.tick(time.delta()).just_finished() {
        debug!("Finish {:?}", timer.0);
        for (interaction, preview) in query.iter_mut() {
            debug!("Find {:?}", interaction);
            if *interaction == Interaction::Pressed {
                debug!("on Button Pressed");
                // 处理长按事件
                let image = asset_server.load(format!("cards/{}.png", preview.0));
                image_stage.0 = Some(image);
                next_state.set(PreviewState::Show);
            }
        }
    }
}

pub fn preview_on_click(
    drag_start: Trigger<Pointer<Pressed>>,
    query: Query<(&mut ImagePreview, Option<&CardState>), With<Card>>,
    mut image_stage: ResMut<ImageStage>,
    mut next_state: ResMut<NextState<PreviewState>>,
    mut now_state: Res<State<PreviewState>>,
    p_q: Query<&ChildOf>,
    asset_server: Res<AssetServer>,
) {
    info!("Clicked on preview");
    match now_state.get() {
        PreviewState::Disable => {
            if let Ok(parent) = p_q.get(drag_start.target) {
                // 右键才处理
                if drag_start.button == PointerButton::Secondary {
                    if let Ok((preview, opt_state)) = query.get(parent.parent()) {
                        if let Some(card_state) = opt_state {
                            if !card_state.face_up {
                                return;
                            }
                        }
                        let image = asset_server.load(format!("cards/{}.png", preview.0));
                        image_stage.0 = Some(image);
                        next_state.set(PreviewState::Show);
                    }
                }
            }
        }
        PreviewState::Show => {}
    }
}

fn check_quit_preview(
    mut query: Query<(&Interaction, &QuitPreview), With<Button>>,
    mut next_state: ResMut<NextState<PreviewState>>,
    mut image_stage: ResMut<ImageStage>,
) {
    for (interaction, _) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            image_stage.0 = None;
            next_state.set(PreviewState::Disable);
        }
    }
}
