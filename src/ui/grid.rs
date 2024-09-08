use crate::piece::block::BLOCK_LENGTH;

use super::*;

pub fn setup(windows: Query<&Window>, mut gizmos: Gizmos) {
    if windows.is_empty() {
        return;
    }
    let window = windows.single();
    let x = (window.width() as f32 / BLOCK_LENGTH).ceil() as u32;
    let y = (window.height() as f32 / BLOCK_LENGTH).ceil() as u32;

    gizmos
        .grid_2d(
            Vec2::ZERO,
            0.0,
            UVec2::new(x * 2, y * 2),
            Vec2::new(BLOCK_LENGTH, BLOCK_LENGTH),
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}

pub fn set_false(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.enabled = false;
}

pub fn manage_config(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    if keyboard.just_pressed(KeyCode::KeyG) {
        config.enabled = !config.enabled;
    }
}
