use bevy::{math::vec2, prelude::*};
use bevy_rapier2d::prelude::ExternalForce;

use crate::objects::Player;

const FORCE_MULT: f32 = 1000000000.;

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Player, &mut ExternalForce)>,
    time: Res<Time>,
) {
    macro_rules! pressed {
        ($key:ident) => {
            keyboard_input.pressed(KeyCode::$key)
        };
    }

    for (_, mut force) in &mut player {
        let mut newforce = vec2(0., 0.);

        if pressed!(A) || pressed!(Left) {
            newforce += vec2(-1., 0.) * time.delta_seconds() * FORCE_MULT
        }
        if pressed!(D) || pressed!(Right) {
            newforce += vec2(1., 0.) * time.delta_seconds() * FORCE_MULT
        }
        if pressed!(W) || pressed!(Up) {
            newforce += vec2(0., 1.) * time.delta_seconds() * FORCE_MULT
        }
        if pressed!(S) || pressed!(Down) {
            newforce += vec2(0., -1.) * time.delta_seconds() * FORCE_MULT
        }

        force.force = newforce;
    }
}
