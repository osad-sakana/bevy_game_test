use bevy::prelude::*;
use crate::game_state::GameState;
use crate::score::Score;
use crate::timer::GameTimer;

pub struct UiPlugin;

impl Plugin for UiPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::GameOver), show_gameover)
            .add_systems(OnExit(GameState::GameOver), hide_gameover)
            .add_systems(Update, restart.run_if(in_state(GameState::GameOver)));
    }
}

#[derive(Component)]
struct GameOverScreen;

fn show_gameover(
    mut commands: Commands,
    score: Res<Score>,
){
    commands.spawn((
        GameOverScreen,
        Text::new(format!("Game Over\nFinal Score: {}\n\nPress R to Restart", score.0)),
        TextFont{
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node{
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            left: Val::Percent(50.0),
            ..default()
        },
    ));
}

fn hide_gameover(
    mut commands: Commands,
    query: Query<Entity, With<GameOverScreen>>,
){
    for entity in &query{
        commands.entity(entity).despawn();
    }
}

fn restart(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
    mut timer: ResMut<GameTimer>,
){
    if keyboard.just_pressed(KeyCode::KeyR){
        score.0 = 0;
        timer.0.reset();
        next_state.set(GameState::Playing);
    }
}
