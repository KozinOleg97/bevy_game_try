use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SPEED: f32 = 500.0;



// Точка входа, конфигурирует игровой цикл
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player))
        .add_systems(Update, (player_movement, print_position, print_player_position))

        .run();
}

// Не используется
#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

// Не используется
#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

// TODO Уточнить, задание пустой структуры
#[derive(Component, Debug)]
pub struct Player {}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // TODO уточнить, про создание спрайта без привязки к Сущности (?)
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

// Не используется
fn update_position(
    mut query: Query<(&Velocity, &mut Position)>
) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn print_position(
    query: Query<(Entity, &Position)>
) {
    for (entity, position) in query.iter() {
        info!("Entity {:?} is at position {:?}", entity, position);
    }
}

fn print_player_position(
    query: Query<(Entity, &Transform, With<Player>)>
) {
    for (entity, position, player) in query.iter() {
        info!("Entity {:?} is at position {:?} is {:?}", entity, position, player);
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}