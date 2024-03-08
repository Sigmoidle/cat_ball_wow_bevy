use bevy::{
    asset::AssetMetaCheck, prelude::*, render::camera::ScalingMode, window::WindowResolution,
};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(GameTouches::default())
        .insert_resource(Score::default())
        .insert_resource(HighScore::default())
        .add_systems(Startup, (setup_camera, setup))
        .add_systems(
            Update,
            (
                touch_input,
                update_hitboxes.after(apply_velocity),
                apply_velocity,
                clamp_paw_movement.after(apply_velocity),
                bounce_balls_off_walls.before(apply_velocity),
                bounce_balls_off_paws.after(update_hitboxes),
                move_paws.after(touch_input),
                paw_friction.before(apply_velocity),
                update_sprites.after(apply_velocity),
                update_high_score,
                update_score_texts,
            ),
        )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                resolution: WindowResolution::new(1000.0, 1000.0),
                canvas: Some("#game".into()),
                ..default()
            }),
            ..default()
        }))
        .run();
}

const PAW_ACCELERATION: f32 = 5.0;
const PAW_FRICTION: f32 = -0.3;
const PAW_SHAPE: Vec2 = Vec2 {
    x: 20.0 / 1.5,
    y: 30.0 / 1.5,
};

const BALL_SHAPE: Vec2 = Vec2 { x: 10.0, y: 10.0 };
const BASE_BALL_VELOCITY: f32 = 0.2;

// In Bevy this means: x: (-50 to 50), y: (-50 to 50)
const GAME_SHAPE: Vec2 = Vec2 { x: 100.0, y: 100.0 };

#[derive(Resource, Default)]
struct GameTouches(Vec<Vec2>);

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Resource, Default)]
struct HighScore(u32);

#[derive(Component, Default)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Size(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct HitBox(Rect);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paw;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HighScoreText;

fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // Spawn background
    cmd.spawn((
        Position(Vec2 { x: 0.0, y: 0.0 }),
        Size(GAME_SHAPE),
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                custom_size: Some(GAME_SHAPE),
                ..default()
            },
            ..default()
        },
    ));
    // Spawn ball
    cmd.spawn((
        Ball,
        HitBox::default(),
        Position(Vec2 { x: 30.0, y: 0.0 }),
        Size(BALL_SHAPE),
        Velocity(Vec2 {
            x: BASE_BALL_VELOCITY,
            y: BASE_BALL_VELOCITY,
        }),
        SpriteBundle {
            texture: asset_server.load("ball.png"),
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
        Paw,
        HitBox::default(),
        Velocity::default(),
        Position(Vec2 {
            x: -(GAME_SHAPE.x / 4.0),
            y: -(GAME_SHAPE.y / 2.0) + PAW_SHAPE.y / 2.0,
        }),
        Size(PAW_SHAPE),
        SpriteBundle {
            texture: asset_server.load("paw_left.png"),
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
        Paw,
        HitBox::default(),
        Velocity::default(),
        Position(Vec2 {
            x: (GAME_SHAPE.x / 4.0),
            y: -(GAME_SHAPE.y / 2.0) + PAW_SHAPE.y / 2.0,
        }),
        Size(PAW_SHAPE),
        SpriteBundle {
            texture: asset_server.load("paw_right.png"),
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
        ScoreText,
        TextBundle::from_section(
            "Score: ",
            TextStyle {
                font: asset_server.load("Orbitron-Bold.ttf"),
                font_size: 50.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        }),
    ));
    // Spawn High Score Text
    cmd.spawn((
        HighScoreText,
        TextBundle::from_section(
            "High Score: ",
            TextStyle {
                font: asset_server.load("Orbitron-Bold.ttf"),
                font_size: 50.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(100.0),
            left: Val::Px(50.0),
            ..default()
        }),
    ));
}

fn setup_camera(mut cmd: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: GAME_SHAPE.x,
        min_height: GAME_SHAPE.y,
    };
    cmd.spawn(camera_bundle);
}

fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0 * time.delta_seconds() * 60.0;
    }
}

fn bounce_balls_off_walls(
    mut query: Query<(&Position, &Size, &mut Velocity), With<Ball>>,
    mut score: ResMut<Score>,
) {
    for (position, size, mut velocity) in query.iter_mut() {
        if position.0.x + (size.0.x / 2.0) > GAME_SHAPE.x / 2.0 && velocity.0.x > 0.0 {
            velocity.0.x = -BASE_BALL_VELOCITY;
            score.0 += 1;
        }
        if position.0.x - (size.0.x / 2.0) < -(GAME_SHAPE.x / 2.0) && velocity.0.x < 0.0 {
            velocity.0.x = BASE_BALL_VELOCITY;
            score.0 += 1;
        }
        if position.0.y + (size.0.y / 2.0) > GAME_SHAPE.y / 2.0 && velocity.0.y > 0.0 {
            velocity.0.y = -BASE_BALL_VELOCITY;
            score.0 += 1;
        }
    }
}

fn update_sprites(mut query: Query<(&mut Transform, &Position, &mut Sprite, &Size)>) {
    for (mut transform, position, mut sprite, size) in query.iter_mut() {
        transform.translation = Vec3 {
            x: position.0.x,
            y: position.0.y,
            ..transform.translation
        };
        sprite.custom_size = Some(size.0);
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
                velocity.0.y = BASE_BALL_VELOCITY;
                score.0 += 1;
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

fn update_hitboxes(mut query: Query<(&Position, &Size, &mut HitBox)>) {
    for (position, size, mut hit_box) in query.iter_mut() {
        hit_box.0 = Rect::from_center_size(position.0, size.0 / 1.5);
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

fn touch_input(
    touches: Res<Touches>,
    mut game_touches: ResMut<GameTouches>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    game_touches.0 = touches
        .iter()
        .filter_map(|t| camera.viewport_to_world_2d(camera_transform, t.position()))
        .collect::<Vec<Vec2>>();
}
