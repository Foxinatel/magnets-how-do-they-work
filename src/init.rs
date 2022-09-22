use bevy::prelude::*;

pub mod magnetic;
pub mod wall;
pub mod player;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Generate 2d camera view
    commands.spawn_bundle(Camera2dBundle::default());

    //Generate playable character
    player::create(&mut commands, 40., (0., 0.));

    wall::create(&mut commands, (100., 100.), (100., 100.));
    wall::create(&mut commands, (100., 100.), (100., -100.));
    wall::create(&mut commands, (100., 100.), (-100., 100.));
    wall::create(&mut commands, (100., 100.), (-100., -100.));

    magnetic::create(&mut commands, (100., 100.), (0., 200.));
}