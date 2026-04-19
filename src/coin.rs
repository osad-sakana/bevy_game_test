use bevy::prelude::*;
use rand::random_range;

const COIN_SIZE: f32 = 20.0;
const COIN_COUNT: usize = 10;

#[derive(Component)]
pub struct Coin;

pub struct CoinPlugin;

impl Plugin for CoinPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_coins);
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
