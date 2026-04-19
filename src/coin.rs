use bevy::prelude::*;
use rand::random_range;
use crate::player::{Player, PLAYER_SIZE};

const COIN_SIZE: f32 = 20.0;
const COIN_COUNT: usize = 10;

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
    player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
){
    let Ok(player_transform) = player_query.single() else { return; };

    for(coin_entiry, coin_transform) in &coin_query {
        let distance = player_transform
            .translation
            .distance(coin_transform.translation);

        let hit_distance = (PLAYER_SIZE + COIN_SIZE) / 2.0;

        if distance < hit_distance {
            commands.entity(coin_entiry).despawn();
        }
    }
}
