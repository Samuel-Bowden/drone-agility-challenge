use bevy::prelude::*;

pub enum PodiumType {
    Start,
    Finish,
}

#[derive(Component)]
pub struct Podium(pub PodiumType);
