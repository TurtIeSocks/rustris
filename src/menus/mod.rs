use bevy::prelude::*;

pub mod button;
pub mod game_over;
pub mod main;
pub mod pause;

// pub struct MenuPlugin;

// impl Plugin for MenuPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, add_people);
//     }
// }

// trait Menu {
//     fn setup(commands: &mut Commands);
// }

pub fn close_menu<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
