use crate::{
    cleanup::{cleanup, CleanUp},
    AppState, CurrentLevel,
};
use bevy::prelude::*;

use super::blank_camera;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_systems((setup, blank_camera).in_schedule(OnEnter(AppState::LevelMenu)))
            .add_system(click.in_set(OnUpdate(AppState::LevelMenu)))
            .add_system(cleanup.in_schedule(OnExit(AppState::LevelMenu)));
    }
}

#[derive(Component)]
pub enum MenuButton {
    Play,
    Return,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
) {
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
            parent.spawn(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::from_section(
                    format!("Level {}", current_level.0),
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
                    crate::level::get_description(current_level.0),
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
                        size: Size::new(Val::Px(350.0), Val::Px(65.0)),
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

pub fn click(
    mut state: ResMut<NextState<AppState>>,
    input: Query<(&Interaction, &MenuButton), With<Button>>,
) {
    for (interaction, button) in input.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state.set(AppState::Game),
                MenuButton::Return => state.set(AppState::MainMenu),
            }
        }
    }
}
