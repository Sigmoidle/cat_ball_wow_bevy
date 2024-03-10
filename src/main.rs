use bevy::{
    asset::AssetMetaCheck, prelude::*, render::camera::ScalingMode, window::WindowResolution,
};

mod game;
mod menu;
mod splash;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(GameTouches::default())
        .insert_resource(Score::default())
        .insert_resource(HighScore::default())
        .init_state::<GameState>()
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(
            Update,
            (
                touch_input,
                update_hitboxes.after(apply_velocity),
                apply_velocity,
                update_sprites.after(apply_velocity),
            ),
        )
        .add_plugins((game::game_plugin, splash::splash_plugin, menu::menu_plugin))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                resolution: WindowResolution::new(1000.0, 1000.0),
                ..default()
            }),
            ..default()
        }))
        .run();
}

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
    GameOver,
}

// In Bevy this means: x: (-50 to 50), y: (-50 to 50)
const GAME_SHAPE: Vec2 = Vec2 { x: 100.0, y: 100.0 };

#[derive(Resource, Default)]
struct GameTouches(Vec<Vec2>);

#[derive(Resource)]
struct GameAssets {
    ball: Handle<Image>,
    paw_left: Handle<Image>,
    paw_right: Handle<Image>,
    title: Handle<Image>,
    touch_to_start: Handle<Image>,
    font: Handle<Font>,
}

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
    cmd.insert_resource(GameAssets {
        ball: asset_server.load("ball.png"),
        paw_left: asset_server.load("paw_left.png"),
        paw_right: asset_server.load("paw_right.png"),
        title: asset_server.load("title.png"),
        touch_to_start: asset_server.load("touch_to_start.png"),
        font: asset_server.load("Orbitron-Bold.ttf"),
    });
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

fn update_hitboxes(mut query: Query<(&Position, &Size, &mut HitBox)>) {
    for (position, size, mut hit_box) in query.iter_mut() {
        hit_box.0 = Rect::from_center_size(position.0, size.0 / 1.5);
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

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
