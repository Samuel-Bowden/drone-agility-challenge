use bevy::prelude::*;
use crate::CurrentLevel;

mod one;
mod two;

pub fn spawn_level(
	commands: Commands,
	current_level: Res<CurrentLevel>,
) {
	match current_level.0 {
		1 => one::spawn(commands),
		2 => two::spawn(commands),
		_ => (),
	}
}

pub fn get_description(current_level: u32) -> &'static str {
	match current_level {
		1 => one::description(),
		2 => two::description(),
		_ => "Invalid Level",
	}
}