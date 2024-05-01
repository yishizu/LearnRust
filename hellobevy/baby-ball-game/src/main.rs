use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::scene::ron::de;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;
const NUMBER_OF_ENEMIES: u32 = 5;
const ENEMY_SPEED: f32 = 200.0;
const NUMBER_OF_STARS: u32 = 15;
const STAR_SIZE: f32 = 32.0;
const STAR_SPWAN_TIME: f32 = 2.0;
const ENEMY_SPAWN_TIME: f32 = 5.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource::<Score>(default())
        .insert_resource::<StarSpawnTimer>(default())
        .insert_resource::<EnemySpawnTimer>(default())
        .add_systems(
            Startup,
            (spawn_camera, spawn_player, spawn_enemies, spawn_stars),
        )
        .add_systems(FixedUpdate, (player_movement, enemy_movement))
        .add_systems(FixedUpdate, confine_player_movement)
        .add_systems(FixedUpdate, update_enemy_direction)
        .add_systems(FixedUpdate, (enermy_hit_player, player_hit_stars))
        .add_systems(FixedUpdate, (tick_star_spawn_timer, tick_enemy_spawn_timer))
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPWAN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..Default::default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),

        ..Default::default()
    });
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if direction.length() > 0.0 {
            transform.translation += direction.normalize() * PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(mut transform) = player_query.get_single_mut() {
        let half_size = PLAYER_SIZE / 2.0;
        if transform.translation.x < half_size {
            transform.translation.x = half_size;
        }
        if transform.translation.x > window.width() - half_size {
            transform.translation.x = window.width() - half_size;
        }
        if transform.translation.y < half_size {
            transform.translation.y = half_size;
        }
        if transform.translation.y > window.height() - half_size {
            transform.translation.y = window.height() - half_size;
        }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for _ in 0..NUMBER_OF_ENEMIES {
        spawn_enemy(&mut commands, &asset_server, &window_query);
    }
}
pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                (window.width()) * rand::random::<f32>(),
                (window.height()) * rand::random::<f32>(),
                0.0,
            ),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..Default::default()
        },
        Enemy {
            direction: Vec3::new(
                rand::random::<f32>() - 0.5,
                rand::random::<f32>() - 0.5,
                0.0,
            )
            .normalize(),
        },
    ));
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let half_size = PLAYER_SIZE / 2.0;
    let x_min = half_size;
    let x_max = window.width() - half_size;
    let y_min = half_size;
    let y_max = window.height() - half_size;

    for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = enemy_transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }
        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
            let sound_effect = if rand::random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            commands.spawn(AudioBundle {
                source: sound_effect,
                ..Default::default()
            });
        }
    }
}

pub fn enermy_hit_player(
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < PLAYER_SIZE {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    ..Default::default()
                });
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for _ in 0..NUMBER_OF_STARS {
        spawn_star(&mut commands, &asset_server, &window_query);
    }
}
fn spawn_star(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                (window.width()) * rand::random::<f32>(),
                (window.height()) * rand::random::<f32>(),
                0.0,
            ),
            texture: asset_server.load("sprites/star.png"),
            ..Default::default()
        },
        Star {},
    ));
}

pub fn player_hit_stars(
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laserLarge_001.ogg"),
                    ..Default::default()
                });
                commands.entity(star_entity).despawn();
                score.value += 1;
                println!("Score: {}", score.value);
            }
        }
    }
}

pub fn tick_star_spawn_timer(
    time: Res<Time>,
    mut timer: ResMut<StarSpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        spawn_star(&mut commands, &asset_server, &window_query);
    }
}
pub fn tick_enemy_spawn_timer(
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        spawn_enemy(&mut commands, &asset_server, &window_query);
    }
}
