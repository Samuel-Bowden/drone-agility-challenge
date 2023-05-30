use bevy::prelude::*;

#[derive(Component)]
pub struct CleanUp;

pub fn cleanup(mut entities: Query<Entity, With<CleanUp>>, mut commands: Commands) {
    for entity in entities.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
