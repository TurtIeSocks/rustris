use super::*;

use crate::piece::block::BLOCK_LENGTH;

pub const STATS_BOARD_LENGTH: f32 = 220.0;
pub const STATS_BOARD_WIDTH: f32 = 50.0;

pub fn get_stats_ui(windows: Query<&Window>, label: &str, offset: f32) -> TextBundle {
    let window = windows.single();
    let (left, bottom) = (
        window.physical_width() as f32 / 2.0 - 5. * BLOCK_LENGTH,
        window.physical_height() as f32 / 2.0 - 10. * BLOCK_LENGTH,
    );
    TextBundle::from_sections([
        TextSection::new(
            label,
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(0.5, 0.5, 1.0),
                ..default()
            },
        ),
        TextSection::new(
            "0",
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(1.0, 0.5, 0.5),
                ..default()
            },
        ),
    ])
    .with_style(Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(bottom + offset),
        left: Val::Px(left - STATS_BOARD_LENGTH),
        ..default()
    })
}
