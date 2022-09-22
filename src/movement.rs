use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalForce;

use crate::Player;

// pub mod velocity;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Player, &mut ExternalForce)>,
    time: Res<Time>,
) {
    macro_rules! pressed {
        ($key:ident) => {
            keyboard_input.pressed(KeyCode::$key)
        };
    }

    // let dist = MOVEMENT_SPEED * time.delta_seconds();

    for (_, mut force) in &mut player {
        let horiz = if pressed!(A) || pressed!(Left) {
            -1000.
        } else if pressed!(D) || pressed!(Right) {
            1000.
        } else {
            0.
        };

        let vert = if pressed!(W) || pressed!(Up) {
            1000.
        } else if pressed!(S) || pressed!(Down) {
            -1000.
        } else {
            0.
        };

        force.force = Vec2::new(horiz, vert);
    }
}
