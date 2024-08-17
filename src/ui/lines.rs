use bevy::prelude::*;

use crate::ui::stats;

#[derive(Resource)]
pub struct Lines(pub u32);

#[derive(Component)]
pub struct Linesboard;

pub fn setup(mut commands: Commands, windows: Query<&Window>) {
    commands
        .spawn(stats::get_stats_ui(
            windows,
            "Lines: ",
            stats::STATS_BOARD_WIDTH,
        ))
        .insert(Linesboard);
}

pub fn reset(mut lines: ResMut<Lines>) {
    lines.0 = 0;
}

pub fn update(lines: Res<Lines>, mut query: Query<&mut Text, With<Linesboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = lines.0.to_string();
}
