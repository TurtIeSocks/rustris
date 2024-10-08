use super::*;

use crate::ui::util;

#[derive(Resource, Default)]
pub struct Lines(u32);

#[derive(Component)]
pub struct Linesboard;

pub fn setup(mut commands: Commands, windows: Query<&Window>) {
    if windows.is_empty() {
        return;
    }

    commands
        .spawn(util::get_stats_ui(
            windows,
            "Lines: ",
            util::STATS_BOARD_WIDTH,
        ))
        .insert(Linesboard);
}

pub fn reset(mut lines: ResMut<Lines>) {
    lines.0 = 0;
}

impl Lines {
    pub fn increment(&mut self, lines: usize) {
        self.0 += lines as u32;
    }
}

pub fn update(lines: Res<Lines>, mut query: Query<&mut Text, With<Linesboard>>) {
    let mut text = query.single_mut();
    text.sections[2].value = lines.0.to_string();
}
