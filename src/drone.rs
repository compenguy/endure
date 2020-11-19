use crate::components::{Motion, Position};
use bevy::prelude::*;

// Entity
struct Drone;

fn add_drones(commands: &mut Commands) {
    commands.spawn((Drone, Position::default(), Motion::default()));
}

fn update_drones(_time: Res<Time>, mut query: Query<(&Drone, &mut Position, &mut Motion)>) {
    for (_, _pos, _motion) in query.iter_mut() {
        // TODO: random jitter, for now
    }
}

// Entity management plugin
pub struct DronePlugin;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_drones.system())
            .add_system(update_drones.system());
    }
}
