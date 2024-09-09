use moveable::Movable;

use super::*;
use crate::{
    audio,
    piece::{block::Block, ghost::Ghost, Piece},
    state,
    ui::{
        self,
        board::{COL_COUNT, ROW_COUNT},
        hold,
    },
};

pub fn check_collision(
    mut piece_query: Query<(&mut Block, &mut moveable::Movable), With<moveable::Movable>>,
    current_state: ResMut<State<state::BoardState>>,
) {
    let mut can_down = true;
    let mut can_left = true;
    let mut can_right = true;

    let current_state = current_state.get();
    for (block, _) in &mut piece_query {
        if block.x == 0 || current_state.check_collision(block.x - 1, block.y) {
            can_left = false;
        }
        if block.x == (COL_COUNT - 1.) as i32 || current_state.check_collision(block.x + 1, block.y)
        {
            can_right = false;
        }
        if block.y == 0 || current_state.check_collision(block.x, block.y - 1) {
            can_down = false;
        }
    }

    for (_, mut movable) in &mut piece_query {
        movable.can_left = can_left;
        movable.can_right = can_right;
        movable.can_down = can_down;
    }
}

pub fn draw_ghost(
    mut commands: Commands,
    mut piece_query: Query<(&Block, &Piece), With<moveable::Movable>>,
    mut ghost_query: Query<Entity, With<Ghost>>,
    current_state: ResMut<State<state::BoardState>>,
) {
    let current_state = current_state.get();
    let mut min_shift = i32::MAX;
    for (block, _) in &mut piece_query {
        min_shift = min_shift.min(block.y - current_state.height(block.x, block.y));
    }
    for block in &mut ghost_query {
        commands.entity(block).despawn();
    }
    for (block, piece) in &mut piece_query {
        let mut ghost_block = block.clone();
        ghost_block.shift_y(-min_shift);
        commands
            .spawn(ghost_block.ghost(piece.variant.color(), Visibility::Inherited))
            .insert(Ghost);
    }
}

pub fn remove_piece_component(
    mut commands: Commands,
    q_piece_blocks: Query<(Entity, &Block, &moveable::Movable), With<Movable>>,
    mut timer: ResMut<timers::RemovePieceComponent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    current_state: ResMut<State<state::BoardState>>,
    mut change_board_state: ResMut<NextState<state::BoardState>>,
    mut held_res: ResMut<hold::Hold>,
) {
    if !q_piece_blocks.is_empty() && !q_piece_blocks.iter().last().unwrap().2.can_down {
        if !q_piece_blocks.iter().last().unwrap().2.can_down {
            timer.0.tick(time.delta());
        } else {
            timer.0.reset();
        }
    }
    let mut reset_timer: bool = false;
    let mut next_board_state = current_state.get().clone();
    next_board_state.set_prev();
    for (entity, block, movable) in &q_piece_blocks {
        if !movable.can_down {
            if timer.0.just_finished()
                || keyboard_input.pressed(KeyCode::ArrowDown)
                || keyboard_input.pressed(KeyCode::Space)
            {
                next_board_state.place_block(block);
                commands
                    .entity(entity)
                    .remove::<Piece>()
                    .remove::<moveable::Movable>();
                reset_timer = true;
            }
        }
    }
    if reset_timer {
        info!("\n{}", next_board_state);
        change_board_state.set(next_board_state);
        timer.0.reset();
        held_res.set(true);
    }
}

pub fn check_game_over(
    mut commands: Commands,
    game_audios: Res<audio::GameAudio>,
    mut app_state: ResMut<NextState<state::AppState>>,
    mut game_state: ResMut<NextState<state::GameState>>,
    q_piece_blocks: Query<(&Block, &moveable::Movable), With<Movable>>,
) {
    for (block, movable) in &q_piece_blocks {
        if !movable.can_down && block.y >= ROW_COUNT as i32 {
            game_audios.play(&mut commands, "gameover");
            app_state.set(state::AppState::GameOver);
            game_state.set(state::GameState::GameQuit);
        }
    }
}

pub fn check_full_line(
    mut commands: Commands,
    game_audios: Res<audio::GameAudio>,
    mut score: ResMut<ui::score::Score>,
    mut lines: ResMut<ui::lines::Lines>,
    mut query: Query<(Entity, &mut Block, &mut Transform), Without<Piece>>,
    current_state: ResMut<State<state::BoardState>>,
    mut change_board_state: ResMut<NextState<state::BoardState>>,
) {
    let mut next_board_state = current_state.get().clone();
    let mut full_lines = next_board_state.full_lines();
    if full_lines.len() > 0 {
        game_audios.play(&mut commands, "line_clear")
    }
    lines.increment(full_lines.len());
    score.increment(full_lines.len());

    let mut despawn_entities = Vec::new();
    for line_no in full_lines.iter() {
        let line_no = *line_no as i32;
        for (entity, block, _) in &mut query {
            if block.y == line_no {
                despawn_entities.push(entity);
                commands.entity(entity).despawn();
            }
        }
    }
    full_lines.sort();
    full_lines.reverse();
    next_board_state.set_prev();
    for line_no in full_lines.iter() {
        next_board_state.clear_line(*line_no);
        for (entity, mut block, mut transform) in &mut query {
            if !despawn_entities.contains(&entity) && block.y > *line_no as i32 {
                block.y -= 1;
                transform.translation = block.translation();
            }
        }
    }
    if full_lines.len() > 0 {
        info!("\n{}", next_board_state);
        change_board_state.set(next_board_state);
    }
}
