use crate::{
    keymap::KeyMap,
    cleanup::{cleanup, CleanUp},
    AppState,
};
use bevy::prelude::*;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::FailedMenu)))
            .add_systems((click, restart_on_movement).in_set(OnUpdate(AppState::FailedMenu)))
            .add_system(cleanup.in_schedule(OnExit(AppState::FailedMenu)));
    }
}

#[derive(Component)]
enum MenuButton {
    Retry,
    Return,
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
                    "Failed",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
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
                            "Retry",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuButton::Retry);
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

fn restart_on_movement(
    input: Res<Input<KeyCode>>,
    keymap: Res<KeyMap>,
    mut state: ResMut<NextState<AppState>>,
) {
    if input.any_pressed([keymap.up, keymap.down, keymap.cw, keymap.anti_cw]) {
        state.set(AppState::Game)
    }
}

fn click(
    mut state: ResMut<NextState<AppState>>,
    input: Query<(&Interaction, &MenuButton), With<Button>>,
) {
    for (interaction, button) in input.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Retry => state.set(AppState::Game),
                MenuButton::Return => state.set(AppState::MainMenu),
            }
        }
    }
}
