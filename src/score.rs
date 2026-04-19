use bevy::prelude::*;
use crate::timer::GameTimer;

#[derive(Resource)]
pub struct Score(pub u32);

pub struct ScorePlugin;

impl Plugin for ScorePlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(Score(0))
            .add_systems(Startup, setup_score_ui)
            .add_systems(Update, update_score_ui);
    }
}

fn setup_score_ui(mut commands: Commands){
    commands.spawn((
        ScoreText,
        Text::new("Score: 0"),
        TextFont{
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node{
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

#[derive(Component)]
struct ScoreText;

fn update_score_ui(
    score: Res<Score>,
    timer: Res<GameTimer>,
    mut query: Query<&mut Text, With<ScoreText>>,
){
    let Ok(mut text) = query.single_mut() else { return; };
    let remaining = (timer.0.remaining_secs() as u32).min(60);
    **text = format!("Score: {}\nTime Left: {}", score.0, remaining);
}
