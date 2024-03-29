use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use rand::Rng;
use std::f32::consts::PI;

use crate::{
    collision::detection::{triangle::TriangleXY, Collider, Topology},
    component::{AngularVelocity, Health, Part, Velocity},
    transform,
};

const HEALTH: u32 = 100;
const DEBRIS_PER_SQUARE_UNIT: f32 = 1.0 / 32.0;

#[derive(Component)]
pub struct Wreckage;

#[derive(Component)]
pub struct WreckageDebris;

pub fn update_debris(
    mut query: Query<(&mut Transform, &Velocity), With<WreckageDebris>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.scale -= 1.0 / HEALTH as f32;
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

pub fn update(
    mut query: Query<(&mut Health, &mut Transform, &Velocity), With<Wreckage>>,
    time: Res<Time>,
) {
    for (mut health, mut transform, velocity) in &mut query {
        health.0 -= 1;
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

pub fn wreck_with<C: Component>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<
        (&AngularVelocity, &Children, Entity, &Transform, &Velocity),
        (With<C>, Without<Part>),
    >,
    query_part: Query<
        (
            &Handle<ColorMaterial>,
            &Collider,
            Entity,
            &Health,
            &Parent,
            &Transform,
        ),
        (With<C>, With<Part>),
    >,
) {
    for (color, collider, part, health, parent, transform) in &query_part {
        if health.0 > 0 {
            continue;
        }

        let mut rng = rand::thread_rng();
        let (p_angular_velocity, children, parent, p_transform, p_velocity) =
            query.get(**parent).unwrap();

        if children
            .iter()
            .filter(|child| query_part.get(**child).is_ok())
            .count()
            > 1
        {
            commands.entity(parent).remove_children(&[part]);
            commands.entity(part).despawn();
        } else {
            commands.entity(parent).despawn_recursive();
        }

        let wreckage = commands
            .spawn(Wreckage)
            .insert(Health(HEALTH))
            .insert(Velocity(
                p_velocity.0
                    + p_angular_velocity.0
                        * Vec3::Z.cross(p_transform.rotation.mul_vec3(transform.translation)),
            ))
            .insert(SpatialBundle {
                transform: transform::global_of(*transform, *p_transform),
                ..Default::default()
            })
            .id();

        match &collider.topology {
            Topology::Triangles { mesh_handle } => {
                if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
                    .get(&mesh_handle.0)
                    .unwrap()
                    .attribute(Mesh::ATTRIBUTE_POSITION)
                {
                    for triplet in vertices.clone().chunks_exact(3) {
                        let triangle: TriangleXY =
                            <[_; 3]>::try_from(triplet).expect("3 items").into();

                        for _ in 0..(triangle.area() * DEBRIS_PER_SQUARE_UNIT).round() as usize {
                            let p = triangle.random_point();
                            let debris =
                                Vec3::new(p.x, p.y, if rng.gen_bool(0.5) { 1.0 } else { -1.0 });
                            let dv =
                                Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0);

                            let debris = commands
                                .spawn(WreckageDebris)
                                .insert(Velocity(dv))
                                .insert(ColorMesh2dBundle {
                                    mesh: meshes
                                        .add(Mesh::from(shape::Circle {
                                            radius: rng.gen_range(1.0..10.0),
                                            vertices: 4 * rng.gen_range(1..5),
                                        }))
                                        .into(),
                                    transform: Transform::from_translation(debris),
                                    material: color.clone(),
                                    ..default()
                                })
                                .id();

                            commands.entity(wreckage).add_child(debris);
                        }
                    }
                }
            }
            Topology::Disk { radius } => {
                let area = PI * radius * radius;
                for _ in 0..(area * DEBRIS_PER_SQUARE_UNIT).round() as usize {
                    let rho = rng.gen_range(0.0..*radius);
                    let theta = rng.gen_range(0.0..2.0 * PI);
                    let (sin, cos) = theta.sin_cos();
                    let (x, y) = (rho * cos, rho * sin);
                    let z = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
                    let debris = Vec3::new(x, y, z);

                    let dv = Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0);

                    let debris = commands
                        .spawn(WreckageDebris)
                        .insert(Velocity(dv))
                        .insert(ColorMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: rng.gen_range(1.0..radius / 10.0),
                                    vertices: 4 * rng.gen_range(1..5),
                                }))
                                .into(),
                            transform: Transform::from_translation(debris),
                            material: color.clone(),
                            ..default()
                        })
                        .id();

                    commands.entity(wreckage).add_child(debris);
                }
            }
            Topology::Point => panic!("Found point topology for explosion."),
        }
    }
}
