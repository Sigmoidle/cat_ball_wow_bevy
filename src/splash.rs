use bevy::prelude::*;

use crate::GameAssets;

use super::{despawn_screen, GameState, Position, Size};

const SPLASH_TITLE_SIZE: Vec2 = Vec2 { x: 75.0, y: 50.0 };

const SPLASH_TIME: f32 = 1.5;

#[derive(Component)]
struct InSplashScreen;

#[derive(Component)]
struct SplashTitle;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), setup)
        .add_systems(
            Update,
            (countdown, zoom).run_if(in_state(GameState::Splash)),
        )
        .add_systems(OnExit(GameState::Splash), despawn_screen::<InSplashScreen>);
}

fn setup(mut cmd: Commands, game_assets: Res<GameAssets>) {
    // spawn title
    cmd.spawn((
        InSplashScreen,
        SplashTitle,
        Position(Vec2 { x: 0.0, y: 10.0 }),
        Size(Vec2::ZERO),
        SpriteBundle {
            texture: game_assets.title.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
    ));
    // Spawn timer resouce to stop the splash screen
    cmd.insert_resource(SplashTimer(Timer::from_seconds(
        SPLASH_TIME,
        TimerMode::Once,
    )));
}

fn zoom(timer: Res<SplashTimer>, mut query: Query<&mut Size, With<SplashTitle>>) {
    for mut size in query.iter_mut() {
        size.0 = SPLASH_TITLE_SIZE * (timer.elapsed_secs() / SPLASH_TIME);
    }
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
