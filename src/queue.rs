use bevy::prelude::*;
use std::collections::VecDeque;

use crate::{
    gameplay::moveable::{self, Movable},
    piece::{self, variant},
    state,
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
    query: Query<&piece::Piece, With<moveable::Movable>>,
    mut piece_queue: ResMut<Queue>,
    current_state: ResMut<State<state::BoardState>>,
) {
    if piece_queue.0.len() < variant::AMOUNT {
        piece_queue.0.extend(piece::random_pieces(7));
    }
    if query.is_empty() {
        let piece = piece_queue.0.pop_front().unwrap();

        let color = piece.variant.color();
        let visibility = Visibility::Visible;

        let shift = match piece.variant {
            variant::Variant::I => 19,
            _ => 18,
        };
        let board_state = current_state.get();
        let mut second_shift = shift;
        for block in piece.variant.blocks().iter_mut() {
            let height = board_state.height(block.x, block.y + shift);
            if height == shift {
                second_shift += 1;
                break;
            }
        }
        for block in piece.variant.blocks().iter_mut() {
            commands
                .spawn(*block.shift_y(second_shift))
                .insert(piece)
                .insert(block.sprite(color, visibility))
                .insert(Movable::default());
        }
    }
}
