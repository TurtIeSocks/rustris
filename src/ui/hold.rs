use board::{COL_COUNT, ROW_COUNT};
use line::Shape;

use super::*;

pub fn setup(mut commands: Commands) {
    let (x, y, scale) = (-COL_COUNT + 1., ROW_COUNT / 2.0, 6.);

    SpriteBundle::square(scale, x, y)
        .into_iter()
        .for_each(|shape| {
            commands.spawn(shape);
        });
}
