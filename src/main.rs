use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod init;
pub mod movement;
pub mod objects;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            depth_test: false,
            style: DebugRenderStyle { ..default() },
            mode: DebugRenderMode::default(),
        })
        .add_startup_system(init::init)
        .add_system(movement::player_movement)
        .run();
}
