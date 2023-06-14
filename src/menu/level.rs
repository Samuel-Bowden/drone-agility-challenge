use crate::{
    cleanup::{cleanup, CleanUp},
    game::levels::Levels,
    AppState,
};
use bevy::prelude::*;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::LevelMenu)))
            .add_system(click.in_set(OnUpdate(AppState::LevelMenu)))
            .add_system(cleanup.in_schedule(OnExit(AppState::LevelMenu)));
    }
}

#[derive(Component)]
enum MenuButton {
    Play,
    AllLevels,
    Return,
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
                    format!("Level {}", levels.get_current_number()),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });
            parent.spawn(TextBundle {
                style: Style {
                    margin: UiRect {
                        top: Val::Percent(2.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    levels.get_current().description,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: Color::ANTIQUE_WHITE,
                    },
                ),
                ..Default::default()
            });
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
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Play",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuButton::Play);
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
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Choose Level",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuButton::AllLevels);
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
                            "Return to main menu",
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
) {
    for (interaction, button) in input.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state.set(AppState::Game),
                MenuButton::AllLevels => state.set(AppState::AllLevelsMenu),
                MenuButton::Return => state.set(AppState::MainMenu),
            }
        }
    }
}
