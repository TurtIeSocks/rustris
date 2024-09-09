use super::*;

use crate::{
    audio,
    piece::{block::Block, variant::Variant, Piece},
    state,
    ui::hold::{self, HOLD_X, HOLD_Y},
};

const PLAY_DROP_SOUND: bool = true;

pub fn rotate_piece(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_piece: Query<(&mut Piece, &mut Block, &mut Transform), With<moveable::Movable>>,
    q_board: Query<&Block, Without<moveable::Movable>>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        let sum_x = q_piece.iter().map(|(_, block, _)| block.x).sum::<i32>();
        let sum_y = q_piece.iter().map(|(_, block, _)| block.y).sum::<i32>();

        let original_blocks: Vec<Block> =
            q_piece.iter().map(|(_, block, _)| block.clone()).collect();

        for (piece, mut block, mut transform) in &mut q_piece {
            match piece.variant {
                Variant::O | Variant::L | Variant::J => block
                    .reverse()
                    .shift_x(sum_x / 4 - sum_y / 4)
                    .shift_y(sum_x / 4 + sum_y / 4 + 1),
                _ => block
                    .reverse()
                    .shift_x(sum_x / 4 - sum_y / 4)
                    .shift_y(sum_x / 4 + sum_y / 4),
            };
            transform.translation = block.translation();
        }

        if is_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_x(-1);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_x(-1);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_x(3);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_x(3);
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

pub fn send_to_bottom(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_piece: Query<(&mut Block, &mut Transform, &moveable::Movable), With<moveable::Movable>>,
    current_state: Res<State<state::BoardState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut min_shift = i32::MAX;
        for (block, _, _) in &mut q_piece {
            min_shift = min_shift.min(block.y - current_state.height(block.x, block.y));
        }
        for (mut block, mut transform, _) in &mut q_piece {
            block.shift_y(-min_shift);
            transform.translation = block.translation();
        }
    }
}

pub fn is_colliding(
    piece_query: &Query<(&mut Piece, &mut Block, &mut Transform), With<moveable::Movable>>,
    board_query: &Query<&Block, Without<moveable::Movable>>,
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
    mut query: Query<(&mut Block, &mut Transform, &moveable::Movable), With<moveable::Movable>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut manually_move_timer: ResMut<timers::ManualMove>,
    mut auto_move_timer: ResMut<timers::AutoMove>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        manually_move_timer.0.reset();
        auto_move_timer.0.reset();
        return;
    }
    manually_move_timer.0.tick(time.delta());
    auto_move_timer.0.tick(time.delta());

    let mut reset_manually_move_timer = false;
    for (mut block, mut transform, movable) in &mut query {
        if manually_move_timer.0.finished() {
            if keyboard_input.pressed(KeyCode::ArrowLeft) && movable.can_left {
                block.shift_x(-1);
                if PLAY_DROP_SOUND {
                    game_audios.play(&mut commands, "drop");
                }
                reset_manually_move_timer = true;
            } else if keyboard_input.pressed(KeyCode::ArrowRight) && movable.can_right {
                block.shift_x(1);
                if PLAY_DROP_SOUND {
                    game_audios.play(&mut commands, "drop");
                }
                reset_manually_move_timer = true;
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) && movable.can_down {
                block.shift_y(-1);
                if PLAY_DROP_SOUND {
                    game_audios.play(&mut commands, "drop");
                }
                reset_manually_move_timer = true;
                auto_move_timer.0.reset();
            }
        }
        if auto_move_timer.0.just_finished() && movable.can_down {
            block.shift_y(-1);

            if PLAY_DROP_SOUND {
                game_audios.play(&mut commands, "drop");
            }
        }

        transform.translation = block.translation();
    }
    if reset_manually_move_timer {
        manually_move_timer.0.reset();
    }
}

pub fn swap_piece(
    mut commands: Commands,
    mut q_piece_blocks: Query<(Entity, &mut Piece), With<moveable::Movable>>,
    mut held_piece_blocks: Query<(Entity, &mut Block), (With<Piece>, Without<moveable::Movable>)>,
    mut held_res: ResMut<hold::Hold>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyC) && held_res.0 {
        if let Some((_, piece)) = q_piece_blocks.iter().next() {
            let variant = piece.variant;
            let color = variant.color();
            let visibility = Visibility::Visible;
            for block in variant.blocks().iter_mut() {
                commands
                    .spawn(*block.shift_x(HOLD_X).shift_y(HOLD_Y))
                    .insert(*piece)
                    .insert(block.sprite(color, visibility));
            }
        }
        for (entity, _) in &mut q_piece_blocks {
            commands.entity(entity).despawn();
            held_res.set(false);
        }
        for (entity, mut block) in &mut held_piece_blocks {
            block.shift_x(9);
            commands.entity(entity).insert(moveable::Movable::default());
        }
    }
}
