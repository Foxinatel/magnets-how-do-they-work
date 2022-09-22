use crate::movement::velocity::Velocity;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::{RapierPhysicsPlugin, NoUserData, RigidBody};
// use bevy_rapier2d::prelude::*;

type Colour = Color;

pub mod movement;

#[derive(Component, Default)]
pub struct Player {
    velocity: Velocity,
}

pub struct Collider {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup)
        .add_system(movement::player_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                radius: 40.,
                center: Vec2::default(),
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 2.),
            },
            Transform::default(),
        ))
        .insert(Player::default())
        .insert(RigidBody::Dynamic);

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                extents: Vec2 { x: 100., y: 100. },
                origin: shapes::RectangleOrigin::BottomLeft,
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Colour::RED),
                outline_mode: StrokeMode::color(Colour::BLACK),
            },
            Transform::from_xyz(100., 100., 1.),
        ))
        .insert(RigidBody::Fixed);
}
