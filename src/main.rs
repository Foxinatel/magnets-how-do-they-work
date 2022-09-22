use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod init;
pub mod movement;
pub mod objects;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(init::init)
        .add_system(movement::player::movement)
        .add_system(movement::magnetic::movement);

    if cfg!(debug_assertions) {
        app.add_plugin(RapierDebugRenderPlugin {
            depth_test: false,
            style: DebugRenderStyle { ..default() },
            mode: DebugRenderMode::default(),
        });
    }

    app.run();
}
