use bevy::prelude::*;
use crate::AppState;

pub fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Welcome",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                });
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect {
                            top: Val::Percent(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "The aim of the game is to get from the red starting podium\nto the green finishing podium - without colliding the drone\ninto anything else.",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::GRAY,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                });
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect {
                            top: Val::Percent(5.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Press return to begin.",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::SILVER,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                });
        });
}

pub fn title(
    mut state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::Return) {
        state.set(AppState::LevelTitle).unwrap();
    }
}