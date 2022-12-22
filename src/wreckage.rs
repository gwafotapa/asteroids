use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::Mesh2dHandle};
use rand::Rng;
use std::f32::consts::PI;

use crate::{Collider, Health, Topology, TriangleXY, Velocity};

const WRECKAGE_HEALTH: i32 = 100;
const DEBRIS_PER_SQUARE_UNIT: f32 = 1.0 / 16.0;

#[derive(Component)]
pub struct Wreckage;

#[derive(Component)]
pub struct WreckageDebris;

pub fn update_debris(mut query: Query<(&mut Transform, &Velocity), With<WreckageDebris>>) {
    for (mut transform, velocity) in &mut query {
        transform.scale -= 1.0 / WRECKAGE_HEALTH as f32;
        transform.translation += velocity.0;
    }
}

pub fn update(mut query: Query<(&mut Health, &mut Transform, Option<&Velocity>), With<Wreckage>>) {
    for (mut health, mut transform, maybe_velocity) in &mut query {
        health.0 -= 1;
        if let Some(velocity) = maybe_velocity {
            transform.translation += velocity.0;
        }
    }
}

pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health), With<Wreckage>>) {
    for (id, health) in &query {
        if health.0 <= -WRECKAGE_HEALTH {
            commands.entity(id).despawn_recursive();
        }
    }
}

pub fn wreck_with<C: Component>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<
        (
            &Handle<ColorMaterial>,
            &Collider,
            Entity,
            // Option<&Parent>,
            // &GlobalTransform,
            &Health,
            // Option<&Velocity>,
        ),
        With<C>,
    >,
) {
    // for (color, collider, maybe_parent, transform, health, maybe_velocity) in &query {
    for (color, collider, id, health) in &query {
        if health.0 > 0 {
            continue;
        }

        let mut rng = rand::thread_rng();
        let color = materials.get(color).unwrap().color;
        // let velocity = maybe_parent
        //     .map_or(maybe_velocity, |parent| {
        //         query.get_component::<Velocity>(**parent).ok()
        //     })
        //     .map_or(Vec3::ZERO, |v| v.0);

        commands.entity(id).insert(Wreckage);
        commands.entity(id).remove::<C>();
        commands.entity(id).remove::<Mesh2dHandle>();

        // let wreck = commands
        //     .spawn(Wreckage)
        //     .insert(Health(WRECK_HEALTH))
        //     .insert(Velocity(velocity))
        //     .insert(SpatialBundle {
        //         transform: transform.compute_transform(),
        //         ..default()
        //     })
        //     .id();

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

                        // Arbitrary number of debris per triangle : area/16
                        for _ in 0..(triangle.area() * DEBRIS_PER_SQUARE_UNIT).round() as usize {
                            let p = triangle.random_point();
                            let debris =
                                Vec3::new(p.x, p.y, if rng.gen_bool(0.5) { 1.0 } else { -1.0 });
                            // let debris = transform.transform_point(debris_relative);

                            let dv =
                                Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0);

                            let debris = commands
                                // .spawn(Debris)
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
                                    material: materials.add(color.into()),
                                    ..default()
                                })
                                .id();

                            commands.entity(id).add_child(debris);
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
                        // .spawn(Debris)
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
                            material: materials.add(color.into()),
                            ..default()
                        })
                        .id();

                    commands.entity(id).add_child(debris);
                }
            }
            Topology::Point => panic!("Found point topology for explosion."),
        }
    }
}