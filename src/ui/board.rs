use line::{Shape, Side};

use super::*;

pub const COL_COUNT: f32 = 10.;
pub const ROW_COUNT: f32 = 20.;

pub fn setup(mut commands: Commands) {
    let half_col_count = COL_COUNT / 2.0;
    let half_row_count = ROW_COUNT / 2.0;

    commands.spawn(SpriteBundle::line(
        true,
        COL_COUNT,
        0.,
        half_row_count,
        Side::Top,
    ));
    commands.spawn(SpriteBundle::line(
        true,
        COL_COUNT,
        0.,
        -half_row_count,
        Side::Bottom,
    ));
    commands.spawn(SpriteBundle::line(
        false,
        ROW_COUNT,
        -half_col_count,
        0.,
        Side::Left,
    ));
    commands.spawn(SpriteBundle::line(
        false,
        ROW_COUNT,
        half_col_count,
        0.,
        Side::Right,
    ));
}
