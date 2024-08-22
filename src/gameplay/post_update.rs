use super::*;
use crate::{
    audio,
    piece::{block::Block, tetrimino::Tetrimino},
    state,
    ui::{
        self,
        board::{COL_COUNT, ROW_COUNT},
    },
};

pub fn check_collision(
    mut piece_query: Query<(&mut Block, &mut moveable::Movable), With<Tetrimino>>,
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

pub fn remove_piece_component(
    mut commands: Commands,
    q_piece_blocks: Query<(Entity, &Block, &moveable::Movable), With<Tetrimino>>,
    mut timer: ResMut<timers::RemovePieceComponent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    current_state: ResMut<State<state::BoardState>>,
    mut change_board_state: ResMut<NextState<state::BoardState>>,
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
    for (entity, block, movable) in &q_piece_blocks {
        if !movable.can_down {
            if timer.0.just_finished() || keyboard_input.pressed(KeyCode::ArrowDown) {
                next_board_state.place_block(block);
                commands.entity(entity).remove::<Tetrimino>();
                reset_timer = true;
            }
        }
    }
    if reset_timer {
        println!("{}", next_board_state);
        change_board_state.set(next_board_state);
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
    if max_block_y >= (ROW_COUNT - 1.) as i32 {
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
    for line_no in full_lines.iter() {
        next_board_state.clear_line(*line_no);
        for (entity, mut block, mut transform) in &mut query {
            if !despawn_entities.contains(&entity) && block.y > *line_no as i32 {
                info!("down block: {:?}, line_no: {}", block, line_no);
                block.y -= 1;
                transform.translation = block.translation();
            }
        }
    }
    if full_lines.len() > 0 {
        println!("{}", next_board_state);
        change_board_state.set(next_board_state);
    }
}
