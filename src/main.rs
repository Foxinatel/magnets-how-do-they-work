use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

type Colour = Color;

pub mod movement;

#[derive(Component, Default)]
pub struct Player {}

// pub struct Collider {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin{ depth_test: false, style: DebugRenderStyle { ..default() }, mode: DebugRenderMode::default() })
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
                fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 2.),
            },
            Transform::default(),
        ))
        .insert(Player::default())
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(40.0))
        .insert(ColliderMassProperties::Density(0.0))
        .insert(ExternalForce::default())
        .insert(GravityScale(0.0))
        .insert(Damping{ linear_damping: 0.99, angular_damping: 1.0 })
        .insert(AdditionalMassProperties::MassProperties(MassProperties {
            mass: 1.0,
            ..default()
        }));

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                extents: Vec2 { x: 100., y: 100. },
                origin: shapes::RectangleOrigin::Center,
            },
            DrawMode::Outlined {
                fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Colour::RED),
                outline_mode: StrokeMode::new(Colour::BLACK, 2.),
            },
            Transform::from_xyz(100., 100., 1.),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(50., 50.));
}
