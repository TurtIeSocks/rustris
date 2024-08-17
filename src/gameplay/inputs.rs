use super::*;

use crate::{
    audio,
    piece::{block::Block, tetrimino::Tetrimino},
};

pub fn rotate_piece(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_piece: Query<(&mut Tetrimino, &mut Block, &mut Transform)>,
    q_board: Query<&Block, Without<Tetrimino>>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        let tetrimino = match q_piece.into_iter().next() {
            Some((piece_type, _, _)) => *piece_type,
            None => {
                return;
            }
        };
        let sum_x = q_piece.iter().map(|(_, block, _)| block.x).sum::<i32>();
        let sum_y = q_piece.iter().map(|(_, block, _)| block.y).sum::<i32>();

        let original_blocks: Vec<Block> =
            q_piece.iter().map(|(_, block, _)| block.clone()).collect();

        for (_, mut block, mut transform) in &mut q_piece {
            match tetrimino {
                Tetrimino::O | Tetrimino::L | Tetrimino::J => block
                    .reverse()
                    .shift_mut(Some(sum_x / 4 - sum_y / 4), Some(sum_x / 4 + sum_y / 4 + 1)),
                _ => block
                    .reverse()
                    .shift_mut(Some(sum_x / 4 - sum_y / 4), Some(sum_x / 4 + sum_y / 4)),
            };
            transform.translation = block.translation();
        }

        if is_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_mut(Some(-1), None);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_mut(Some(-1), None);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_mut(Some(3), None);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_mut(Some(3), None);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board) {
            let mut index = 0;
            for (_, mut block, mut transform) in &mut q_piece {
                *block = original_blocks[index];
                transform.translation = block.translation();
                index += 1;
            }
        }
    }
}

pub fn is_colliding(
    piece_query: &Query<(&mut Tetrimino, &mut Block, &mut Transform)>,
    board_query: &Query<&Block, Without<Tetrimino>>,
) -> bool {
    for (_, block, _) in piece_query {
        if block.x < 0 {
            return true;
        }
        if block.x > 9 {
            return true;
        }
        if block.y < 0 {
            return true;
        }
    }

    for (_, block, _) in piece_query {
        for board_block in board_query {
            if board_block.y == block.y && block.x > 0 && board_block.x == block.x - 1 {
                return true;
            }
            if board_block.y == block.y && board_block.x == block.x + 1 {
                return true;
            }
            if board_block.x == block.x && block.y > 0 && board_block.y == block.y - 1 {
                return true;
            }
        }
    }
    return false;
}

pub fn move_piece(
    mut commands: Commands,
    game_audios: Res<audio::GameAudio>,
    mut query: Query<(&mut Block, &mut Transform, &moveable::Movable), With<Tetrimino>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut manually_move_timer: ResMut<timers::ManualMove>,
    mut auto_move_timer: ResMut<timers::AutoMove>,
    time: Res<Time>,
) {
    manually_move_timer.0.tick(time.delta());
    auto_move_timer.0.tick(time.delta());
    let mut reset_manually_move_timer = false;
    for (mut block, mut transform, movable) in &mut query {
        let mut already_down = false;
        if auto_move_timer.0.just_finished() && movable.can_down {
            block.y -= 1;

            game_audios.play(&mut commands, "drop");
            already_down = true;
        }
        if manually_move_timer.0.finished() {
            if keyboard_input.pressed(KeyCode::ArrowLeft) && movable.can_left {
                block.x -= 1;
                game_audios.play(&mut commands, "drop");
                reset_manually_move_timer = true;
            } else if keyboard_input.pressed(KeyCode::ArrowRight) && movable.can_right {
                block.x += 1;
                game_audios.play(&mut commands, "drop");
                reset_manually_move_timer = true;
            } else if keyboard_input.pressed(KeyCode::ArrowDown)
                && movable.can_down
                && !already_down
            {
                block.y -= 1;
                game_audios.play(&mut commands, "drop");
                reset_manually_move_timer = true;
            }
        }
        transform.translation = block.translation();
    }
    if reset_manually_move_timer {
        manually_move_timer.0.reset();
    }
}
