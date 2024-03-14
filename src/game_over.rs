use bevy::prelude::*;

use crate::{despawn_screen, GameAssets, GameState, GameTouches, HighScore, Score};

#[derive(Component)]
struct InGameOverScreen;

pub fn game_over_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), setup)
        .add_systems(Update, wait_for_touch.run_if(in_state(GameState::GameOver)))
        .add_systems(
            OnExit(GameState::GameOver),
            despawn_screen::<InGameOverScreen>,
        );
}

fn setup(
    mut cmd: Commands,
    game_assets: Res<GameAssets>,
    score: Res<Score>,
    high_score: Res<HighScore>,
) {
    // Spawn game over Text
    cmd.spawn((
        InGameOverScreen,
        TextBundle::from_section(
            "Game over!",
            TextStyle {
                font: game_assets.font.clone(),
                font_size: 50.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(300.0),
            left: Val::Px(400.0),
            ..default()
        }),
    ));
    // Spawn Final Score Text
    cmd.spawn((
        InGameOverScreen,
        TextBundle::from_section(
            format!("Final Score: {}", score.0),
            TextStyle {
                font: game_assets.font.clone(),
                font_size: 50.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(400.0),
            left: Val::Px(400.0),
            ..default()
        }),
    ));
    // Spawn High Score Text
    cmd.spawn((
        InGameOverScreen,
        TextBundle::from_section(
            format!("High Score: {}", high_score.0),
            TextStyle {
                font: game_assets.font.clone(),
                font_size: 50.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(500.0),
            left: Val::Px(400.0),
            ..default()
        }),
    ));
}

fn wait_for_touch(
    mut game_state: ResMut<NextState<GameState>>,
    touches: Res<GameTouches>,
    mut score: ResMut<Score>,
) {
    if !touches.0.is_empty() {
        score.0 = 0;
        game_state.set(GameState::Splash);
    }
}
