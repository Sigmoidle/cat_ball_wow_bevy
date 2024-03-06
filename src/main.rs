use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResized};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(GameTouches::default())
        .insert_resource(ScreenSize::default())
        .insert_resource(MinScreenAxis::default())
        .add_systems(Startup, (setup_camera, setup))
        .add_systems(FixedPreUpdate, (update_hitboxes, touch_input))
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                bounce_balls_off_walls,
                bounce_balls_off_paws,
                move_paws,
            ),
        )
        .add_systems(Update, (update_sprites, on_resize_system))
        .add_plugins(DefaultPlugins)
        .run();
}

const PAW_ACCELERATION: f32 = 5.0;
const PAW_FRICTION: f32 = -0.2;
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

#[derive(Resource, Default, Debug)]
struct ScreenSize(Vec2);

#[derive(Resource, Default, Debug)]
struct MinScreenAxis(f32);

#[derive(Component, Default)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Size(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct HitBox(Rect);

#[derive(Component, Default)]
struct Ball;

#[derive(Component, Default)]
struct Paw;

fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // Spawn background
    cmd.spawn((
        Position(Vec2 { x: 0.0, y: 0.0 }),
        Size(GAME_SHAPE),
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: Transform::from_xyz(0., 0., 0.),
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
            ..default()
        },
    ));
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0
    }
}

fn bounce_balls_off_walls(mut query: Query<(&Position, &Size, &mut Velocity), With<Ball>>) {
    for (position, size, mut velocity) in query.iter_mut() {
        if position.0.x + (size.0.x / 2.0) > GAME_SHAPE.x / 2.0 {
            velocity.0.x = -BASE_BALL_VELOCITY
        }
        if position.0.x - (size.0.x / 2.0) < -(GAME_SHAPE.x / 2.0) {
            velocity.0.x = BASE_BALL_VELOCITY
        }
        if position.0.y + (size.0.y / 2.0) > GAME_SHAPE.y / 2.0 {
            velocity.0.y = -BASE_BALL_VELOCITY
        }
    }
}

fn update_sprites(
    min_screen_axis: Res<MinScreenAxis>,
    mut query: Query<(&mut Transform, &mut Sprite, &Position, &Size)>,
) {
    for (mut transform, mut sprite, game_position, game_size) in query.iter_mut() {
        let screen_scale = (Vec2 {
            x: min_screen_axis.0,
            y: min_screen_axis.0,
        } / GAME_SHAPE)
            * game_size.0;
        let screen_point = (Vec2 {
            x: min_screen_axis.0,
            y: min_screen_axis.0,
        } / GAME_SHAPE)
            * game_position.0;
        transform.translation = Vec3 {
            x: screen_point.x,
            y: screen_point.y,
            ..transform.translation
        };
        sprite.custom_size = Some(Vec2::new(screen_scale.x, screen_scale.y));
    }
}

fn bounce_balls_off_paws(
    mut ball_query: Query<(&mut Velocity, &HitBox), With<Ball>>,
    paw_query: Query<&HitBox, With<Paw>>,
) {
    for (mut velocity, ball_hitbox) in ball_query.iter_mut() {
        for paw_hitbox in paw_query.iter() {
            if !ball_hitbox.0.intersect(paw_hitbox.0).is_empty() {
                velocity.0.y = BASE_BALL_VELOCITY
            }
        }
    }
}

fn move_paws(mut query: Query<(&Position, &mut Velocity), With<Paw>>, touches: Res<GameTouches>) {
    let mut paw_touches = touches.0.clone();
    for (position, mut velocity) in query.iter_mut() {
        if paw_touches.is_empty() {
            return;
        }

        let closest_touch = paw_touches
            .iter()
            .fold((f32::INFINITY, Vec2::default()), |last_len, touch| {
                if (touch.x - position.0.x).abs() < last_len.0 {
                    ((touch.x - position.0.x).abs(), *touch)
                } else {
                    last_len
                }
            })
            .1;

        if closest_touch.x > position.0.x {
            velocity.0.x = BASE_BALL_VELOCITY
        } else if closest_touch.x < position.0.x {
            velocity.0.x = -BASE_BALL_VELOCITY
        }

        paw_touches.remove(
            paw_touches
                .iter()
                .position(|touch| *touch == closest_touch)
                .unwrap(),
        );
    }
}

fn update_hitboxes(mut query: Query<(&Position, &Size, &mut HitBox)>) {
    for (position, size, mut hit_box) in query.iter_mut() {
        hit_box.0 = Rect::from_center_size(position.0, size.0 / 1.5);
    }
}

fn on_resize_system(
    mut screen_size: ResMut<ScreenSize>,
    mut min_screen_axis: ResMut<MinScreenAxis>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        screen_size.0 = Vec2 {
            x: e.width,
            y: e.height,
        };
        min_screen_axis.0 = screen_size.0.min_element()
    }
}

fn touch_input(
    touches: Res<Touches>,
    mut game_touches: ResMut<GameTouches>,
    min_axis: Res<MinScreenAxis>,
) {
    game_touches.0 = touches
        .iter()
        .map(|t| ((t.position() / min_axis.0) * GAME_SHAPE) - GAME_SHAPE / 2.0)
        .collect::<Vec<Vec2>>();
}
