use bevy::prelude::*;
use std::collections::VecDeque;

use crate::{
    gameplay::moveable::Movable,
    piece::{self, tetrimino},
};

#[derive(Debug, Resource)]
pub struct Queue(pub VecDeque<piece::Piece>);

impl Queue {
    pub fn new() -> Self {
        Queue(VecDeque::new())
    }
}

pub fn setup(mut commands: Commands) {
    let mut piece_queue = Queue::new();
    piece_queue.0.extend(piece::random_pieces(7));
    commands.insert_resource(piece_queue);
}

pub fn auto_generate_new_piece(
    mut commands: Commands,
    query: Query<&tetrimino::Tetrimino>,
    mut piece_queue: ResMut<Queue>,
) {
    if piece_queue.0.len() < tetrimino::Tetrimino::AMOUNT {
        piece_queue.0.extend(piece::random_pieces(7));
    }
    if query.is_empty() {
        let piece = piece_queue.0.pop_front().unwrap();
        let color = piece.variant.color();
        let visibility = Visibility::Hidden;

        for block in piece.variant.blocks().iter() {
            let block = block.shift(None, Some(20));
            commands
                .spawn(piece.variant)
                .insert(block.sprite(color, visibility))
                .insert(block)
                .insert(Movable::default());
        }
    }
}
