use bevy::prelude::*;

use crate::state::{AppState, GameState};

#[derive(Component)]
pub enum Button {
    StartGame,
    RestartGame,
    BackToMainMenu,
    ResumeGame,
    Quit,
}

pub fn click(
    mut interaction_query: Query<(&Interaction, &Button), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match menu_button_action {
                Button::StartGame => {
                    info!("StartGame button clicked");
                    app_state.set(AppState::InGame);
                    game_state.set(GameState::GamePlaying);
                }
                Button::RestartGame => {
                    info!("RestartGame button clicked");
                    app_state.set(AppState::InGame);
                    game_state.set(GameState::GameRestarted);
                }
                Button::BackToMainMenu => {
                    info!("BackToMainMenu button clicked");
                    println!("{:?}", app_state);
                    app_state.set(AppState::MainMenu);
                    game_state.set(GameState::GameQuit);
                }
                Button::ResumeGame => {
                    info!("ResumeGame button clicked");
                    game_state.set(GameState::GamePlaying);
                }
                Button::Quit => {
                    info!("Quit button clicked");
                    exit.send_default();
                }
            },
            _ => {}
        }
    }
}
