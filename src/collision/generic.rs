use bevy::prelude::*;

use crate::{transform, AngularVelocity, Health, Mass, MomentOfInertia, Velocity};

use super::{
    cache::Cache,
    damages::{Damageable, Damages},
    detection::{self, Collider},
    response,
};

// pub fn asteroid_fire_spaceship(
//     meshes: Res<Assets<Mesh>>,
//     mut cache: ResMut<Cache>,
//     mut query: Query<
//         (
//             &mut AngularVelocity,
//             &Collider,
//             Entity,
//             &mut Health,
//             &Mass,
//             &MomentOfInertia,
//             &Transform,
//             &mut Velocity,
//         ),
//         Or<(With<Asteroid>, With<Fire>, With<Spaceship>)>,
//     >,
// ) {
//     let mut combinations = query.iter_combinations_mut();
//     while let Some(
//         [(
//             mut angular_velocity1,
//             collider1,
//             entity1,
//             mut health1,
//             mass1,
//             moment_of_inertia1,
//             transform1,
//             mut velocity1,
//         ), (
//             mut angular_velocity2,
//             collider2,
//             entity2,
//             mut health2,
//             mass2,
//             moment_of_inertia2,
//             transform2,
//             mut velocity2,
//         )],
//     ) = combinations.fetch_next()
//     {
//         if let Some(contact) = detection::collision(
//             *transform1,
//             *transform2,
//             &collider1,
//             &collider2,
//             Some(&meshes),
//         ) {
//             // println!(
//             //     "m1(v1 - v2): {}\n",
//             //     (mass1.0 * velocity1.0 - mass2.0 * velocity2.0).length()
//             // );

//             // println!(
//             //     "{}",
//             //     mass1.0 * velocity1.0.length() + mass2.0 * velocity2.0.length()
//             // );
//             // commands
//             //     .spawn(crate::wreckage::WreckageDebris)
//             //     .insert(ColorMesh2dBundle {
//             //         mesh: meshes
//             //             .add(Mesh::from(shape::Circle {
//             //                 radius: 3.0,
//             //                 vertices: 32,
//             //             }))
//             //             .into(),
//             //         transform: Transform::from_xyz(contact.point.x, contact.point.y, 500.0),
//             //         ..Default::default()
//             //     });
//             // commands.insert_resource(NextState(GameState::Paused));
//             // println!(
//             //     "normal: {}\nw1: {}\nw2: {}",
//             //     contact.normal, angular_velocity1.0, angular_velocity2.0
//             // );
//             if !cache.contains(Collision(entity1, entity2)) {
//                 response::compute(
//                     transform1,
//                     transform2,
//                     *mass1,
//                     *mass2,
//                     *moment_of_inertia1,
//                     *moment_of_inertia2,
//                     &mut velocity1,
//                     &mut velocity2,
//                     &mut angular_velocity1,
//                     &mut angular_velocity2,
//                     contact,
//                 );
//                 let dv = (velocity1.0 - velocity2.0).length();
//                 let h1 = (mass2.0 / mass1.0 * dv / 10.0) as i32 + 1;
//                 let h2 = (mass1.0 / mass2.0 * dv / 10.0) as i32 + 1;
//                 println!("health1: {}, h1: {}", health1.0, h1);
//                 println!("health2: {}, h2: {}", health2.0, h2);
//                 health1.0 -= h1;
//                 health2.0 -= h2;
//                 // println!("m1: {}, v1: {}", mass1.0, velocity1.0);
//                 // println!("m2: {}, v2: {}", mass2.0, velocity2.0);
//                 // println!("dv = v1 - v2: {}", dv);
//                 // println!("m2/m1 * dv: {}, m1/m2 * dv: {}", h1, h2);
//             }
//             // println!(
//             //     "w'1: {}\nw'2: {}\n",
//             //     angular_velocity1.0, angular_velocity2.0
//             // );
//             cache.add(Collision(entity1, entity2));
//         }
//     }
// }

pub fn with<C: Component + Damageable>(
    meshes: Res<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query: Query<(
        &mut AngularVelocity,
        &C,
        &mut Collider,
        Entity,
        &mut Health,
        &Mass,
        &MomentOfInertia,
        &mut Transform,
        &mut Velocity,
    )>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some(
        [(
            mut angular_velocity1,
            component1,
            mut collider1,
            entity1,
            mut health1,
            mass1,
            moment_of_inertia1,
            mut transform1,
            mut velocity1,
        ), (
            mut angular_velocity2,
            component2,
            mut collider2,
            entity2,
            mut health2,
            mass2,
            moment_of_inertia2,
            mut transform2,
            mut velocity2,
        )],
    ) = combinations.fetch_next()
    {
        if let Some((contact, time_c, transform1_c, transform2_c)) = detection::intersection_at(
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
            &collider1,
            &collider2,
            Res::clone(&meshes),
            Res::clone(&time),
        ) {
            // if !cache.contains(Collision(entity1, entity2)) {
            response::compute(
                &transform1_c,
                &transform2_c,
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
		 translation 1: {}, translation 2 :{}\n\
		 velocity 1: {}, velocity 2: {}\n",
                transform1.translation, transform2.translation, velocity1.0, velocity2.0,
            );

            let dv = (velocity1.0 - velocity2.0).length();
            // println!("health1: {}, h1: {}", health1.0, h1);
            // println!("health2: {}, h2: {}", health2.0, h2);
            component1.damage(
                &mut health1,
                &mut collider1,
                Damages {
                    location: contact.point.extend(0.0),
                    extent: (mass2.0 / mass1.0 * dv / 10.0) as u32 + 1,
                },
            );
            component2.damage(
                &mut health2,
                &mut collider2,
                Damages {
                    location: contact.point.extend(0.0),
                    extent: (mass1.0 / mass2.0 * dv / 10.0) as u32 + 1,
                },
            );
            // }
            // cache.add(Collision(entity1, entity2));
        }
    }
}

