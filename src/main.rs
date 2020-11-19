use bevy::prelude::*;

mod components;
mod drone;
mod math;
mod player;
mod resources;

use crate::components::{Motion, Position};
use crate::drone::DronePlugin;
use crate::math::Rotation;
use crate::player::PlayerPlugin;
use crate::resources::InfoTimer;

fn dump_objects(time: Res<Time>, mut timer: ResMut<InfoTimer>, query: Query<(&Position, &Motion)>) {
    timer.tick(time.delta_seconds());
    if !timer.finished() {
        return;
    }

    for (pos, _motion) in query.iter() {
        let yaw_deg = (pos.rotation.yaw() * 360.0) / std::f32::consts::PI;
        println!(
            "Object at ({},{}) ∡{}°",
            pos.translation.x, pos.translation.y, yaw_deg
        );
    }
}

fn main() {
    App::build()
        .add_resource(resources::PlayerTimer(Timer::from_seconds(0.2, true)))
        .add_resource(resources::DroneTimer(Timer::from_seconds(0.2, true)))
        .add_resource(resources::InfoTimer(Timer::from_seconds(2.0, true)))
        .add_system(dump_objects.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DronePlugin)
        .run();
}
