use bevy::prelude::*;

mod builder_func;
mod l1;
mod l2;
mod l3;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_system(spawn.on_startup())
            .insert_resource(Levels::new());
    }
}

fn spawn(mut commands: Commands, levels: Res<Levels>) {
    for level in &levels.levels {
        (level.spawn)(&mut commands, level.offset);
    }
}

pub struct Level {
    pub description: &'static str,
    offset: (f32, f32),
    spawn: fn(commands: &mut Commands, offset: (f32, f32)),
    pub unlocked: bool,
}

#[derive(Resource)]
pub struct Levels {
    current: usize,
    pub levels: [Level; 3],
}

impl Levels {
    fn new() -> Self {
        let levels = [l1::new(), l2::new(), l3::new()];

        Self { current: 0, levels }
    }

    pub fn next_level(&mut self) -> bool {
        if self.current < self.levels.len() - 1 {
            self.current += 1;
            self.levels[self.current].unlocked = true;
            true
        } else {
            false
        }
    }

    pub fn set_level(&mut self, level: usize) {
        self.current = level;
    }

    pub fn back_to_start(&mut self) {
        self.current = 0;
    }

    pub fn get_current_number(&self) -> usize {
        self.current + 1
    }

    pub fn get_drone_spawn_position(&self) -> Vec3 {
        let offset = self.levels[self.current].offset;
        Vec3::new(offset.0, offset.1 + 20., 0.)
    }

    pub fn get_current(&self) -> &Level {
        &self.levels[self.current]
    }
}
