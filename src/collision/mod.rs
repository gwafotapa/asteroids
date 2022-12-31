use bevy::prelude::*;
// use iyes_loopless::prelude::*;

use crate::{
    asteroid::Asteroid,
    boss::{BossCore, BossEdge},
    fire::{Enemy, Fire},
    spaceship::Spaceship,
    AngularVelocity, Health, Mass, MomentOfInertia, Velocity,
};

use cache::{Cache, Collision};
pub use detection::{Aabb, Collider, Topology};
use impact::Impact;

pub mod cache;
pub mod detection;
pub mod impact;
pub mod response;

pub fn fire_and_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_fire: Query<(
        &Collider,
        &Handle<ColorMaterial>,
        &Fire,
        &Transform,
        &mut Health,
    )>,
    mut query_asteroid: Query<
        (&Collider, &GlobalTransform, &mut Health),
        (With<Asteroid>, Without<Fire>),
    >,
) {
    for (f_collider, f_color, fire, f_transform, mut f_health) in query_fire.iter_mut() {
        for (a_collider, a_transform, mut a_health) in query_asteroid.iter_mut() {
            // if detection::collision_point_circle(
            //     f_transform,
            //     &a_transform.compute_transform(),
            //     asteroid.radius,
            if detection::collision(
                *f_transform,
                a_transform.compute_transform(),
                f_collider,
                a_collider,
                None,
            )
            .is_some()
            {
                a_health.0 -= 1;
                f_health.0 = 0;
                let color = materials.get(f_color).unwrap().color;

                let _impact = commands
                    .spawn(Impact)
                    .insert(Health(10))
                    .insert(ColorMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: fire.impact_radius,
                                vertices: fire.impact_vertices,
                            }))
                            .into(),
                        transform: Transform::from_translation(f_transform.translation),
                        material: materials.add(color.into()),
                        ..default()
                    })
                    .id();

                break;
            }
        }
    }
}

pub fn fire_and_boss(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_fire: Query<
        (
            &Collider,
            &Handle<ColorMaterial>,
            // Entity,
            &Fire,
            &Transform,
            &mut Health,
            // &mut Velocity,
        ),
        Without<Enemy>,
    >,
    // mut query_boss_core: Query<
    //     (
    //         Option<&Children>,
    //         &Handle<ColorMaterial>,
    //         Entity,
    //         &GlobalTransform,
    //         &mut Health,
    //         &Surface,
    //     ),
    //     (With<BossCore>, Without<Fire>),
    // >,
    mut query_boss_part: Query<
        (
            Option<&BossCore>,
            &Collider,
            &Handle<ColorMaterial>,
            Entity,
            &GlobalTransform,
            &mut Health,
        ),
        (Or<(With<BossEdge>, With<BossCore>)>, Without<Fire>),
    >,
) {
    for (bp_core, bp_collider, bp_color, bp_entity, bp_transform, mut bp_health) in
        query_boss_part.iter_mut()
    {
        let bp_transform = bp_transform.compute_transform();
        for (f_collider, f_color, fire, f_transform, mut f_health) in query_fire.iter_mut() {
            // if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
            //     .get(&bp_mesh.0)
            //     .unwrap()
            //     .attribute(Mesh::ATTRIBUTE_POSITION)
            // {
            // if detection::collision_point_triangles(
            //     f_transform,
            //     &bp_transform,
            //     vertices,
            //     bp_collider.aabb,
            if detection::collision(
                *f_transform,
                bp_transform,
                f_collider,
                bp_collider,
                Some(&meshes),
            )
            .is_some()
            {
                f_health.0 = 0;
                let f_color = materials.get(f_color).unwrap().color;

                if bp_core.is_none() || bp_core.unwrap().edges == 0 {
                    bp_health.0 -= 1;
                    let bp_color = materials.get_mut(bp_color).unwrap();
                    let [mut r, mut g, mut b, _] = bp_color.color.as_rgba_f32();
                    let [r2, g2, b2, _] = f_color.as_rgba_f32();
                    r += (r2 - r) / (1. + bp_health.0 as f32);
                    g += (g2 - g) / (1. + bp_health.0 as f32);
                    b += (b2 - b) / (1. + bp_health.0 as f32);
                    bp_color.color = Color::rgb(r, g, b);
                }

                let impact = commands
                    .spawn(Impact)
                    .insert(Health(10))
                    .insert(ColorMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: fire.impact_radius,
                                vertices: fire.impact_vertices,
                            }))
                            .into(),
                        transform: Transform::from_translation(
                            bp_transform
                                .rotation
                                .inverse()
                                .mul_vec3(f_transform.translation - bp_transform.translation),
                        ),
                        // transform: *f_transform,
                        material: materials.add(f_color.into()),
                        ..default()
                    })
                    .id();

                commands.entity(bp_entity).add_child(impact);
                // } else {
                //     f_velocity.0 = -f_velocity.0;
                //     commands.entity(f_entity).insert(Enemy);
                // }
            }
            // } else {
            //     panic!("Cannot find the boss's mesh to compute collision");
            // }
        }
    }
}

