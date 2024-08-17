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
pub enum Tetrimino {
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

impl Tetrimino {
    pub const AMOUNT: usize = 7;

    pub fn color(&self) -> Color {
        Color::Srgba(match self {
            Tetrimino::I => palettes::css::LIGHT_CYAN,
            Tetrimino::J => palettes::css::BLUE,
            Tetrimino::L => palettes::css::ORANGE,
            Tetrimino::O => palettes::css::YELLOW,
            Tetrimino::S => palettes::css::GREEN,
            Tetrimino::T => palettes::css::PURPLE,
            Tetrimino::Z => palettes::css::RED,
        })
    }

    pub fn blocks(&self) -> [block::Block; 4] {
        match self {
            Tetrimino::I => SHAPE_I.map(|pos| pos.into()).into(),
            Tetrimino::J => SHAPE_J.map(|pos| pos.into()).into(),
            Tetrimino::L => SHAPE_L.map(|pos| pos.into()).into(),
            Tetrimino::O => SHAPE_O.map(|pos| pos.into()).into(),
            Tetrimino::S => SHAPE_S.map(|pos| pos.into()).into(),
            Tetrimino::T => SHAPE_T.map(|pos| pos.into()).into(),
            Tetrimino::Z => SHAPE_Z.map(|pos| pos.into()).into(),
        }
    }

    pub fn shift(self, delta_x: Option<i32>, delta_y: Option<i32>) -> Self {
        let mut blocks = self.blocks();
        match delta_x {
            Some(delta) => {
                blocks[0].x += delta;
                blocks[1].x += delta;
                blocks[2].x += delta;
                blocks[3].x += delta;
            }
            None => {}
        }
        match delta_y {
            Some(delta) => {
                blocks[0].y += delta;
                blocks[1].y += delta;
                blocks[2].y += delta;
                blocks[3].y += delta;
            }
            None => {}
        }
        self
    }
}

impl From<usize> for Tetrimino {
    fn from(value: usize) -> Self {
        match value {
            0 => Tetrimino::I,
            1 => Tetrimino::J,
            2 => Tetrimino::L,
            3 => Tetrimino::O,
            4 => Tetrimino::S,
            5 => Tetrimino::T,
            6 => Tetrimino::Z,
            _ => panic!("Unexpected value"),
        }
    }
}
