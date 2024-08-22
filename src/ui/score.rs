use super::*;

use crate::ui::stats;

#[derive(Resource, Default)]
pub struct Score(u32);

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

impl Score {
    pub fn increment(&mut self, lines: usize) {
        self.0 += match lines {
            0 => 0,
            1 => 100,
            2 => 200,
            3 => 400,
            4 => 800,
            _ => 1000,
        };
    }
}

pub fn update(score: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.0.to_string();
}
