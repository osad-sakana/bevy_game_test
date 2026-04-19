use bevy::prelude::*;

const SPEED: f32 = 5000.0;
const FRICTION: f32 = 0.85;
pub const PLAYER_SIZE: f32 = 50.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(Vec2);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_player)
            .add_systems(Update, (move_player, apply_velocity, clamp_position));
    }
}

fn setup_player(mut commands: Commands){
    commands.spawn((
        Player,
        Velocity(Vec2::ZERO),
        Sprite{
            color: Color::srgb(0.25, 0.75, 0.25),
            custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut velocity) = query.single_mut() else { return; };

    if keyboard.pressed(KeyCode::ArrowRight){
        velocity.0.x += SPEED * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::ArrowLeft){
        velocity.0.x -= SPEED * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::ArrowUp){
        velocity.0.y += SPEED * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::ArrowDown){
        velocity.0.y -= SPEED * time.delta_secs();
    }

     // 摩擦の適用
    velocity.0 *= FRICTION;
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    time: Res<Time>,
){
    for(mut transform, velocity) in &mut query{
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

fn clamp_position(
    mut query: Query<&mut Transform, With<Player>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else { return; };
    let Ok(mut transform) = query.single_mut() else { return; };

    let half_w = window.width() / 2.0 - PLAYER_SIZE / 2.0;
    let half_h = window.height() / 2.0 - PLAYER_SIZE / 2.0;

    transform.translation.x = transform.translation.x.clamp(-half_w, half_w);
    transform.translation.y = transform.translation.y.clamp(-half_h, half_h);
}
