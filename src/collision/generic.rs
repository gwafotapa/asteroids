use bevy::prelude::*;
// use iyes_loopless::prelude::*;

use crate::{transform, AngularVelocity, Mass, MomentOfInertia, Part, Velocity};

use super::{
    // cache::{Cache, Collision},
    damages::DamageEvent,
    detection::{self, Collider, Contact},
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
        Without<Part>,
    >,
    query_c_part: Query<(&Collider, Entity, &Transform), (With<C>, With<Part>)>,
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
                        [(collider1p, entity1, transform1p), (collider2p, entity2, transform2p)],
                    ) = query_c_part.get_many([*child1, *child2])
                    {
                        if let Some((contact, time_c, transform1_c, transform2_c)) = intersection_at(
                            *mass1,
                            *mass2,
                            *moment_of_inertia1,
                            *moment_of_inertia2,
                            *transform1,
                            *transform2,
                            *velocity1,
                            *velocity2,
                            *angular_velocity1,
                            *angular_velocity2,
                            *transform1p,
                            *transform2p,
                            &collider1p,
                            &collider2p,
                            Res::clone(&meshes),
                            Res::clone(&time),
                        ) {
                            // if !cache.contains(Collision(spaceship, b_id)) {
                            response::compute_velocities(
                                &mut velocity1,
                                &mut velocity2,
                                &mut angular_velocity1,
                                &mut angular_velocity2,
                                &transform1_c,
                                &transform2_c,
                                *mass1,
                                *mass2,
                                *moment_of_inertia1,
                                *moment_of_inertia2,
                                contact,
                            );
                            [*transform1, *transform2] = [
                                transform::at(
                                    time.delta_seconds() - time_c,
                                    transform1_c,
                                    *velocity1,
                                    *angular_velocity1,
                                ),
                                transform::at(
                                    time.delta_seconds() - time_c,
                                    transform2_c,
                                    *velocity2,
                                    *angular_velocity2,
                                ),
                            ];
                            debug!(
                                "\nMore precise response\n\
				 translation1: {}, translation2: {}\n\
				 velocity1: {}, velocity2: {}\n",
                                transform1.translation,
                                transform2.translation,
                                velocity1.0,
                                velocity2.0,
                            );

                            let dv = (velocity1.0 - velocity2.0).length();
                            // println!("health1: {}, h1: {}", health1.0, h1);
                            // println!("health2: {}, h2: {}", health2.0, h2);
                            let damage1 = (mass2.0 / mass1.0 * dv / 10.0) as u32 + 1;
                            let damage2 = (mass1.0 / mass2.0 * dv / 10.0) as u32 + 1;
                            damage_event.send(DamageEvent {
                                entity: entity1,
                                extent: damage1,
                            });
                            damage_event.send(DamageEvent {
                                entity: entity2,
                                extent: damage2,
                            });

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
    query_c1_part: Query<(&Collider, Entity, &Transform), (With<C1>, With<Part>)>,
    query_c2_part: Query<(&Collider, Entity, &Transform), (With<C2>, With<Part>, Without<C1>)>,
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
                if let Ok((collider1p, entity1, transform1p)) = query_c1_part.get(*child1) {
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
                                if let Ok((collider2p, entity2, transform2p)) =
                                    query_c2_part.get(*child2)
                                {
                                    if let Some((contact, time_c, transform1_c, transform2_c)) =
                                        intersection_at(
                                            *mass1,
                                            *mass2,
                                            *moment_of_inertia1,
                                            *moment_of_inertia2,
                                            *transform1,
                                            *transform2,
                                            *velocity1,
                                            *velocity2,
                                            *angular_velocity1,
                                            *angular_velocity2,
                                            *transform1p,
                                            *transform2p,
                                            &collider1p,
                                            &collider2p,
                                            Res::clone(&meshes),
                                            Res::clone(&time),
                                        )
                                    {
                                        // if !cache.contains(Collision(spaceship, b_id)) {
                                        response::compute_velocities(
                                            &mut velocity1,
                                            &mut velocity2,
                                            &mut angular_velocity1,
                                            &mut angular_velocity2,
                                            &transform1_c,
                                            &transform2_c,
                                            *mass1,
                                            *mass2,
                                            *moment_of_inertia1,
                                            *moment_of_inertia2,
                                            contact,
                                        );
                                        [*transform1, *transform2] = [
                                            transform::at(
                                                time.delta_seconds() - time_c,
                                                transform1_c,
                                                *velocity1,
                                                *angular_velocity1,
                                            ),
                                            transform::at(
                                                time.delta_seconds() - time_c,
                                                transform2_c,
                                                *velocity2,
                                                *angular_velocity2,
                                            ),
                                        ];
                                        debug!(
                                            "\nMore precise response\n\
					     translation1: {}, translation2: {}\n\
					     velocity1: {}, velocity2: {}\n",
                                            transform1.translation,
                                            transform2.translation,
                                            velocity1.0,
                                            velocity2.0,
                                        );

                                        let dv = (velocity1.0 - velocity2.0).length();
                                        // println!("health1: {}, h1: {}", health1.0, h1);
                                        // println!("health2: {}, h2: {}", health2.0, h2);
                                        let damage1 = (mass2.0 / mass1.0 * dv / 10.0) as u32 + 1;
                                        let damage2 = (mass1.0 / mass2.0 * dv / 10.0) as u32 + 1;
                                        damage_event.send(DamageEvent {
                                            entity: entity1,
                                            extent: damage1,
                                        });
                                        damage_event.send(DamageEvent {
                                            entity: entity2,
                                            extent: damage2,
                                        });
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
    query_part: Query<(&Collider, Entity, &Transform), (Or<(With<C1>, With<C2>)>, With<Part>)>,
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
                        [(collider1p, entity1, transform1p), (collider2p, entity2, transform2p)],
                    ) = query_part.get_many([*child1, *child2])
                    {
                        if let Some((contact, time_c, transform1_c, transform2_c)) = intersection_at(
                            *mass1,
                            *mass2,
                            *moment_of_inertia1,
                            *moment_of_inertia2,
                            *transform1,
                            *transform2,
                            *velocity1,
                            *velocity2,
                            *angular_velocity1,
                            *angular_velocity2,
                            *transform1p,
                            *transform2p,
                            &collider1p,
                            &collider2p,
                            Res::clone(&meshes),
                            Res::clone(&time),
                        ) {
                            // if !cache.contains(Collision(spaceship, b_id)) {
                            response::compute_velocities(
                                &mut velocity1,
                                &mut velocity2,
                                &mut angular_velocity1,
                                &mut angular_velocity2,
                                &transform1_c,
                                &transform2_c,
                                *mass1,
                                *mass2,
                                *moment_of_inertia1,
                                *moment_of_inertia2,
                                contact,
                            );
                            [*transform1, *transform2] = [
                                transform::at(
                                    time.delta_seconds() - time_c,
                                    transform1_c,
                                    *velocity1,
                                    *angular_velocity1,
                                ),
                                transform::at(
                                    time.delta_seconds() - time_c,
                                    transform2_c,
                                    *velocity2,
                                    *angular_velocity2,
                                ),
                            ];
                            debug!(
                                "\nMore precise response\n\
			     translation1: {}, translation2: {}\n\
			     velocity1: {}, velocity2: {}\n",
                                transform1.translation,
                                transform2.translation,
                                velocity1.0,
                                velocity2.0,
                            );

                            let dv = (velocity1.0 - velocity2.0).length();
                            // println!("health1: {}, h1: {}", health1.0, h1);
                            // println!("health2: {}, h2: {}", health2.0, h2);
                            let damage1 = (mass2.0 / mass1.0 * dv / 10.0) as u32 + 1;
                            let damage2 = (mass1.0 / mass2.0 * dv / 10.0) as u32 + 1;
                            damage_event.send(DamageEvent {
                                entity: entity1,
                                extent: damage1,
                            });
                            damage_event.send(DamageEvent {
                                entity: entity2,
                                extent: damage2,
                            });

                            // cache.add(Collision(spaceship, b_id));
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
}

pub fn intersection_at(
    mass1: Mass,
    mass2: Mass,
    moment_of_inertia1: MomentOfInertia,
    moment_of_inertia2: MomentOfInertia,
    transform1: Transform,
    transform2: Transform,
    velocity1: Velocity,
    velocity2: Velocity,
    angular_velocity1: AngularVelocity,
    angular_velocity2: AngularVelocity,
    transform1p: Transform,
    transform2p: Transform,
    collider1p: &Collider,
    collider2p: &Collider,
    meshes: Res<Assets<Mesh>>,
    time: Res<Time>,
) -> Option<(Contact, f32, Transform, Transform)> {
    if let Some(mut contact_c) = detection::intersection(
        transform::global_of(transform1p, transform1),
        transform::global_of(transform2p, transform2),
        collider1p,
        collider2p,
        Some(Res::clone(&meshes)),
    ) {
        let [mut time_a, mut time_c] = [0.0, time.delta_seconds()];
        let [mut transform1_a, mut transform2_a] = [
            transform::at(-time_c, transform1, velocity1, angular_velocity1),
            transform::at(-time_c, transform2, velocity2, angular_velocity2),
        ];
        let [mut transform1_c, mut transform2_c] = [transform1, transform2];

        let [mut v1, mut v2] = [velocity1, velocity2];
        let [mut w1, mut w2] = [angular_velocity1, angular_velocity2];
        super::response::compute_velocities(
            &mut v1,
            &mut v2,
            &mut w1,
            &mut w2,
            &transform1_c,
            &transform2_c,
            mass1,
            mass2,
            moment_of_inertia1,
            moment_of_inertia2,
            contact_c,
        );
        debug!(
            "\nCollision detected at time tc\n\
             translation1: {}, translation2: {}\n\
	     Standard response\n\
	     velocity1: {}, velocity2: {}\n\
	     Rewind\n\
             translation1_a: {}, translation2_a: {}\n\
             ta = {}, tc = {}, contact = {:?}",
            transform1_c.translation,
            transform2_c.translation,
            v1.0,
            v2.0,
            transform1_a.translation,
            transform2_a.translation,
            time_a,
            time_c,
            contact_c
        );

        while time_c - time_a > detection::EPSILON {
            let time_b = (time_a + time_c) / 2.0;
            let [transform1_b, transform2_b] = [
                transform::at(time_b - time_a, transform1_a, velocity1, angular_velocity1),
                transform::at(time_b - time_a, transform2_a, velocity2, angular_velocity2),
            ];

            if let Some(contact_b) = detection::intersection(
                transform::global_of(transform1p, transform1_b),
                transform::global_of(transform2p, transform2_b),
                collider1p,
                collider2p,
                Some(Res::clone(&meshes)),
            ) {
                contact_c = contact_b;
                [transform1_c, transform2_c] = [transform1_b, transform2_b];
                time_c = time_b;
            } else {
                [transform1_a, transform2_a] = [transform1_b, transform2_b];
                time_a = time_b;
            }

            debug!(
                "\nta = {}, tc = {}, contact = {:?}",
                time_a, time_c, contact_c
            );
        }

        Some((contact_c, time_c, transform1_c, transform2_c))
    } else {
        None
    }
}
