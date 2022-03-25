use bevy::prelude::*;
use crate::Drone;

#[derive(Component)]
pub struct PositionText {}

pub fn position_text(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::BLACK), 
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "x: _, y: _",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(PositionText {} );
        });
}

pub fn update_position_text(
    mut q: Query<&mut Text, With<PositionText>>,
    x: Query<&Transform, With<Drone>>,
) {
    for mut text in q.iter_mut() {
        for transform in x.iter() {
            text.sections[0].value = format!(
                "x: {}, y {}",
                transform.translation.x.round() as i32,
                transform.translation.y.round() as i32,
            );
        }
    }
}