use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalForce;

use crate::objects::{Player, Magnetic};

pub fn movement(
    mouse_input: Res<Input<MouseButton>>,
    player: Query<(&Player, &Transform)>,
    mut objects: Query<(&Magnetic, &Transform, &mut ExternalForce)>,
    time: Res<Time>,
) {

  if mouse_input.pressed(MouseButton::Left) {
    let (_, target) = player.single();
    let target = target.translation.truncate();
  
    for (_, pos, mut force) in &mut objects {
      let pos = pos.translation.truncate();
      let diff = target - pos;
  
      force.force = diff.normalize() * (1./diff.length_squared()) * time.delta_seconds() * 2e13;
    }
  } else {
    for (_, _, mut force) in &mut objects {
      force.force = Vec2::default();
    }
  }
}
