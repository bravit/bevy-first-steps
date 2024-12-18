use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;

const JUMP_FORCE: f32 = 600.0;
const GRAVITY: f32 = -800.0;
const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;
const GAME_SPEED: f32 = 400.0;
const SPAWN_INTERVAL: f32 = 1.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct Health(usize);

#[derive(Component)]
struct HealthInfo;

#[derive(Resource)]
struct GameState {
    spawn_timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(GameState {
            spawn_timer: Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating),
        })
        .add_systems(Update, (jump, apply_gravity, player_movement))
        .add_systems(Update, (spawn_obstacles, move_obstacles, detect_collision, render_health_info))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let initial_health = 10;
    // Player
    commands
        .spawn((
            Player,
            Sprite {
                color: Color::srgb(0.5, 1.0, 0.5),
                custom_size: Some(Vec2::new(30.0, 50.0)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
            Velocity(Vec3::ZERO),
            Health(initial_health)
        ));

    commands.spawn((
            HealthInfo,
            Text::new(format!("Health: {}", initial_health))
        )
    );

    // Ground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(800.0, 10.0)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        Transform::from_xyz(-400.0, GROUND_LEVEL, 0.0)
    ));
}

fn jump(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>
) {
    for e in events.read() {
        if let Ok((mut velocity, transform)) = query.get_single_mut() {
            if e.state.is_pressed() && e.key_code == KeyCode::Space && transform.translation.y <= GROUND_LEVEL {
                velocity.0.y = JUMP_FORCE;
            }
        }
    }
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();

        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
        }
    }
}

fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        velocity.0.y += GRAVITY * time.delta_secs();
    }
}

fn spawn_obstacles(
    time: Res<Time>,
    mut state: ResMut<GameState>,
    mut commands: Commands,
) {
    state.spawn_timer.tick(time.delta());

    if state.spawn_timer.finished() {
        let mut rng = rand::thread_rng();
        let obstacle_x = 400.0;
        let obstacle_y = GROUND_LEVEL + rng.gen_range(0.0..50.0);

        commands.spawn((
            Obstacle,
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
        ));
    }
}

fn move_obstacles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();

        // Remove obstacles once they're off-screen
        if transform.translation.x < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn detect_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
) {
    if let Ok((player_transform, mut health)) = player_query.get_single_mut() {
        for (entity, obstacle_transform) in obstacle_query.iter() {
            let collision = player_transform.translation.distance(obstacle_transform.translation) < 40.0;
            if collision {
                health.0 -= 1;
                commands.entity(entity).despawn(); // Remove obstacle
            }
        }
    }
}

fn render_health_info(
    player_query: Query<&mut Health, With<Player>>,
    mut health_info_query: Query<&mut Text, With<HealthInfo>>,
) {
    if let Ok(mut health_info) = health_info_query.get_single_mut() {
        if let Ok(health) = player_query.get_single() {
            health_info.0 = format!("Health: {}", health.0);
        }
    }
}