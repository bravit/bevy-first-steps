use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

const JUMP_FORCE: f32 = 600.0;
const GRAVITY: f32 = -800.0;
const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (jump, player_movement, apply_gravity))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands
        .spawn((
            Player,
            Sprite {
                color: Color::srgb(0.5, 1.0, 0.5),
                custom_size: Some(Vec2::new(30.0, 50.0)),
                ..default()
            },
            Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
            Velocity(Vec3::ZERO),
        ));
}

fn jump(
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
    mut events: EventReader<KeyboardInput>,
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
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta().as_secs_f32();

        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
        }
    }
}

fn apply_gravity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity), With<Player>>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.0.y += GRAVITY * time.delta().as_secs_f32();
        transform.translation.y += velocity.0.y * time.delta().as_secs_f32();

        // Ensure the player stays above the ground
        if transform.translation.y <= -100.0 {
            transform.translation.y = -100.0;
            velocity.0.y = 0.0;
        }
    }
}