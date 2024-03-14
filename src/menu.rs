use bevy::prelude::*;

use crate::{GameAssets, GameTouches};

use super::{despawn_screen, GameState, Position, Size};

#[derive(Component)]
struct InMenuScreen;

#[derive(Component)]
struct MenuTitle;

#[derive(Component)]
struct Blinking;

#[derive(Resource, Deref, DerefMut)]
struct BlinkTimer(Timer);

const MENU_TITLE_SIZE: Vec2 = Vec2 { x: 75.0, y: 50.0 };
const MENU_TOUCH_TO_START_SIZE: Vec2 = Vec2 { x: 60.0, y: 22.0 };
const BLINK_EVER_SECS: f32 = 1.0;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), setup)
        .add_systems(
            Update,
            (wait_for_touch, blink).run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnExit(GameState::Menu), despawn_screen::<InMenuScreen>);
}

fn setup(mut cmd: Commands, game_assets: Res<GameAssets>) {
    // spawn title
    cmd.spawn((
        InMenuScreen,
        MenuTitle,
        Position(Vec2 { x: 0.0, y: 10.0 }),
        Size(MENU_TITLE_SIZE),
        SpriteBundle {
            texture: game_assets.title.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
    ));
    // spawn touch to start
    cmd.spawn((
        InMenuScreen,
        Blinking,
        Position(Vec2 { x: 0.0, y: -20.0 }),
        Size(MENU_TOUCH_TO_START_SIZE),
        SpriteBundle {
            texture: game_assets.touch_to_start.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
    ));
    cmd.insert_resource(BlinkTimer(Timer::from_seconds(
        BLINK_EVER_SECS,
        TimerMode::Repeating,
    )))
}

fn blink(
    time: Res<Time>,
    mut timer: ResMut<BlinkTimer>,
    mut query: Query<&mut Visibility, With<Blinking>>,
) {
    if timer.tick(time.delta()).finished() {
        for mut visibility in query.iter_mut() {
            match *visibility {
                Visibility::Visible => *visibility = Visibility::Hidden,
                Visibility::Hidden => *visibility = Visibility::Visible,
                Visibility::Inherited => *visibility = Visibility::Hidden,
            }
        }
    }
}

fn wait_for_touch(mut game_state: ResMut<NextState<GameState>>, touches: Res<GameTouches>) {
    if !touches.0.is_empty() {
        game_state.set(GameState::Game);
    }
}
