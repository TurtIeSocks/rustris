use super::*;

use board::{COL_COUNT, ROW_COUNT};
use line::Shape;

pub const HOLD_X: i32 = -3;
pub const HOLD_Y: i32 = 18;

#[derive(Debug, Resource)]
pub struct Hold(pub bool);

impl Hold {
    pub fn set(&mut self, value: bool) {
        self.0 = value;
    }
}

pub fn setup(mut commands: Commands) {
    let (x, y, scale) = (-COL_COUNT + 1., ROW_COUNT / 2.0, 4.5);

    SpriteBundle::square(scale, x, y)
        .into_iter()
        .for_each(|shape| {
            commands.spawn(shape);
        });

    commands.insert_resource(Hold(true))
}
