use crate::components::{Motion, Position};
use crate::resources::PlayerTimer;
use bevy::prelude::*;

// Entity
struct Player;

fn add_player(commands: &mut Commands) {
    commands.spawn((Player, Position::default(), Motion::default()));
}

fn update_player(
    time: Res<Time>,
    mut timer: ResMut<PlayerTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Position, &mut Motion)>,
) {
    timer.tick(time.delta_seconds());
    if !timer.finished() {
        return;
    }

    let do_left = keyboard_input.pressed(KeyCode::Left);
    let do_right = keyboard_input.pressed(KeyCode::Right);
    let do_forward = keyboard_input.pressed(KeyCode::Up);
    let do_back = keyboard_input.pressed(KeyCode::Down);

    for (_, mut pos, mut motion) in query.iter_mut() {
        // Calculate yaw change rate
        // If no active spin change, apply a decay rate that will ensure its
        // decay to zero
        let scaled_rot = (std::f32::consts::PI / 4.0) * time.delta_seconds();
        let yawdelta: f32 = match (do_left, do_right) {
            (true, false) => 0.0 + scaled_rot,
            (false, true) => std::f32::consts::PI - scaled_rot,
            // TODO: implement decay
            (_, _) => 0.0,
        };
        let rotation = Transform::from_rotation(Quat::from_rotation_ypr(yawdelta, 0.0, 0.0));

        // Calculate translation change
        let transdelta = match (do_forward, do_back) {
            (true, false) => 1.0 * time.delta_seconds(),
            (false, true) => -1.0 * time.delta_seconds(),
            // TODO: implement decay
            (_, _) => 0.0,
        };
        let translation = Transform::from_translation(Vec3::new(transdelta, 0.0, 0.0));
        motion.0 = motion.0 * rotation * translation;

        // And update position
        pos.0 = pos.0 * motion.0;
    }
}

// Entity management plugin
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_player.system())
            .add_system(update_player.system());
    }
}
