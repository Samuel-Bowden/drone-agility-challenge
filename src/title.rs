use bevy::prelude::*;
use crate::AppState;

pub struct MenuData {
    layout: Entity,
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let layout = commands
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
                        "The aim of the game is to get from start\nto finish without crashing the drone.",
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
                        "Press return to start level 1.",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::SILVER,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                });
        }).id();

    commands.insert_resource(MenuData { layout });
}

pub fn menu(
    mut state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::Return) {
        state.set(AppState::InGame).unwrap();
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.layout).despawn_recursive();
}