pub fn fire_and_spaceship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_fire: Query<
        (
            &Collider,
            &Handle<ColorMaterial>,
            &Fire,
            &mut Health,
            &Transform,
        ),
        With<Enemy>,
    >,
    mut query_spaceship: Query<
        (&Collider, Entity, &mut Health, &Transform),
        (With<Spaceship>, Without<Fire>),
    >,
) {
    if let Ok((s_collider, s_entity, mut s_health, s_transform)) = query_spaceship.get_single_mut()
    {
        for (f_collider, f_color, fire, mut f_health, f_transform) in query_fire.iter_mut() {
            // if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
            //     .get(&s_mesh.0)
            //     .unwrap()
            //     .attribute(Mesh::ATTRIBUTE_POSITION)
            // {
            // if detection::collision_point_triangles(
            //     f_transform,
            //     s_transform,
            //     vertices,
            //     s_collider.aabb,
            if detection::collision(
                *f_transform,
                *s_transform,
                f_collider,
                s_collider,
                Some(&meshes),
            )
            .is_some()
            {
                f_health.0 = 0;
                s_health.0 -= 1;
                let f_color = materials.get(f_color).unwrap().color;

                let impact = commands
                    .spawn(Impact)
                    .insert(Health(10))
                    .insert(ColorMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle {
                                radius: fire.impact_radius,
                                vertices: fire.impact_vertices,
                            }))
                            .into(),
                        transform: Transform::from_translation(
                            s_transform
                                .rotation
                                .inverse()
                                .mul_vec3(f_transform.translation - s_transform.translation),
                        ),
                        // transform: Transform::from_translation(f_transform.translation()),
                        material: materials.add(f_color.into()),
                        ..default()
                    })
                    .id();

                commands.entity(s_entity).add_child(impact);
            }
            // }
        }
    }
}

