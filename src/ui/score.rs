use super::*;

use crate::ui::stats;

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct Scoreboard;

pub fn setup(mut commands: Commands, windows: Query<&Window>) {
    commands
        .spawn(stats::get_stats_ui(windows, "Score: ", 0.))
        .insert(Scoreboard);
}

pub fn reset(mut score: ResMut<Score>) {
    score.0 = 0;
}

pub fn update(score: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.0.to_string();
}
