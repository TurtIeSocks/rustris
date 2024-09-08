use super::*;

use crate::{
    audio,
    piece::{
        block::Block,
        tetrimino::{self, Tetrimino},
    },
    state,
    ui::hold,
};

const PLAY_DROP_SOUND: bool = false;

pub fn rotate_piece(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_piece: Query<(&mut Tetrimino, &mut Block, &mut Transform), With<moveable::Movable>>,
    q_board: Query<&Block, Without<moveable::Movable>>,
    current_state: Res<State<state::BoardState>>,
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
                    .shift_x(sum_x / 4 - sum_y / 4)
                    .shift_y(sum_x / 4 + sum_y / 4 + 1),
                _ => block
                    .reverse()
                    .shift_x(sum_x / 4 - sum_y / 4)
                    .shift_y(sum_x / 4 + sum_y / 4),
            };
            transform.translation = block.translation();
        }

        if is_colliding(&q_piece, &q_board, &current_state) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_x(-1);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board, &current_state) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_x(-1);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board, &current_state) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_x(3);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board, &current_state) {
            for (_, mut block, mut transform) in &mut q_piece {
                block.shift_x(3);
                transform.translation = block.translation();
            }
        }
        if is_colliding(&q_piece, &q_board, &current_state) {
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
            min_shift = min_shift.min(block.y - current_state.height(block.x));
        }
        for (mut block, mut transform, _) in &mut q_piece {
            block.shift_y(-min_shift);
            transform.translation = block.translation();
        }
    }
}

pub fn is_colliding(
    piece_query: &Query<(&mut Tetrimino, &mut Block, &mut Transform), With<moveable::Movable>>,
    board_query: &Query<&Block, Without<moveable::Movable>>,
    current_state: &Res<State<state::BoardState>>,
) -> bool {
    for (_, block, _) in piece_query {
        if block.x < 0 {
            info!("Colliding left");
            return true;
        }
        if block.x > 9 {
            println!("Colliding right");
            return true;
        }
        if block.y < 0 {
            println!("Colliding down");
            return true;
        }
    }

    // for (_, block, _) in piece_query {
    //     if block.x < 0 || current_state.check_collision(block.x - 1, block.y) {
    //         println!("Colliding left");
    //         return true;
    //     }
    //     if block.x > 9 || current_state.check_collision(block.x + 1, block.y) {
    //         return true;
    //     }
    //     if block.y < 0 || current_state.check_collision(block.x, block.y - 1) {
    //         println!("Colliding down");
    //         return true;
    //     }
    // }

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
        let mut already_down = false;
        if auto_move_timer.0.just_finished() && movable.can_down {
            block.shift_y(-1);

            if PLAY_DROP_SOUND {
                game_audios.play(&mut commands, "drop");
            }
            already_down = true;
        }
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
            if keyboard_input.pressed(KeyCode::ArrowDown) && movable.can_down && !already_down {
                block.shift_y(-1);
                if PLAY_DROP_SOUND {
                    game_audios.play(&mut commands, "drop");
                }
                reset_manually_move_timer = true;
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
    mut q_piece_blocks: Query<(Entity, &mut Block), With<moveable::Movable>>,
    mut held_piece_blocks: Query<
        (Entity, &mut Block),
        (With<tetrimino::Tetrimino>, Without<moveable::Movable>),
    >,
    mut held_res: ResMut<hold::Hold>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyC) && held_res.0 {
        let (mut x, mut y) = (i32::MAX, i32::MAX);
        for (_, block) in &mut q_piece_blocks {
            let (block_x, block_y) = block.dist_to_hold();
            x = x.min(block_x);
            y = y.min(block_y);
        }
        for (entity, mut block) in &mut q_piece_blocks {
            block.shift_x(x).shift_y(y);
            commands.entity(entity).remove::<moveable::Movable>();
            held_res.set(false);
        }
        for (entity, mut block) in &mut held_piece_blocks {
            block.shift_x(9);
            commands.entity(entity).insert(moveable::Movable::default());
        }
    }
}
