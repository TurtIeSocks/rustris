use std::fmt::Display;

use bevy::prelude::*;

use crate::{
    piece::block::Block,
    ui::board::{COL_COUNT, ROW_COUNT},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
    GameOver,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default]
    GamePlaying,
    GamePaused,
    GameRestarted,
    GameQuit,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub struct BoardState {
    board: [[bool; COL_COUNT as usize]; ROW_COUNT as usize],
    heights: [i32; COL_COUNT as usize],
}

impl Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌{}┐", "─".repeat(COL_COUNT as usize))?;
        for row in self.board.iter().rev() {
            write!(f, "│")?;
            for cell in row.iter() {
                write!(f, "{}", if *cell { "X" } else { " " })?;
            }
            write!(f, "│")?;
            writeln!(f)?;
        }
        writeln!(f, "└{}┘", "─".repeat(COL_COUNT as usize))?;
        write!(f, "|")?;
        for height in self.heights.iter() {
            write!(f, "{}", height)?;
        }
        write!(f, "|")?;
        Ok(())
    }
}

impl BoardState {
    pub fn place_block(&mut self, block: &Block) {
        if block.y < 0 || block.y >= ROW_COUNT as i32 {
            return;
        }
        if block.x < 0 || block.x >= COL_COUNT as i32 {
            return;
        }
        self.board[block.y as usize][block.x as usize] = true;
        self.heights[block.x as usize] = self.heights[block.x as usize].max(block.y + 1);
    }

    pub fn clear_line(&mut self, y: usize) {
        self.board[y] = [false; COL_COUNT as usize];
        for row in (y + 1)..(ROW_COUNT as usize) {
            self.board[row - 1] = self.board[row];
        }
        for col in 0..COL_COUNT as usize {
            self.heights[col] -= 1;
        }
    }

    pub fn full_lines(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(y, row)| {
                if row.iter().all(|&cell| cell) {
                    Some(y)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn height(&self, x: i32) -> i32 {
        if x >= COL_COUNT as i32 {
            return 0;
        }
        self.heights[x as usize]
    }

    pub fn check_collision(&self, x: i32, y: i32) -> bool {
        if y >= ROW_COUNT as i32 {
            return false;
        }
        y < 0 || x < 0 || x >= COL_COUNT as i32 || self.board[y as usize][x as usize]
    }
}

pub fn pause_game(
    game_state: Res<State<GameState>>,
    mut change_game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let GameState::GamePlaying = game_state.get() {
            change_game_state.set(GameState::GamePaused);
        } else {
            change_game_state.set(GameState::GamePlaying);
        }
    }
}

pub fn play_game(
    mut game_state: ResMut<NextState<GameState>>,
    mut board_state: ResMut<NextState<BoardState>>,
) {
    game_state.set(GameState::GamePlaying);
    board_state.set(BoardState::default());
}
