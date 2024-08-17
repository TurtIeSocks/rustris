use std::collections::{HashMap, HashSet};

use super::*;
use crate::{
    audio,
    piece::{block::Block, tetrimino::Tetrimino},
    state,
    ui::{self, board::COL_COUNT},
};

pub fn check_collision(
    mut piece_query: Query<(&mut Block, &mut moveable::Movable), With<Tetrimino>>,
    board_query: Query<&Block, Without<Tetrimino>>,
) {
    let mut can_down = true;
    let mut can_left = true;
    let mut can_right = true;

    for (block, _) in &mut piece_query {
        if block.x == 0 {
            can_left = false;
        }
        if block.x == 9 {
            can_right = false;
        }
        if block.y == 0 {
            can_down = false;
        }
    }

    for (block, _) in &piece_query {
        for board_block in &board_query {
            if board_block.y == block.y && block.x > 0 && board_block.x == block.x - 1 {
                can_left = false;
            }
            if board_block.y == block.y && board_block.x == block.x + 1 {
                can_right = false;
            }
            if board_block.x == block.x && block.y > 0 && board_block.y == block.y - 1 {
                can_down = false;
            }
        }
    }

    for (_, mut movable) in &mut piece_query {
        movable.can_left = can_left;
        movable.can_right = can_right;
        movable.can_down = can_down;
    }
}

pub fn remove_piece_component(
    mut commands: Commands,
    q_piece_blocks: Query<(Entity, &moveable::Movable), With<Tetrimino>>,
    mut timer: ResMut<timers::RemovePieceComponent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if !q_piece_blocks.is_empty() && !q_piece_blocks.iter().last().unwrap().1.can_down {
        if !q_piece_blocks.iter().last().unwrap().1.can_down {
            timer.0.tick(time.delta());
        } else {
            timer.0.reset();
        }
    }
    let mut reset_timer = false;
    for (entity, movable) in &q_piece_blocks {
        if !movable.can_down {
            if timer.0.just_finished() || keyboard_input.pressed(KeyCode::ArrowDown) {
                commands.entity(entity).remove::<Tetrimino>();
                reset_timer = true;
            }
        }
    }
    if reset_timer {
        timer.0.reset();
    }
}

pub fn check_game_over(
    mut commands: Commands,
    game_audios: Res<audio::GameAudio>,
    mut app_state: ResMut<NextState<state::AppState>>,
    mut game_state: ResMut<NextState<state::GameState>>,
    query: Query<&Block, Without<Tetrimino>>,
) {
    let mut max_block_y = 0;
    for block in &query {
        if block.y > max_block_y {
            max_block_y = block.y;
        }
    }
    if max_block_y >= 19 {
        game_audios.play(&mut commands, "gameover");
        app_state.set(state::AppState::GameOver);
        game_state.set(state::GameState::GameQuit);
    }
}

pub fn check_full_line(
    mut commands: Commands,
    game_audios: Res<audio::GameAudio>,
    mut score: ResMut<ui::score::Score>,
    mut lines: ResMut<ui::lines::Lines>,
    mut query: Query<(Entity, &mut Block, &mut Transform), Without<Tetrimino>>,
) {
    let mut y_to_x_set_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (_, block, _) in &query {
        if y_to_x_set_map.contains_key(&block.y) {
            let x_set = y_to_x_set_map.get_mut(&block.y).unwrap();
            x_set.insert(block.x);
        } else {
            let mut x_set = HashSet::new();
            x_set.insert(block.x);
            y_to_x_set_map.insert(block.y, x_set);
        }
    }
    let mut full_lines = Vec::new();
    for (y, x_set) in y_to_x_set_map.iter() {
        if x_set.len() == COL_COUNT as usize {
            full_lines.push(y);
        }
    }
    if full_lines.len() > 0 {
        dbg!(full_lines.len());
        game_audios.play(&mut commands, "line_clear")
    }
    lines.0 += full_lines.len() as u32;
    score.0 += match full_lines.len() {
        0 => 0,
        1 => 100,
        2 => 200,
        3 => 400,
        4 => 800,
        _ => 1000,
    };

    let mut despawn_entities = Vec::new();
    for line_no in full_lines.iter() {
        let line_no = **line_no;
        for (entity, block, _) in &mut query {
            if block.y == line_no {
                despawn_entities.push(entity);
                commands.entity(entity).despawn();
            }
        }
    }
    full_lines.sort();
    full_lines.reverse();
    for line_no in full_lines.iter() {
        for (entity, mut block, mut transform) in &mut query {
            if !despawn_entities.contains(&entity) && block.y > **line_no {
                info!("down block: {:?}, line_no: {}", block, line_no);
                block.y -= 1;
                transform.translation = block.translation();
            }
        }
    }
}
