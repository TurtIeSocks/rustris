use bevy::color::palettes::css::WHITE;

use super::*;

use crate::piece::block::BLOCK_LENGTH;

const BORDER_THICKNESS: f32 = 10.0;
const BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

pub enum Side {
    Unset,
    Top,
    Right,
    Bottom,
    Left,
}

pub trait Shape {
    fn line(horizontal: bool, scale: f32, x: f32, y: f32, side: Side) -> SpriteBundle;
    fn square(scale: f32, x: f32, y: f32) -> [SpriteBundle; 4];
}

fn get_trans(val: f32, side: &Side) -> f32 {
    if val == 0. {
        0.
    } else {
        (match side {
            Side::Unset => 0.,
            Side::Top => BORDER_THICKNESS,
            Side::Right => BORDER_THICKNESS,
            Side::Bottom => -BORDER_THICKNESS,
            Side::Left => -BORDER_THICKNESS,
        } / 2.0)
    }
}

const UNSET: Side = Side::Unset;

impl Shape for SpriteBundle {
    fn line(horizontal: bool, scale: f32, x: f32, y: f32, side: Side) -> Self {
        let offset = 2.;
        let (one, two) = (
            BLOCK_LENGTH * scale + offset * BORDER_THICKNESS,
            BORDER_THICKNESS,
        );
        Self {
            transform: Transform {
                translation: Vec3 {
                    x: x * BLOCK_LENGTH + get_trans(x, if horizontal { &UNSET } else { &side }),
                    y: y * BLOCK_LENGTH + get_trans(y, if horizontal { &side } else { &UNSET }),
                    z: 0.0,
                },
                scale: Vec3 {
                    x: if horizontal { one } else { two },
                    y: if horizontal { two } else { one },
                    z: 0.0,
                },
                ..default()
            },
            sprite: Sprite {
                color: if horizontal {
                    BORDER_COLOR
                } else {
                    Color::Srgba(WHITE)
                },
                ..default()
            },
            ..default()
        }
    }

    fn square(scale: f32, x: f32, y: f32) -> [Self; 4] {
        let div = 2.;
        [
            SpriteBundle::line(true, scale, x, y, Side::Top),
            SpriteBundle::line(false, scale, x + scale / div, y - scale / div, Side::Right),
            SpriteBundle::line(true, scale, x, y - scale, Side::Bottom),
            SpriteBundle::line(false, scale, x - scale / div, y - scale / div, Side::Left),
        ]
    }
}
