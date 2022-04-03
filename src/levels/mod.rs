use bevy::prelude::*;
use crate::CurrentLevel;

mod l1;
mod l2;
mod l3;

pub fn spawn_level(
	commands: Commands,
	current_level: Res<CurrentLevel>,
) {
	match current_level.0 {
		1 => l1::spawn(commands),
		2 => l2::spawn(commands),
		3 => l3::spawn(commands),
		_ => (),
	}
}

pub fn get_description(current_level: u32) -> &'static str {
	match current_level {
		1 => l1::description(),
		2 => l2::description(),
		3 => l3::description(),
		_ => "Invalid Level",
	}
}