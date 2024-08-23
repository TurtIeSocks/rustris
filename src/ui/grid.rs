use crate::piece::block::BLOCK_LENGTH;

use super::*;

pub fn setup(windows: Query<&Window>, mut gizmos: Gizmos) {
    let window = windows.single();
    let x = (window.width() as f32 / BLOCK_LENGTH).ceil() as u32;
    let y = (window.height() as f32 / BLOCK_LENGTH).ceil() as u32;

    gizmos
        .grid_2d(
            Vec2::ZERO,
            0.0,
            UVec2::new(x * 2, y * 2),
            Vec2::new(BLOCK_LENGTH, BLOCK_LENGTH),
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}
