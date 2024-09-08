use super::block;

use bevy::color::palettes;
use bevy::prelude::*;

const SHAPE_I: [[i32; 2]; 4] = [[3, 0], [4, 0], [5, 0], [6, 0]];
const SHAPE_J: [[i32; 2]; 4] = [[3, 1], [3, 0], [4, 0], [5, 0]];
const SHAPE_L: [[i32; 2]; 4] = [[3, 0], [4, 0], [5, 0], [5, 1]];
const SHAPE_O: [[i32; 2]; 4] = [[4, 1], [4, 0], [5, 1], [5, 0]];
const SHAPE_S: [[i32; 2]; 4] = [[3, 0], [4, 0], [4, 1], [5, 1]];
const SHAPE_T: [[i32; 2]; 4] = [[3, 0], [4, 1], [4, 0], [5, 0]];
const SHAPE_Z: [[i32; 2]; 4] = [[3, 1], [4, 1], [4, 0], [5, 0]];

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Variant {
    // ####
    I,

    // #
    // ###
    J,

    //   #
    // ###
    L,

    // ##
    // ##
    O,

    //  ##
    // ##
    S,

    //  #
    // ###
    T,

    // ##
    //  ##
    Z,
}

pub const AMOUNT: usize = 7;

impl Variant {
    pub fn color(&self) -> Color {
        Color::Srgba(match self {
            Variant::I => palettes::css::LIGHT_CYAN,
            Variant::J => palettes::css::BLUE,
            Variant::L => palettes::css::ORANGE,
            Variant::O => palettes::css::YELLOW,
            Variant::S => palettes::css::GREEN,
            Variant::T => palettes::css::PURPLE,
            Variant::Z => palettes::css::RED,
        })
    }

    pub fn blocks(&self) -> [block::Block; 4] {
        match self {
            Variant::I => SHAPE_I.map(|pos| pos.into()).into(),
            Variant::J => SHAPE_J.map(|pos| pos.into()).into(),
            Variant::L => SHAPE_L.map(|pos| pos.into()).into(),
            Variant::O => SHAPE_O.map(|pos| pos.into()).into(),
            Variant::S => SHAPE_S.map(|pos| pos.into()).into(),
            Variant::T => SHAPE_T.map(|pos| pos.into()).into(),
            Variant::Z => SHAPE_Z.map(|pos| pos.into()).into(),
        }
    }
}

impl From<usize> for Variant {
    fn from(value: usize) -> Self {
        match value {
            0 => Variant::I,
            1 => Variant::J,
            2 => Variant::L,
            3 => Variant::O,
            4 => Variant::S,
            5 => Variant::T,
            6 => Variant::Z,
            _ => panic!("Unexpected value"),
        }
    }
}
