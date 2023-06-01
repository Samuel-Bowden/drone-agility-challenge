use super::LevelTime;
use crate::{cleanup::CleanUp, AppState};
use bevy::{prelude::*, time::Stopwatch};

#[derive(Component)]
struct LevelDuration(Stopwatch);

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_system(spawn.in_schedule(OnEnter(AppState::Game)))
            .add_system(update.in_set(OnUpdate(AppState::Game)))
            .add_system(level_time.in_schedule(OnExit(AppState::Game)));
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                padding: UiRect {
                    top: Val::Px(20.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|node| {
            node.spawn(
                TextBundle::from_section(
                    "0s",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            )
            .insert(LevelDuration(Stopwatch::new()));
        })
        .insert(CleanUp);
}

fn update(mut clock: Query<(&mut Text, &mut LevelDuration)>, time: Res<Time>) {
    let (mut text, mut level_duration) = clock.single_mut();

    level_duration.0.tick(time.delta());

    text.sections[0].value = level_duration.0.elapsed().as_secs().to_string() + "s";
}

fn level_time(mut clock: Query<&mut LevelDuration>, mut level_time: ResMut<LevelTime>) {
    level_time.0 = clock.single_mut().0.elapsed().as_secs();
}
