use crate::cleanup::CleanUp;
use crate::{cleanup::cleanup, AppState};
use bevy::prelude::*;

#[cfg(not(target_family = "wasm"))]
use bevy::app::AppExit;

use super::blank_camera;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_systems((setup, blank_camera).in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(click.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(cleanup.in_schedule(OnExit(AppState::MainMenu)));
    }
}

#[derive(Component)]
pub enum MenuButton {
    Play,
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Welcome",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                });
            parent
                .spawn(TextBundle {
                    style: Style {
                        margin: UiRect {
                            top: Val::Percent(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "The aim of the game is to get from the red starting podium\nto the green finishing podium - without colliding the drone\ninto anything else.",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::GRAY,
                        },
                    ),
                    ..Default::default()
                });
            parent
                .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: UiRect {
                        top: Val::Percent(4.0),
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
                            "Start",
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

            #[cfg(not(target_family = "wasm"))]
            parent
                .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: UiRect {
                        top: Val::Percent(4.0),
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
                            "Exit",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuButton::Exit);
        })
            .insert(CleanUp);
}

pub fn click(
    mut state: ResMut<NextState<AppState>>,
    input: Query<(&Interaction, &MenuButton), With<Button>>,
    #[cfg(not(target_family = "wasm"))]
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in input.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state.set(AppState::LevelMenu),
                #[cfg(not(target_family = "wasm"))]
                MenuButton::Exit => exit.send(AppExit),
            }
        }
    }
}
