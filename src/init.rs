use bevy::prelude::*;

pub mod magnetic;
pub mod player;
pub mod wall;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Generate 2d camera view
    commands.spawn_bundle(Camera2dBundle::default());

    //Generate playable character
    player::create(&mut commands, 40., (0., 0.));

    wall::create(&mut commands, (100., 100.), (200., 200.));
    wall::create(&mut commands, (100., 100.), (200., -200.));
    wall::create(&mut commands, (100., 100.), (-200., 200.));
    wall::create(&mut commands, (100., 100.), (-200., -200.));

    magnetic::create(&mut commands, (100., 100.), (0., 200.));
    magnetic::create(&mut commands, (100., 100.), (0., -200.));
    magnetic::create(&mut commands, (100., 100.), (200., 0.));
    magnetic::create(&mut commands, (100., 100.), (-200., 0.));
}
