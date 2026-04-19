mod player;
mod coin;

use bevy::prelude::*;
use player::PlayerPlugin;
use coin::CoinPlugin;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(CoinPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
