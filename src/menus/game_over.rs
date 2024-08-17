use super::*;
use bevy::color::palettes;

#[derive(Component)]
pub struct GameOver;

pub fn setup(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            GameOver,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: palettes::css::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(
                        TextBundle::from_section(
                            "Game Over",
                            TextStyle {
                                font_size: 25.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    // 返回主菜单按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(90.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::default()
                                    .with_color(Color::srgb(0.15, 0.15, 0.15).into()),
                                ..default()
                            },
                            button::Button::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });

                    // 重新开始按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(90.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::default()
                                    .with_color(Color::srgb(0.15, 0.15, 0.15).into()),
                                ..default()
                            },
                            button::Button::RestartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Restart",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}
