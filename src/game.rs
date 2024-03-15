use bevy::prelude::*;

use crate::GameAssets;

use super::{
    apply_velocity, despawn_screen, update_hitboxes, GameState, GameTouches, HighScore, HitBox,
    Position, Score, Size, Velocity, GAME_SHAPE,
};

const PAW_ACCELERATION: f32 = 5.0;
const PAW_FRICTION: f32 = -0.3;
const PAW_SHAPE: Vec2 = Vec2 {
    x: 20.0 / 1.5,
    y: 30.0 / 1.5,
};

const BALL_SHAPE: Vec2 = Vec2 { x: 10.0, y: 10.0 };
const BASE_BALL_VELOCITY: f32 = 0.2;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paw;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HighScoreText;

#[derive(Component)]
struct InGameScreen;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup)
        .add_systems(
            Update,
            (
                clamp_paw_movement.after(apply_velocity),
                bounce_balls_off_walls.before(apply_velocity),
                bounce_balls_off_paws.after(update_hitboxes),
                move_paws,
                paw_friction.before(apply_velocity),
                update_high_score,
                update_score_texts,
                despawn_ball,
                end_game,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Game), despawn_screen::<InGameScreen>);
}

fn setup(mut cmd: Commands, game_assets: Res<GameAssets>) {
    // Spawn ball
    cmd.spawn((
        InGameScreen,
        Ball,
        HitBox::default(),
        Position(Vec2 { x: 30.0, y: 0.0 }),
        Size(BALL_SHAPE),
        Velocity(Vec2 {
            x: BASE_BALL_VELOCITY,
            y: -BASE_BALL_VELOCITY,
        }),
        SpriteBundle {
            texture: game_assets.ball.clone(),
            transform: Transform::from_xyz(0., 0., 2.),
            sprite: Sprite {
                custom_size: Some(BALL_SHAPE),
                ..default()
            },
            ..default()
        },
    ));
    // Spawn another ball
    cmd.spawn((
        InGameScreen,
        Ball,
        HitBox::default(),
        Position(Vec2 { x: 0.0, y: 0.0 }),
        Size(BALL_SHAPE),
        Velocity(Vec2 {
            x: -BASE_BALL_VELOCITY,
            y: -BASE_BALL_VELOCITY,
        }),
        SpriteBundle {
            texture: game_assets.ball.clone(),
            transform: Transform::from_xyz(0., 0., 2.),
            sprite: Sprite {
                custom_size: Some(BALL_SHAPE),
                ..default()
            },
            ..default()
        },
    ));
    // Spawn paws
    cmd.spawn((
        InGameScreen,
        Paw,
        HitBox::default(),
        Velocity::default(),
        Position(Vec2 {
            x: -(GAME_SHAPE.x / 4.0),
            y: -(GAME_SHAPE.y / 2.0) + PAW_SHAPE.y / 2.0,
        }),
        Size(PAW_SHAPE),
        SpriteBundle {
            texture: game_assets.paw_left.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            sprite: Sprite {
                custom_size: Some(PAW_SHAPE),
                ..default()
            },
            ..default()
        },
    ));
    // Spawn paws
    cmd.spawn((
        InGameScreen,
        Paw,
        HitBox::default(),
        Velocity::default(),
        Position(Vec2 {
            x: (GAME_SHAPE.x / 4.0),
            y: -(GAME_SHAPE.y / 2.0) + PAW_SHAPE.y / 2.0,
        }),
        Size(PAW_SHAPE),
        SpriteBundle {
            texture: game_assets.paw_right.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            sprite: Sprite {
                custom_size: Some(PAW_SHAPE),
                ..default()
            },
            ..default()
        },
    ));
    // Spawn Score Text
    cmd.spawn((
        Position(Vec2 { x: -40.0, y: 40.0 }),
        InGameScreen,
        ScoreText,
        TextBundle::from_section(
            "Score: ",
            TextStyle {
                font: game_assets.font.clone(),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ),
    ));
    // Spawn High Score Text
    cmd.spawn((
        Position(Vec2 { x: -40.0, y: 30.0 }),
        InGameScreen,
        HighScoreText,
        TextBundle::from_section(
            "High Score: ",
            TextStyle {
                font: game_assets.font.clone(),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ),
    ));
}

fn calc_ball_velocity(score: u32) -> f32 {
    BASE_BALL_VELOCITY + BASE_BALL_VELOCITY * (score as f32 / 30.0)
}

fn bounce_balls_off_walls(
    mut query: Query<(&Position, &Size, &mut Velocity), With<Ball>>,
    mut score: ResMut<Score>,
) {
    for (position, size, mut velocity) in query.iter_mut() {
        if position.0.x + (size.0.x / 2.0) > GAME_SHAPE.x / 2.0 && velocity.0.x > 0.0 {
            score.0 += 1;
            velocity.0.x = -calc_ball_velocity(score.0);
        }
        if position.0.x - (size.0.x / 2.0) < -(GAME_SHAPE.x / 2.0) && velocity.0.x < 0.0 {
            score.0 += 1;
            velocity.0.x = calc_ball_velocity(score.0);
        }
        if position.0.y + (size.0.y / 2.0) > GAME_SHAPE.y / 2.0 && velocity.0.y > 0.0 {
            score.0 += 1;
            velocity.0.y = -calc_ball_velocity(score.0);
        }
    }
}

fn bounce_balls_off_paws(
    mut ball_query: Query<(&mut Velocity, &HitBox), With<Ball>>,
    paw_query: Query<&HitBox, With<Paw>>,
    mut score: ResMut<Score>,
) {
    for (mut velocity, ball_hitbox) in ball_query.iter_mut() {
        for paw_hitbox in paw_query.iter() {
            if !ball_hitbox.0.intersect(paw_hitbox.0).is_empty() && velocity.0.y < 0.0 {
                score.0 += 1;
                velocity.0.y = calc_ball_velocity(score.0);
            }
        }
    }
}

fn move_paws(
    mut query: Query<(&mut Position, &mut Velocity), With<Paw>>,
    touches: Res<GameTouches>,
) {
    for touch in &touches.0 {
        // Find the closest paw for the current touch
        if let Some((position, mut velocity)) = query.iter_mut().min_by_key(|(position, _)| {
            let diff = touch.x - position.0.x;
            (diff * diff) as i32
        }) {
            // Calculate the velocity based on the matched paw's position
            let direction = (touch.x - position.0.x) / GAME_SHAPE.x;
            velocity.0.x += (direction * PAW_ACCELERATION) + 0.5 * (direction * PAW_ACCELERATION);
        }
    }
}

fn update_high_score(mut high_score: ResMut<HighScore>, score: Res<Score>) {
    if high_score.0 < score.0 {
        high_score.0 = score.0;
    }
}

fn clamp_paw_movement(mut query: Query<&mut Position, With<Paw>>) {
    for mut position in query.iter_mut() {
        position.0.x = position.0.x.clamp(-GAME_SHAPE.x / 2.0, GAME_SHAPE.x / 2.0);
    }
}

fn paw_friction(mut query: Query<&mut Velocity, With<Paw>>) {
    for mut velocity in query.iter_mut() {
        velocity.0.x += velocity.0.x * PAW_FRICTION;
    }
}

#[allow(clippy::type_complexity)]
fn update_score_texts(
    mut set: ParamSet<(
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<HighScoreText>>,
    )>,
    score: Res<Score>,
    high_score: Res<HighScore>,
) {
    for mut score_text in set.p0().iter_mut() {
        score_text.sections[0].value = format!("Score: {}", score.0);
    }
    for mut high_score_text in set.p1().iter_mut() {
        high_score_text.sections[0].value = format!("High Score: {}", high_score.0);
    }
}

fn despawn_ball(mut cmd: Commands, query: Query<(Entity, &Position), With<Ball>>) {
    for (entity, position) in query.iter() {
        if position.0.y < (-GAME_SHAPE.y / 2.0) - BALL_SHAPE.y {
            cmd.entity(entity).despawn_recursive();
        }
    }
}

fn end_game(mut game_state: ResMut<NextState<GameState>>, query: Query<&Ball>) {
    if query.is_empty() {
        game_state.set(GameState::GameOver);
    }
}
