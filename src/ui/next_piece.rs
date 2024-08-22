use super::*;

use crate::{piece::Piece, queue};

#[derive(Debug, Resource)]
pub struct NextPieceType(pub Option<Piece>);

#[derive(Debug, Component)]
pub struct NextPieceBoard;

pub fn reset(mut commands: Commands, query: Query<Entity, With<NextPieceBoard>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn update(
    mut commands: Commands,
    piece_queue: Res<queue::Queue>,
    mut next_piece_type: ResMut<NextPieceType>,
    query: Query<Entity, With<NextPieceBoard>>,
) {
    let next_in_queue = *piece_queue.0.front().unwrap();

    if next_piece_type.0.is_none() || next_in_queue != next_piece_type.0.unwrap() {
        next_piece_type.0 = Some(next_in_queue);

        for entity in &query {
            commands.entity(entity).despawn();
        }
        let visibility = Visibility::Visible;
        let color = next_in_queue.variant.color();
        for block in next_in_queue.variant.blocks().iter_mut() {
            block.shift_x(8).shift_y(17);
            commands
                .spawn(block.sprite(color, visibility))
                .insert(NextPieceBoard);
        }
    }
}
