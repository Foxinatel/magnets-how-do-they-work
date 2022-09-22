use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::objects::Player;

pub fn create(commands: &mut Commands, radius: f32, centered_at: (f32, f32)) {
    let (x, y) = centered_at;
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                radius,
                center: Vec2 { x, y },
            },
            DrawMode::Outlined {
                fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 2.),
            },
            Transform::default(),
        ))
        .insert(Player {})
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(radius))
        .insert(ExternalForce::default())
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 5.,
            angular_damping: 1.0,
        });
}