pub fn spaceship_and_boss(
    // mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query_spaceship: Query<
        (
            &mut AngularVelocity,
            &Collider,
            Entity,
            &mut Health,
            &Mass,
            &MomentOfInertia,
            &Transform,
            // &mut Transform,
            &mut Velocity,
        ),
        With<Spaceship>,
    >,
    mut query_boss_edge: Query<
        (&Collider, Entity, &mut Transform),
        (With<BossEdge>, Without<Spaceship>),
    >,
    mut query_boss_core: Query<
        (
            &mut AngularVelocity,
            &Collider,
            Entity,
            &Mass,
            &MomentOfInertia,
            &Transform,
            // &mut Transform,
            &mut Velocity,
        ),
        (With<BossCore>, Without<BossEdge>, Without<Spaceship>),
    >,
) {
    if let Ok((
        mut s_angular_velocity,
        s_collider,
        spaceship,
        mut _s_health,
        s_mass,
        s_moment_of_inertia,
        mut s_transform,
        mut s_velocity,
    )) = query_spaceship.get_single_mut()
    {
        if let Ok((
            mut bc_angular_velocity,
            bc_collider,
            boss_core,
            bc_mass,
            bc_moment_of_inertia,
            mut bc_transform,
            mut bc_velocity,
        )) = query_boss_core.get_single_mut()
        {
            if let Some(contact) = detection::collision(
                *s_transform,
                *bc_transform,
                &s_collider,
                &bc_collider,
                Some(&meshes),
            ) {
                if !cache.contains(Collision(spaceship, boss_core)) {
                    response::compute(
                        &mut s_transform,
                        &mut bc_transform,
                        *s_mass,
                        *bc_mass,
                        *s_moment_of_inertia,
                        *bc_moment_of_inertia,
                        &mut s_velocity,
                        &mut bc_velocity,
                        &mut s_angular_velocity,
                        &mut bc_angular_velocity,
                        contact,
                    );
                }
                cache.add(Collision(spaceship, boss_core));
                // s_health.0 = 0;
                return;
            }
            for (be_collider, boss_edge, be_transform) in query_boss_edge.iter_mut() {
                let be_global_transform = Transform::from_translation(
                    bc_transform.transform_point(be_transform.translation),
                )
                .with_rotation(bc_transform.rotation * be_transform.rotation);
                if let Some(contact) = detection::collision(
                    *s_transform,
                    be_global_transform,
                    &s_collider,
                    &be_collider,
                    Some(&meshes),
                ) {
                    // TODO
                    // contact.normal = (s_transform.translation - bc_transform.translation)
                    //     .truncate()
                    //     .normalize();

                    // commands.spawn(ColorMesh2dBundle {
                    //     mesh: meshes
                    //         .add(Mesh::from(shape::Circle {
                    //             radius: 3.0,
                    //             vertices: 32,
                    //         }))
                    //         .into(),
                    //     transform: Transform::from_xyz(contact.point.x, contact.point.y, 500.0),
                    //     ..Default::default()
                    // });
                    // commands.insert_resource(NextState(GameState::Paused));

                    if !cache.contains(Collision(spaceship, boss_edge)) {
                        println!("spaceship -- w1: {}", s_angular_velocity.0);
                        println!("boss      -- w2: {}", bc_angular_velocity.0);
                        response::compute(
                            &mut s_transform,
                            &mut bc_transform,
                            *s_mass,
                            *bc_mass,
                            *s_moment_of_inertia,
                            *bc_moment_of_inertia,
                            &mut s_velocity,
                            &mut bc_velocity,
                            &mut s_angular_velocity,
                            &mut bc_angular_velocity,
                            contact,
                        );
                        println!("spaceship -- w'1: {}", s_angular_velocity.0);
                        println!("boss      -- w'2: {}", bc_angular_velocity.0);
                        println!("");
                    }
                    cache.add(Collision(spaceship, boss_edge));
                    // s_health.0 = 0;
                    return;
                }
            }
        }
    }
}

pub fn boss_and_asteroid(
    meshes: Res<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query_asteroid: Query<
        (
            &mut AngularVelocity,
            &Collider,
            Entity,
            &mut Health,
            &Mass,
            &MomentOfInertia,
            &Transform,
            // &mut Transform,
            &mut Velocity,
        ),
        With<Asteroid>,
    >,
    mut query_boss_edge: Query<
        (&Collider, Entity, &GlobalTransform),
        (With<BossEdge>, Without<Asteroid>),
    >,
    mut query_boss_core: Query<
        (
            &mut AngularVelocity,
            &Collider,
            Entity,
            &Mass,
            &MomentOfInertia,
            &Transform,
            // &mut Transform,
            &mut Velocity,
        ),
        (With<BossCore>, Without<BossEdge>, Without<Asteroid>),
    >,
) {
    if let Ok((
        mut bc_angular_velocity,
        bc_collider,
        boss_core,
        bc_mass,
        bc_moment_of_inertia,
        mut bc_transform,
        mut bc_velocity,
    )) = query_boss_core.get_single_mut()
    {
        for (
            mut a_angular_velocity,
            a_collider,
            asteroid,
            mut _a_health,
            a_mass,
            a_moment_of_inertia,
            mut a_transform,
            mut a_velocity,
        ) in query_asteroid.iter_mut()
        {
            for (be_collider, boss_edge, be_transform) in query_boss_edge.iter_mut() {
                if let Some(contact) = detection::collision(
                    *a_transform,
                    be_transform.compute_transform(),
                    &a_collider,
                    &be_collider,
                    Some(&meshes),
                ) {
                    // println!("Collision boss / asteroid");
                    if !cache.contains(Collision(asteroid, boss_edge)) {
                        response::compute(
                            &mut a_transform,
                            &mut be_transform.compute_transform(),
                            *a_mass,
                            *bc_mass,
                            *a_moment_of_inertia,
                            *bc_moment_of_inertia,
                            &mut a_velocity,
                            &mut bc_velocity,
                            &mut a_angular_velocity,
                            &mut bc_angular_velocity,
                            contact,
                        );
                    }
                    cache.add(Collision(asteroid, boss_edge));
                    // a_health.0 = 0;
                    return;
                }
            }
            if let Some(contact) = detection::collision(
                *a_transform,
                *bc_transform,
                &a_collider,
                &bc_collider,
                Some(&meshes),
            ) {
                if !cache.contains(Collision(asteroid, boss_core)) {
                    response::compute(
                        &mut a_transform,
                        &mut bc_transform,
                        *a_mass,
                        *bc_mass,
                        *a_moment_of_inertia,
                        *bc_moment_of_inertia,
                        &mut a_velocity,
                        &mut bc_velocity,
                        &mut a_angular_velocity,
                        &mut bc_angular_velocity,
                        contact,
                    );
                }
                cache.add(Collision(asteroid, boss_core));
                // a_health.0 = 0;
                return;
            }
        }
    }
}

