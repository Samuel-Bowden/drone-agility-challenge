use bevy::prelude::*;
use crate::AppState;

pub fn setup_level_title(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        "Level 1",
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
                        "An easy level to start. Move from the first tower to the other.",
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
                        "Press space to start.",
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

pub fn level_title(
    mut state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::Space) {
        state.set(AppState::Game).unwrap();
    }
}