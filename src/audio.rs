use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_audio);
    }
}

fn setup_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    // BGM
    commands.spawn((
        AudioPlayer::new(asset_server.load("bgm.mp3")),
        PlaybackSettings::LOOP,
    ));
}