pub fn asteroids_and_spaceship(
    meshes: Res<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query: Query<
        (
            &mut AngularVelocity,
            &Collider,
            Entity,
            &mut Health,
            &Mass,
            &MomentOfInertia,
            &Transform,
            &mut Velocity,
        ),
        Or<(With<Asteroid>, With<Spaceship>)>,
    >,
) {
    let mut i = 0;
    loop {
        let mut iter = query.iter_mut().skip(i);
        if let Some((
            mut angular_velocity1,
            collider1,
            entity1,
            mut _health1,
            mass1,
            moment_of_inertia1,
            transform1,
            mut velocity1,
        )) = iter.next()
        {
            for (
                mut angular_velocity2,
                collider2,
                entity2,
                mut _health2,
                mass2,
                moment_of_inertia2,
                transform2,
                mut velocity2,
            ) in iter
            {
                if let Some(contact) = detection::collision(
                    *transform1,
                    *transform2,
                    &collider1,
                    &collider2,
                    Some(&meshes),
                ) {
                    // println!(
                    //     "{}",
                    //     mass1.0 * velocity1.0.length() + mass2.0 * velocity2.0.length()
                    // );
                    // commands
                    //     .spawn(crate::wreckage::WreckageDebris)
                    //     .insert(ColorMesh2dBundle {
                    //         mesh: meshes
                    //             .add(Mesh::from(shape::Circle {
                    //                 radius: 3.0,
                    //                 vertices: 32,
                    //             }))
                    //             .into(),
                    //         transform: Transform::from_xyz(contact.point.x, contact.point.y, 500.0),
                    //         ..Default::default()
                    //     });
                    // commands.insert_resource(NextState(GameState::Paused));
                    println!(
                        "normal: {}\nw1: {}\nw2: {}",
                        contact.normal, angular_velocity1.0, angular_velocity2.0
                    );
                    if !cache.contains(Collision(entity1, entity2)) {
                        response::compute(
                            transform1,
                            transform2,
                            *mass1,
                            *mass2,
                            *moment_of_inertia1,
                            *moment_of_inertia2,
                            &mut velocity1,
                            &mut velocity2,
                            &mut angular_velocity1,
                            &mut angular_velocity2,
                            contact,
                        );
                    }
                    println!(
                        "w'1: {}\nw'2: {}\n",
                        angular_velocity1.0, angular_velocity2.0
                    );
                    cache.add(Collision(entity1, entity2));
                    break;
                }
            }
            i += 1;
        } else {
            break;
        }
    }
}

#[cfg(fire)]
pub fn fire_and_fire(
    mut query_ally: Query<(&mut Health, &Transform), (With<Fire>, Without<Enemy>)>,
    mut query_enemy: Query<(&mut Health, &Transform), (With<Fire>, With<Enemy>)>,
) {
    for (mut a_health, a_transform) in query_ally.iter_mut() {
        for (mut e_health, e_transform) in query_enemy.iter_mut() {
            if (a_transform.translation - e_transform.translation).length() < 5.0 {
                a_health.0 = 0;
                e_health.0 = 0;
            }
        }
    }
}
