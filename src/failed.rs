use bevy::prelude::*;
use crate::AppState;

pub fn setup_failed(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        "Failed",
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
                            top: Val::Percent(5.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Press return to retry",
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

pub fn failed(
    mut state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::Return) {
        state.set(AppState::Game).unwrap();
    }
}

pub fn cleanup_failed(
    mut entities: Query<Entity>,
    mut commands: Commands,
) {
    for entity in entities.iter_mut() {
        commands.entity(entity).despawn();
    }
}