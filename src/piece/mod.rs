use bevy::prelude::*;

use std::collections::BTreeSet;

use rand::Rng;

pub mod block;
mod rotation;
pub mod tetrimino;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    pub variant: tetrimino::Tetrimino,
    pub rotation: rotation::Rotation,
}

impl Piece {
    fn new(variant: tetrimino::Tetrimino) -> Self {
        Piece {
            variant,
            rotation: rotation::Rotation::Zero,
        }
    }

    pub fn rotate(self) -> Self {
        let mut piece = self;
        piece.rotation.next();
        piece
    }
}

pub fn random_pieces(amount: usize) -> Vec<Piece> {
    let mut rng = rand::thread_rng();
    let mut piece_type_set = BTreeSet::new();

    for _ in 0..amount {
        piece_type_set.insert(Piece::new(
            rng.gen_range(0..tetrimino::Tetrimino::AMOUNT).into(),
        ));
    }

    piece_type_set.into_iter().collect()
}

pub fn control_visibility(
    mut q_piece: Query<(&mut Visibility, &block::Block), With<tetrimino::Tetrimino>>,
) {
    for (mut visibility, block) in &mut q_piece {
        if block.y > 19 {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}
