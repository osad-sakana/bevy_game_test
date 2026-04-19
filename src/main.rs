mod player;
mod coin;
mod score;

use bevy::prelude::*;
use player::PlayerPlugin;
use coin::CoinPlugin;
use score::ScorePlugin;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(CoinPlugin)
        .add_plugins(ScorePlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
