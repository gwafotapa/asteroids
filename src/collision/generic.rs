use bevy::prelude::*;
// use iyes_loopless::prelude::*;

use crate::{transform, AngularVelocity, Health, Mass, MomentOfInertia, Part, Velocity};

use super::{
    // cache::{Cache, Collision},
    damages::DamageEvent,
    detection::{self, Collider},
    response,
};

pub fn with<C: Component>(
    // mut cache: ResMut<Cache>,
    mut damage_event: EventWriter<DamageEvent>,
    mut query_c: Query<
        (
            &mut AngularVelocity,
            Option<&Children>,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        (With<C>, Without<Part>),
    >,
    query_c_part: Query<(&Collider, Entity, &Health, &Transform), (With<C>, With<Part>)>,
    meshes: Res<Assets<Mesh>>,
    time: Res<Time>,
) {
    let mut combinations = query_c.iter_combinations_mut();
    'outer: while let Some(
        [(
            mut angular_velocity1,
            maybe_children1,
            mass1,
            moment_of_inertia1,
            mut transform1,
            mut velocity1,
        ), (
            mut angular_velocity2,
            maybe_children2,
            mass2,
            moment_of_inertia2,
            mut transform2,
            mut velocity2,
        )],
    ) = combinations.fetch_next()
    {
        if let Some((children1, children2)) = maybe_children1.zip(maybe_children2) {
            for child1 in children1 {
                for child2 in children2 {
                    if let Ok(
                        [(collider1p, entity1p, health1p, transform1p), (collider2p, entity2p, health2p, transform2p)],
                    ) = query_c_part.get_many([*child1, *child2])
                    {
                        let mut time_c = time.delta_seconds();
                        if let Some(contact) = detection::intersection_at(
                            &mut transform1,
                            &mut transform2,
                            &mut time_c,
                            *mass1,
                            *mass2,
                            *moment_of_inertia1,
                            *moment_of_inertia2,
                            *velocity1,
                            *velocity2,
                            *angular_velocity1,
                            *angular_velocity2,
                            *transform1p,
                            *transform2p,
                            &collider1p,
                            &collider2p,
                            Res::clone(&meshes),
                        ) {
                            // if !cache.contains(Collision(spaceship, b_id)) {
                            response::compute_velocities(
                                &mut velocity1,
                                &mut velocity2,
                                &mut angular_velocity1,
                                &mut angular_velocity2,
                                *transform1,
                                *transform2,
                                *mass1,
                                *mass2,
                                *moment_of_inertia1,
                                *moment_of_inertia2,
                                contact,
                            );

                            let dv = (velocity1.0 - velocity2.0).length();
                            // println!("health1: {}, h1: {}", health1.0, h1);
                            // println!("health2: {}, h2: {}", health2.0, h2);
                            let damage1 = (mass2.0 / mass1.0 * dv / 10.0) as u32 + 1;
                            let damage2 = (mass1.0 / mass2.0 * dv / 10.0) as u32 + 1;
                            damage_event.send(DamageEvent {
                                entity: entity1p,
                                extent: damage1,
                            });
                            damage_event.send(DamageEvent {
                                entity: entity2p,
                                extent: damage2,
                            });

                            if damage1 < health1p.0 {
                                *transform1 = transform::at(
                                    time.delta_seconds() - time_c,
                                    *transform1,
                                    *velocity1,
                                    *angular_velocity1,
                                );
                            }

                            if damage2 < health2p.0 {
                                *transform2 = transform::at(
                                    time.delta_seconds() - time_c,
                                    *transform2,
                                    *velocity2,
                                    *angular_velocity2,
                                );
                            }

                            debug!(
                                "translation1_c = {}, translation2_c = {}\n\
				 velocity1 = {}, velocity2 = {}\n",
                                transform1.translation,
                                transform2.translation,
                                velocity1.0,
                                velocity2.0,
                            );

                            // cache.add(Collision(spaceship, b_id));
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
}

pub fn between<C1: Component, C2: Component>(
    // mut cache: ResMut<Cache>,
    mut damage_event: EventWriter<DamageEvent>,
    mut query_c1: Query<
        (
            &mut AngularVelocity,
            Option<&Children>,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        (With<C1>, Without<Part>),
    >,
    mut query_c2: Query<
        (
            &mut AngularVelocity,
            Option<&Children>,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        (With<C2>, Without<Part>, Without<C1>),
    >,
    query_c1_part: Query<(&Collider, Entity, &Health, &Transform), (With<C1>, With<Part>)>,
    query_c2_part: Query<
        (&Collider, Entity, &Health, &Transform),
        (With<C2>, With<Part>, Without<C1>),
    >,
    meshes: Res<Assets<Mesh>>,
    time: Res<Time>,
) {
    'outer: for (
        mut angular_velocity1,
        maybe_children1,
        mass1,
        moment_of_inertia1,
        mut transform1,
        mut velocity1,
    ) in query_c1.iter_mut()
    {
        if let Some(children1) = maybe_children1 {
            for child1 in children1 {
                if let Ok((collider1p, entity1p, health1p, transform1p)) =
                    query_c1_part.get(*child1)
                {
                    for (
                        mut angular_velocity2,
                        maybe_children2,
                        mass2,
                        moment_of_inertia2,
                        mut transform2,
                        mut velocity2,
                    ) in query_c2.iter_mut()
                    {
                        if let Some(children2) = maybe_children2 {
                            for child2 in children2 {
                                if let Ok((collider2p, entity2p, health2p, transform2p)) =
                                    query_c2_part.get(*child2)
                                {
                                    let mut time_c = time.delta_seconds();
                                    if let Some(contact) = detection::intersection_at(
                                        &mut transform1,
                                        &mut transform2,
                                        &mut time_c,
                                        *mass1,
                                        *mass2,
                                        *moment_of_inertia1,
                                        *moment_of_inertia2,
                                        *velocity1,
                                        *velocity2,
                                        *angular_velocity1,
                                        *angular_velocity2,
                                        *transform1p,
                                        *transform2p,
                                        &collider1p,
                                        &collider2p,
                                        Res::clone(&meshes),
                                    ) {
                                        // if !cache.contains(Collision(spaceship, b_id)) {
                                        response::compute_velocities(
                                            &mut velocity1,
                                            &mut velocity2,
                                            &mut angular_velocity1,
                                            &mut angular_velocity2,
                                            *transform1,
                                            *transform2,
                                            *mass1,
                                            *mass2,
                                            *moment_of_inertia1,
                                            *moment_of_inertia2,
                                            contact,
                                        );

                                        let dv = (velocity1.0 - velocity2.0).length();
                                        // println!("health1: {}, h1: {}", health1.0, h1);
                                        // println!("health2: {}, h2: {}", health2.0, h2);
                                        let damage1 = (mass2.0 / mass1.0 * dv / 10.0) as u32 + 1;
                                        let damage2 = (mass1.0 / mass2.0 * dv / 10.0) as u32 + 1;
                                        damage_event.send(DamageEvent {
                                            entity: entity1p,
                                            extent: damage1,
                                        });
                                        damage_event.send(DamageEvent {
                                            entity: entity2p,
                                            extent: damage2,
                                        });
                                        if damage1 < health1p.0 {
                                            *transform1 = transform::at(
                                                time.delta_seconds() - time_c,
                                                *transform1,
                                                *velocity1,
                                                *angular_velocity1,
                                            );
                                        }

                                        if damage2 < health2p.0 {
                                            *transform2 = transform::at(
                                                time.delta_seconds() - time_c,
                                                *transform2,
                                                *velocity2,
                                                *angular_velocity2,
                                            );
                                        }

                                        debug!(
                                            "translation1_c = {}, translation2_c = {}\n\
					     velocity1 = {}, velocity2 = {}\n",
                                            transform1.translation,
                                            transform2.translation,
                                            velocity1.0,
                                            velocity2.0,
                                        );

                                        // cache.add(Collision(spaceship, b_id));
                                        continue 'outer;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn among<C1: Component, C2: Component>(
    // mut cache: ResMut<Cache>,
    mut damage_event: EventWriter<DamageEvent>,
    mut query: Query<
        (
            &mut AngularVelocity,
            Option<&Children>,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        (Or<(With<C1>, With<C2>)>, Without<Part>),
    >,
    query_part: Query<
        (&Collider, Entity, &Health, &Transform),
        (Or<(With<C1>, With<C2>)>, With<Part>),
    >,
    meshes: Res<Assets<Mesh>>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    'outer: while let Some(
        [(
            mut angular_velocity1,
            maybe_children1,
            mass1,
            moment_of_inertia1,
            mut transform1,
            mut velocity1,
        ), (
            mut angular_velocity2,
            maybe_children2,
            mass2,
            moment_of_inertia2,
            mut transform2,
            mut velocity2,
        )],
    ) = combinations.fetch_next()
    {
        if let Some((children1, children2)) = maybe_children1.zip(maybe_children2) {
            for child1 in children1 {
                for child2 in children2 {
                    if let Ok(
                        [(collider1p, entity1p, health1p, transform1p), (collider2p, entity2p, health2p, transform2p)],
                    ) = query_part.get_many([*child1, *child2])
                    {
                        let mut time_c = time.delta_seconds();
                        if let Some(contact) = detection::intersection_at(
                            &mut transform1,
                            &mut transform2,
                            &mut time_c,
                            *mass1,
                            *mass2,
                            *moment_of_inertia1,
                            *moment_of_inertia2,
                            *velocity1,
                            *velocity2,
                            *angular_velocity1,
                            *angular_velocity2,
                            *transform1p,
                            *transform2p,
                            &collider1p,
                            &collider2p,
                            Res::clone(&meshes),
                        ) {
                            // if !cache.contains(Collision(spaceship, b_id)) {
                            response::compute_velocities(
                                &mut velocity1,
                                &mut velocity2,
                                &mut angular_velocity1,
                                &mut angular_velocity2,
                                *transform1,
                                *transform2,
                                *mass1,
                                *mass2,
                                *moment_of_inertia1,
                                *moment_of_inertia2,
                                contact,
                            );

                            let dv = (velocity1.0 - velocity2.0).length();
                            // println!("health1: {}, h1: {}", health1.0, h1);
                            // println!("health2: {}, h2: {}", health2.0, h2);
                            let damage1 = (mass2.0 / mass1.0 * dv / 10.0) as u32 + 1;
                            let damage2 = (mass1.0 / mass2.0 * dv / 10.0) as u32 + 1;
                            damage_event.send(DamageEvent {
                                entity: entity1p,
                                extent: damage1,
                            });
                            damage_event.send(DamageEvent {
                                entity: entity2p,
                                extent: damage2,
                            });

                            if damage1 < health1p.0 {
                                *transform1 = transform::at(
                                    time.delta_seconds() - time_c,
                                    *transform1,
                                    *velocity1,
                                    *angular_velocity1,
                                );
                            }

                            if damage2 < health2p.0 {
                                *transform2 = transform::at(
                                    time.delta_seconds() - time_c,
                                    *transform2,
                                    *velocity2,
                                    *angular_velocity2,
                                );
                            }

                            debug!(
                                "translation1_c = {}, translation2_c = {}\n\
				 velocity1 = {}, velocity2 = {}\n",
                                transform1.translation,
                                transform2.translation,
                                velocity1.0,
                                velocity2.0,
                            );

                            // cache.add(Collision(spaceship, b_id));
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
}
