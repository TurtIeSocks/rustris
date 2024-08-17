use bevy::prelude::*;

// game board
pub const COL_COUNT: f32 = 10.;
pub const ROW_COUNT: f32 = 20.;
pub const BORDER_THICKNESS: f32 = 10.0;
pub const BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

pub const BLOCK_LENGTH: f32 = 30.0;

fn column_border(pos: f32) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: pos * BLOCK_LENGTH
                    + if pos > 0. {
                        BORDER_THICKNESS
                    } else {
                        -BORDER_THICKNESS
                    } / 2.0,
                ..default()
            },
            scale: Vec3 {
                x: BORDER_THICKNESS,
                y: ROW_COUNT as f32 * BLOCK_LENGTH + 2.0 * BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    }
}

fn row_border(pos: f32) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                y: pos * BLOCK_LENGTH
                    + if pos > 0. {
                        BORDER_THICKNESS
                    } else {
                        -BORDER_THICKNESS
                    } / 2.0,
                ..default()
            },
            scale: Vec3 {
                x: COL_COUNT as f32 * BLOCK_LENGTH,
                y: BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    }
}

pub fn setup(mut commands: Commands) {
    let half_col_count = COL_COUNT / 2.0;
    let half_row_count = ROW_COUNT / 2.0;

    // left border
    commands.spawn(column_border(-half_col_count));
    // right border
    commands.spawn(column_border(half_col_count));
    // bottom border
    commands.spawn(row_border(-half_row_count));
    // top border
    commands.spawn(row_border(half_row_count));
}