pub fn between<C1: Component + Damageable, C2: Component + Damageable>(
    meshes: Res<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query_c1: Query<(
        &mut AngularVelocity,
        &C1,
        &mut Collider,
        Entity,
        &mut Health,
        &Mass,
        &MomentOfInertia,
        &mut Transform,
        &mut Velocity,
    )>,
    mut query_c2: Query<
        (
            &mut AngularVelocity,
            &C2,
            &mut Collider,
            Entity,
            &mut Health,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        Without<C1>,
    >,
    time: Res<Time>,
) {
    for (
        mut angular_velocity1,
        component1,
        mut collider1,
        entity1,
        mut health1,
        mass1,
        moment_of_inertia1,
        mut transform1,
        mut velocity1,
    ) in query_c1.iter_mut()
    {
        for (
            mut angular_velocity2,
            component2,
            mut collider2,
            entity2,
            mut health2,
            mass2,
            moment_of_inertia2,
            mut transform2,
            mut velocity2,
        ) in query_c2.iter_mut()
        {
            if let Some((contact, time_c, transform1_c, transform2_c)) = detection::intersection_at(
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
                &collider1,
                &collider2,
                Res::clone(&meshes),
                Res::clone(&time),
            ) {
                // if !cache.contains(Collision(c1_entity, c2_entity)) {
                response::compute(
                    &transform1_c,
                    &transform2_c,
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

                let dv = (velocity1.0 - velocity2.0).length();
                component1.damage(
                    &mut health1,
                    &mut collider1,
                    Damages {
                        location: contact.point.extend(0.0),
                        extent: (mass2.0 / mass1.0 * dv / 10.0) as u32 + 1,
                    },
                );
                component2.damage(
                    &mut health2,
                    &mut collider2,
                    Damages {
                        location: contact.point.extend(0.0),
                        extent: (mass1.0 / mass2.0 * dv / 10.0) as u32 + 1,
                    },
                );
                // }
                // cache.add(Collision(c1_entity, c2_entity));
                break;
            }
        }
    }
}

pub fn among<C1: Component + Damageable, C2: Component + Damageable, C3: Component + Damageable>(
    meshes: Res<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query: Query<
        (
            &mut AngularVelocity,
            Option<&C1>,
            Option<&C2>,
            Option<&C3>,
            &mut Collider,
            Entity,
            &mut Health,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        Or<(With<C1>, With<C2>, With<C3>)>,
    >,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some(
        [(
            mut angular_velocity1,
            component11,
            component12,
            component13,
            mut collider1,
            entity1,
            mut health1,
            mass1,
            moment_of_inertia1,
            mut transform1,
            mut velocity1,
        ), (
            mut angular_velocity2,
            component21,
            component22,
            component23,
            mut collider2,
            entity2,
            mut health2,
            mass2,
            moment_of_inertia2,
            mut transform2,
            mut velocity2,
        )],
    ) = combinations.fetch_next()
    {
        if let Some((contact, time_c, transform1_c, transform2_c)) = detection::intersection_at(
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
            &collider1,
            &collider2,
            Res::clone(&meshes),
            Res::clone(&time),
        ) {
            // if !cache.contains(Collision(entity1, entity2)) {
            response::compute(
                &transform1_c,
                &transform2_c,
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
		 translation 1: {}, translation 2 :{}\n\
		 velocity 1: {}, velocity 2: {}\n",
                transform1.translation, transform2.translation, velocity1.0, velocity2.0,
            );

            let dv = (velocity1.0 - velocity2.0).length();
            let damages1 = Damages {
                location: contact.point.extend(0.0),
                extent: (mass2.0 / mass1.0 * dv / 10.0) as u32 + 1,
            };
            let damages2 = Damages {
                location: contact.point.extend(0.0),
                extent: (mass1.0 / mass2.0 * dv / 10.0) as u32 + 1,
            };
            if let Some(component) = component11 {
                component.damage(&mut health1, &mut collider1, damages1);
            } else if let Some(component) = component12 {
                component.damage(&mut health1, &mut collider1, damages1);
            } else if let Some(component) = component13 {
                component.damage(&mut health1, &mut collider1, damages1);
            }
            if let Some(component) = component11 {
                component.damage(&mut health2, &mut collider2, damages2);
            } else if let Some(component) = component12 {
                component.damage(&mut health2, &mut collider2, damages2);
            } else if let Some(component) = component13 {
                component.damage(&mut health2, &mut collider2, damages2);
            }

            // println!("health1: {}, h1: {}", health1.0, h1);
            // println!("health2: {}, h2: {}", health2.0, h2);
            // }
            // cache.add(Collision(entity1, entity2));
        }
    }
}
