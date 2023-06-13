use crate::{
    cleanup::{cleanup, CleanUp},
    AppState,
};
use bevy::prelude::*;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::EndMenu)))
            .add_system(click.in_set(OnUpdate(AppState::EndMenu)))
            .add_system(cleanup.in_schedule(OnExit(AppState::EndMenu)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
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
                style: Style {
                    ..Default::default()
                },
                text: Text::from_section(
                    "The End",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });
            parent.spawn(
                TextBundle {
                    style: Style {
                        margin: UiRect {
                            top: Val::Percent(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Well done!\nYou have completed all the levels.",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::ANTIQUE_WHITE,
                        },
                    ),
                    ..Default::default()
                }
                .with_text_alignment(TextAlignment::Center),
            );
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
                            "Return to Main Menu",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..Default::default()
                    });
                });
        })
        .insert(CleanUp);
}

fn click(mut state: ResMut<NextState<AppState>>, input: Query<&Interaction, With<Button>>) {
    for interaction in input.iter() {
        if *interaction == Interaction::Clicked {
            state.set(AppState::MainMenu);
        }
    }
}
