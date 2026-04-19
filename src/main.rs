mod player;
mod coin;
mod score;
mod audio;

use bevy::prelude::*;
use player::PlayerPlugin;
use coin::CoinPlugin;
use score::ScorePlugin;
use audio::AudioPlugin;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(CoinPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(AudioPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
