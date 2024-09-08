use bevy::prelude::*;

use std::collections::BTreeSet;

use rand::Rng;

pub mod block;
pub mod ghost;
pub mod variant;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    pub variant: variant::Variant,
}

impl Piece {
    fn new(variant: variant::Variant) -> Self {
        Piece { variant }
    }
}

pub fn random_pieces(amount: usize) -> Vec<Piece> {
    let mut rng = rand::thread_rng();
    let mut piece_type_set = BTreeSet::new();

    for _ in 0..amount {
        piece_type_set.insert(Piece::new(rng.gen_range(0..variant::AMOUNT).into()));
    }

    piece_type_set.into_iter().collect()
}

pub fn control_visibility(mut q_piece: Query<(&mut Visibility, &block::Block), With<Piece>>) {
    for (mut visibility, block) in &mut q_piece {
        if block.y > 19 {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}
