use bevy::prelude::*;

use crate::{AngularVelocity, Health, Mass, MomentOfInertia, Velocity};

use super::{
    cache::{Cache, Collision},
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

pub fn with<C: Component>(
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
        With<C>,
    >,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some(
        [(
            mut angular_velocity1,
            collider1,
            entity1,
            mut health1,
            mass1,
            moment_of_inertia1,
            transform1,
            mut velocity1,
        ), (
            mut angular_velocity2,
            collider2,
            entity2,
            mut health2,
            mass2,
            moment_of_inertia2,
            transform2,
            mut velocity2,
        )],
    ) = combinations.fetch_next()
    {
        if let Some(contact) = detection::collision(
            *transform1,
            *transform2,
            &collider1,
            &collider2,
            Some(&meshes),
        ) {
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
                let dv = (velocity1.0 - velocity2.0).length();
                let h1 = (mass2.0 / mass1.0 * dv / 10.0) as i32 + 1;
                let h2 = (mass1.0 / mass2.0 * dv / 10.0) as i32 + 1;
                // println!("health1: {}, h1: {}", health1.0, h1);
                // println!("health2: {}, h2: {}", health2.0, h2);
                health1.0 -= h1;
                health2.0 -= h2;
            }
            cache.add(Collision(entity1, entity2));
        }
    }
}

pub fn between<C1: Component, C2: Component>(
    meshes: Res<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query_c1: Query<
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
        With<C1>,
    >,
    mut query_c2: Query<
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
        (With<C2>, Without<C1>),
    >,
) {
    for (
        mut c1_angular_velocity,
        c1_collider,
        c1_entity,
        mut c1_health,
        c1_mass,
        c1_moment_of_inertia,
        c1_transform,
        mut c1_velocity,
    ) in query_c1.iter_mut()
    {
        for (
            mut c2_angular_velocity,
            c2_collider,
            c2_entity,
            mut c2_health,
            c2_mass,
            c2_moment_of_inertia,
            c2_transform,
            mut c2_velocity,
        ) in query_c2.iter_mut()
        {
            if let Some(contact) = detection::collision(
                *c1_transform,
                *c2_transform,
                &c1_collider,
                &c2_collider,
                Some(&meshes),
            ) {
                if !cache.contains(Collision(c1_entity, c2_entity)) {
                    response::compute(
                        c1_transform,
                        c2_transform,
                        *c1_mass,
                        *c2_mass,
                        *c1_moment_of_inertia,
                        *c2_moment_of_inertia,
                        &mut c1_velocity,
                        &mut c2_velocity,
                        &mut c1_angular_velocity,
                        &mut c2_angular_velocity,
                        contact,
                    );
                    let dv = (c1_velocity.0 - c2_velocity.0).length();
                    c1_health.0 -= (c2_mass.0 / c1_mass.0 * dv / 10.0) as i32 + 1;
                    c2_health.0 -= (c1_mass.0 / c2_mass.0 * dv / 10.0) as i32 + 1;
                }
                cache.add(Collision(c1_entity, c2_entity));
                break;
            }
        }
    }
}

pub fn among<C1: Component, C2: Component, C3: Component>(
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
        Or<(With<C1>, With<C2>, With<C3>)>,
    >,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some(
        [(
            mut angular_velocity1,
            collider1,
            entity1,
            mut health1,
            mass1,
            moment_of_inertia1,
            transform1,
            mut velocity1,
        ), (
            mut angular_velocity2,
            collider2,
            entity2,
            mut health2,
            mass2,
            moment_of_inertia2,
            transform2,
            mut velocity2,
        )],
    ) = combinations.fetch_next()
    {
        if let Some(contact) = detection::collision(
            *transform1,
            *transform2,
            &collider1,
            &collider2,
            Some(&meshes),
        ) {
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
                let dv = (velocity1.0 - velocity2.0).length();
                let h1 = (mass2.0 / mass1.0 * dv / 10.0) as i32 + 1;
                let h2 = (mass1.0 / mass2.0 * dv / 10.0) as i32 + 1;
                // println!("health1: {}, h1: {}", health1.0, h1);
                // println!("health2: {}, h2: {}", health2.0, h2);
                health1.0 -= h1;
                health2.0 -= h2;
            }
            cache.add(Collision(entity1, entity2));
        }
    }
}
