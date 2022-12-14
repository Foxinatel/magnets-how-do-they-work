use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

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
                fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::RED),
                outline_mode: StrokeMode::new(Color::BLACK, 2.),
            },
            Transform::from_xyz(centered_at.0, centered_at.1, 1.),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(size.0 / 2., size.1 / 2.));
}
