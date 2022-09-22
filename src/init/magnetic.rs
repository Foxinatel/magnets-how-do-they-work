use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::objects::Magnetic;

pub fn create(commands: &mut Commands, size: (f32, f32), centered_at: (f32, f32)) {
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                extents: Vec2 {
                    x: size.0,
                    y: size.1,
                },
                origin: shapes::RectangleOrigin::Center,
            },
            DrawMode::Outlined {
                fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::GRAY),
                outline_mode: StrokeMode::new(Color::BLACK, 2.),
            },
            Transform::from_xyz(centered_at.0, centered_at.1, 1.),
        ))
        .insert(Magnetic {})
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(ExternalForce::default())
        .insert(Collider::cuboid(size.0 / 2., size.1 / 2.))
        .insert(Damping {
          linear_damping: 5.,
          angular_damping: 1.0,
        })
        .insert(Dominance::group(0));
}
