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
    let text_style = TextStyle {
        font: game_assets.font.clone(),
        font_size: 50.0,
        color: Color::BLACK,
    };
    // Spawn Game over text
    cmd.spawn((
        Text2dBundle {
            text: Text::from_section("Game over!", text_style.clone())
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 45.0, 2.0).with_scale(Vec3::splat(0.14)),
            ..default()
        },
        InGameOverScreen,
    ));
    // Spawn final score text
    cmd.spawn((
        Text2dBundle {
            text: Text::from_section(format!("Final Score: {}", score.0), text_style.clone())
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 35.0, 2.0).with_scale(Vec3::splat(0.14)),
            ..default()
        },
        InGameOverScreen,
    ));
    // Spawn final score text
    cmd.spawn((
        Text2dBundle {
            text: Text::from_section(format!("High Score: {}", high_score.0), text_style.clone())
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 25.0, 2.0).with_scale(Vec3::splat(0.14)),
            ..default()
        },
        InGameOverScreen,
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
