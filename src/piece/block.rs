use bevy::prelude::*;

use crate::ui::board;

pub const BLOCK_STICKER_LENGTH: f32 = 28.0;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn translation(&self) -> Vec3 {
        Vec3 {
            x: (self.x as f32 - (board::COL_COUNT / 2.0) + 0.5) * board::BLOCK_LENGTH,
            y: (self.y as f32 - (board::ROW_COUNT / 2.0) + 0.5) * board::BLOCK_LENGTH,
            z: 0.0,
        }
    }

    pub fn sprite(&self, color: Color, visibility: Visibility) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite { color, ..default() },
            transform: Transform {
                scale: Vec3::new(
                    BLOCK_STICKER_LENGTH,
                    BLOCK_STICKER_LENGTH,
                    BLOCK_STICKER_LENGTH,
                ),
                translation: self.translation(),
                ..default()
            },
            visibility,
            ..default()
        }
    }

    pub fn shift(self, delta_x: Option<i32>, delta_y: Option<i32>) -> Self {
        let mut new_block = self;
        new_block.shift_mut(delta_x, delta_y);
        new_block
    }

    pub fn shift_mut(&mut self, delta_x: Option<i32>, delta_y: Option<i32>) {
        match delta_x {
            Some(delta) => {
                self.x += delta;
            }
            None => {}
        }
        match delta_y {
            Some(delta) => {
                self.y += delta;
            }
            None => {}
        }
    }

    pub fn reverse(&mut self) -> &mut Self {
        let y = self.y;
        self.y = -self.x;
        self.x = y;
        self
    }
}

impl From<[i32; 2]> for Block {
    fn from([x, y]: [i32; 2]) -> Self {
        Block { x, y }
    }
}

pub fn reset(mut commands: Commands, query: Query<Entity, With<Block>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
