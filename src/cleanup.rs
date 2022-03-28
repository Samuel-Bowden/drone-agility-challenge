use bevy::prelude::*;

pub fn cleanup(
    mut entities: Query<Entity>,
    mut commands: Commands,
) {
    for entity in entities.iter_mut() {
        commands.entity(entity).despawn();
    }
}