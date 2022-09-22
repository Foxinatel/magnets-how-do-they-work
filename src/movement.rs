use bevy::prelude::*;

use crate::Player;

pub mod velocity;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
) {
    macro_rules! pressed {
        ($key:ident) => {
            keyboard_input.pressed(KeyCode::$key)
        };
    }

    // let dist = MOVEMENT_SPEED * time.delta_seconds();

    let (mut player, mut transform) = player.single_mut();

    let delta_accel = velocity::ACCEL * time.delta_seconds();

    if pressed!(A) || pressed!(Left) {
        player.velocity.x -= delta_accel
        // debug!("{:?}", transform)
    } else {
        player.velocity.x /= 1.02
    }

    if pressed!(D) || pressed!(Right) {
        player.velocity.x += delta_accel
    } else {
        player.velocity.x /= 1.02
    }

    if pressed!(W) || pressed!(Up) {
        player.velocity.y += delta_accel
    } else {
        player.velocity.y /= 1.02
    }

    if pressed!(S) || pressed!(Down) {
        player.velocity.y -= delta_accel
    } else {
        player.velocity.y /= 1.02
    }

    transform.translation.x += player.velocity.x;
    transform.translation.y += player.velocity.y;
}
