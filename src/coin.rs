use bevy::prelude::*;
use rand::random_range;
use crate::player::{Player, PLAYER_SIZE};
use crate::score::Score;

const COIN_SIZE: f32 = 20.0;
const COIN_COUNT: usize = 10;
const ATTRACT_RADIUS: f32 = 100.0;
const ATTRACT_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Coin;

pub struct CoinPlugin;

impl Plugin for CoinPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, setup_coins)
            .add_systems(Update, collect_coins);
    }
}

fn setup_coins(
    mut commands: Commands,
    windows: Query<&Window>,
){
    let Ok(window) = windows.single() else { return; };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    for _ in 0..COIN_COUNT{
        let x = random_range(-half_w..half_w);
        let y = random_range(-half_h..half_h);

        commands.spawn((
            Coin,
            Sprite{
                color: Color::srgb(1.0, 0.85, 0.0),
                custom_size: Some(Vec2::new(COIN_SIZE, COIN_SIZE)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}

fn collect_coins(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_query: Query<&Transform, With<Player>>,
    mut coin_query: Query<(Entity, &mut Transform), (With<Coin>, Without<Player>)>,
    time: Res<Time>,
){
    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation;

    for(coin_entiry, mut coin_transform) in &mut coin_query {
        let coin_pos = coin_transform.translation;
        let distance = player_pos.distance(coin_pos);

        if distance < ATTRACT_RADIUS {
            let direction = (player_pos - coin_pos).normalize();
            coin_transform.translation += direction * ATTRACT_SPEED * time.delta_secs();
        }

        let hit_distance = (PLAYER_SIZE + COIN_SIZE) / 2.0;

        if distance < hit_distance {
            commands.entity(coin_entiry).despawn();
            score.0 += 1;
        }
    }
}
