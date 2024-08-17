use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct GameAudio {
    pub drop: Handle<AudioSource>,
    pub gameover: Handle<AudioSource>,
    pub line_clear: Handle<AudioSource>,
}

impl GameAudio {
    pub fn play(&self, commands: &mut Commands, audio_type: &str) {
        match audio_type {
            "drop" => {
                commands.spawn(AudioBundle {
                    source: self.drop.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            "gameover" => {
                commands.spawn(AudioBundle {
                    source: self.gameover.clone(),
                    ..default()
                });
            }
            "line_clear" => {
                commands.spawn(AudioBundle {
                    source: self.line_clear.clone(),
                    ..default()
                });
            }
            _ => panic!("Invalid audio type: {}", audio_type),
        }
    }
}

pub fn setup(mut command: Commands, asset_server: Res<AssetServer>) {
    let game_audios = GameAudio {
        drop: asset_server.load("sounds/Drop.wav"),
        gameover: asset_server.load("sounds/Gameover.wav"),
        line_clear: asset_server.load("sounds/Lineclear.wav"),
    };
    command.insert_resource(game_audios);
}
