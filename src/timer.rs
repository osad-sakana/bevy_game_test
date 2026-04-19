use bevy::prelude::*;
use crate::game_state::GameState;

const GAME_TIME: f32 = 60.0;

#[derive(Resource)]
pub struct GameTimer(pub Timer);

pub struct GameTimerPlugin;

impl Plugin for GameTimerPlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(GameTimer(Timer::from_seconds(GAME_TIME, TimerMode::Once)))
            .add_systems(Update, tick_timer.run_if(in_state(GameState::Playing)));
    }
}

fn tick_timer(
    mut timer: ResMut<GameTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
){
    timer.0.tick(time.delta());

    if timer.0.just_finished(){
        next_state.set(GameState::GameOver);
    }
}
