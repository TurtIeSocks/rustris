use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Rotation {
    Zero,
    One,
    Two,
    Three,
}

impl Rotation {
    pub fn next(&mut self) {
        *self = match self {
            Rotation::Zero => Rotation::One,
            Rotation::One => Rotation::Two,
            Rotation::Two => Rotation::Three,
            Rotation::Three => Rotation::Zero,
        };
    }
}
