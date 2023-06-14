use crate::{
    cleanup::{cleanup, CleanUp},
    game::levels::Levels,
    AppState,
};
use bevy::prelude::*;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::AllLevelsMenu)))
            .add_system(click.in_set(OnUpdate(AppState::AllLevelsMenu)))
            .add_system(cleanup.in_schedule(OnExit(AppState::AllLevelsMenu)));
    }
}

#[derive(Component)]
enum MenuButton {
    Level(usize),
    Return,
    None,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, levels: Res<Levels>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Levels",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });

            for (i, level) in levels.levels.iter().enumerate() {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                            margin: UiRect {
                                top: Val::Percent(2.0),
                                ..Default::default()
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: if level.unlocked {
                            Color::rgb(0.15, 0.15, 0.15)
                        } else {
                            Color::BLACK
                        }
                        .into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                if level.unlocked {
                                    format!("Level {}", i + 1)
                                } else {
                                    "Locked".to_string()
                                },
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ),
                            ..Default::default()
                        });
                    })
                    .insert(if level.unlocked {
                        MenuButton::Level(i)
                    } else {
                        MenuButton::None
                    });
            }

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                        margin: UiRect {
                            top: Val::Percent(2.0),
                            ..Default::default()
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.6, 0.07, 0.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Return",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuButton::Return);
        })
        .insert(CleanUp);
}

fn click(
    mut state: ResMut<NextState<AppState>>,
    input: Query<(&Interaction, &MenuButton), With<Button>>,
    mut levels: ResMut<Levels>,
) {
    for (interaction, button) in input.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Level(i) => {
                    levels.set_level(*i);
                    state.set(AppState::LevelMenu);
                }
                MenuButton::Return => state.set(AppState::LevelMenu),
                MenuButton::None => (),
            }
        }
    }
}
