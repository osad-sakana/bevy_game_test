use bevy::prelude::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

// 自作コンポーネント(マーカー)
#[derive(Component)]
struct Player;

fn setup(mut commands: Commands){
    // 2Dカメラ
    commands.spawn(Camera2d);

    // 四角いスプライト(プレイヤー仮置き)
    commands.spawn((
        Player,
        Sprite{
            color: Color::srgb(0.25, 0.75, 0.25),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.single_mut() else { return; };
    let speed = 200.0;

    if keyboard.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.translation.x += speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        transform.translation.y += speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= speed * time.delta_secs();
    }
}
