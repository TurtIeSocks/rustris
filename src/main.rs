use std::time::Duration;

use bevy::prelude::*;

mod audio;
mod gameplay;
mod menus;
mod piece;
mod queue;
mod state;
mod ui;

fn main() {
    App::new()
        .insert_resource(ui::score::Score::default())
        .insert_resource(ui::lines::Lines::default())
        .insert_resource(ui::next_piece::NextPieceType(None))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(gameplay::timers::AutoMove(Timer::new(
            Duration::from_millis(1000),
            TimerMode::Repeating,
        )))
        .insert_resource(gameplay::timers::ManualMove(Timer::new(
            Duration::from_millis(100),
            TimerMode::Once,
        )))
        .insert_resource(gameplay::timers::RemovePieceComponent(Timer::new(
            Duration::from_millis(300),
            TimerMode::Once,
        )))
        .add_plugins(DefaultPlugins)
        .init_state::<state::AppState>()
        .init_state::<state::GameState>()
        .init_state::<state::BoardState>()
        .add_systems(
            Startup,
            (
                setup_camera,
                ui::board::setup,
                ui::score::setup,
                ui::lines::setup,
                ui::hold::setup,
                ui::next_piece::setup,
                audio::setup,
                queue::setup,
            ),
        )
        // Main Menu
        .add_systems(
            OnEnter(state::AppState::MainMenu),
            (
                menus::main::setup,
                piece::block::reset,
                ui::score::reset,
                ui::lines::reset,
                ui::next_piece::reset,
            ),
        )
        .add_systems(
            OnExit(state::AppState::MainMenu),
            menus::close_menu::<menus::main::MainMenu>,
        )
        // Gameover Menu
        .add_systems(OnEnter(state::AppState::GameOver), menus::game_over::setup)
        .add_systems(
            OnExit(state::AppState::GameOver),
            (
                menus::close_menu::<menus::game_over::GameOver>,
                piece::block::reset,
                ui::score::reset,
                ui::lines::reset,
                ui::next_piece::reset,
            ),
        )
        // Game Playing
        .add_systems(
            Update,
            (
                gameplay::inputs::move_piece,
                gameplay::inputs::rotate_piece,
                gameplay::inputs::send_to_bottom,
                gameplay::inputs::swap_piece,
                queue::auto_generate_new_piece,
                ui::grid::setup,
                ui::grid::manage_config,
                ui::score::update,
                ui::lines::update,
                ui::next_piece::update,
                piece::control_visibility,
            )
                .run_if(in_state(state::GameState::GamePlaying)),
        )
        .add_systems(
            PostUpdate,
            (
                gameplay::post_update::check_collision,
                gameplay::post_update::remove_piece_component,
                gameplay::post_update::draw_ghost,
                gameplay::post_update::check_game_over
                    .after(gameplay::post_update::remove_piece_component),
                gameplay::post_update::check_full_line
                    .after(gameplay::post_update::remove_piece_component)
                    .before(TransformSystem::TransformPropagate),
            )
                .run_if(in_state(state::GameState::GamePlaying)),
        )
        .add_systems(OnEnter(state::GameState::GamePaused), menus::pause::setup)
        // Game Paused
        .add_systems(
            OnExit(state::GameState::GamePaused),
            menus::close_menu::<menus::pause::PauseMenu>,
        )
        // Game Restarted
        .add_systems(
            OnEnter(state::GameState::GameRestarted),
            (piece::block::reset, ui::score::reset, ui::lines::reset),
        )
        .add_systems(
            Update,
            state::play_game.run_if(in_state(state::GameState::GameRestarted)),
        )
        // Common
        .add_systems(
            Update,
            state::pause_game.run_if(
                in_state(state::GameState::GamePlaying)
                    .or_else(in_state(state::GameState::GamePaused)),
            ),
        )
        .add_systems(
            Update,
            menus::button::click.run_if(
                in_state(state::AppState::MainMenu)
                    .or_else(in_state(state::AppState::GameOver))
                    .or_else(in_state(state::GameState::GamePaused)),
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
