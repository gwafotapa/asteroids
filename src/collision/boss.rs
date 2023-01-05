use bevy::prelude::*;
// use iyes_loopless::prelude::*;

use crate::{
    asteroid::Asteroid,
    boss::{Boss, BossCore, BossEdge, BossPart},
    fire::{Enemy, Fire},
    spaceship::Spaceship,
    transform, AngularVelocity, Health, Mass, MomentOfInertia, Velocity,
};

use super::{
    cache::{Cache, Collision},
    detection::{self, Collider, Contact},
    response,
};

pub fn with_fire(
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_fire: Query<
        (
            &AngularVelocity,
            &Collider,
            &Handle<ColorMaterial>,
            &mut Health,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &Velocity,
        ),
        (With<Fire>, Without<Enemy>),
    >,
    query_boss: Query<
        (
            &AngularVelocity,
            &Boss,
            &Mass,
            &MomentOfInertia,
            &Transform,
            &Velocity,
        ),
        Without<Fire>,
    >,
    mut query_boss_part: Query<
        (
            Option<&BossEdge>,
            &Collider,
            &Handle<ColorMaterial>,
            &mut Health,
            &Transform,
        ),
        (Or<(With<BossCore>, With<BossEdge>)>, Without<Fire>),
    >,
    time: Res<Time>,
) {
    let (b_angular_velocity, boss, b_mass, b_moment_of_inertia, b_transform, b_velocity) =
        query_boss.single();
    for (bp_edge, bp_collider, bp_color, mut bp_health, bp_transform) in query_boss_part.iter_mut()
    {
        let bp_global_transform =
            Transform::from_translation(b_transform.transform_point(bp_transform.translation))
                .with_rotation(b_transform.rotation * bp_transform.rotation);
        for (
            f_angular_velocity,
            f_collider,
            f_color,
            mut f_health,
            f_mass,
            f_moment_of_inertia,
            mut f_transform,
            f_velocity,
        ) in query_fire.iter_mut()
        {
            if let Some((contact, time_c, transform1_c, transform2_c)) = detection::intersection_at(
                *f_mass,
                *b_mass,
                *f_moment_of_inertia,
                *b_moment_of_inertia,
                *f_transform,
                bp_global_transform,
                *f_velocity,
                *b_velocity,
                *f_angular_velocity,
                *b_angular_velocity,
                f_collider,
                bp_collider,
                Res::clone(&meshes),
                Res::clone(&time),
            ) {
                *f_transform = transform1_c;
                f_health.0 = 0;

                if bp_edge.is_some() || boss.edges == 0 {
                    bp_health.0 -= 1;
                    let [fr, fg, fb, _] = materials.get(f_color).unwrap().color.as_rgba_f32();
                    let bp_color = materials.get_mut(bp_color).unwrap();
                    let [mut r, mut g, mut b, _] = bp_color.color.as_rgba_f32();
                    r += (fr - r) / (1. + bp_health.0 as f32);
                    g += (fg - g) / (1. + bp_health.0 as f32);
                    b += (fb - b) / (1. + bp_health.0 as f32);
                    bp_color.color = Color::rgb(r, g, b);
                }
            }
        }
    }
}

pub fn with_asteroid_or_spaceship(
    // mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query_boss: Query<
        (
            &mut AngularVelocity,
            Entity,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        With<Boss>,
    >,
    mut query_boss_part: Query<(&Collider, &Transform), (With<BossPart>, Without<Boss>)>,
    mut query_asteroid_spaceship: Query<
        (
            &mut AngularVelocity,
            &Collider,
            Entity,
            &mut Health,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        (
            Or<(With<Asteroid>, With<Spaceship>)>,
            Without<Boss>,
            Without<BossPart>,
        ),
    >,
    time: Res<Time>,
) {
    if let Ok((
        mut b_angular_velocity,
        b_id,
        b_mass,
        b_moment_of_inertia,
        mut b_transform,
        mut b_velocity,
    )) = query_boss.get_single_mut()
    {
        for (
            mut as_angular_velocity,
            as_collider,
            spaceship,
            mut _as_health,
            as_mass,
            as_moment_of_inertia,
            mut as_transform,
            mut as_velocity,
        ) in query_asteroid_spaceship.iter_mut()
        {
            for (bp_collider, bp_transform) in query_boss_part.iter_mut() {
                let bp_global_transform = Transform::from_translation(
                    b_transform.transform_point(bp_transform.translation),
                )
                .with_rotation(b_transform.rotation * bp_transform.rotation);
                if let Some((contact, time_c, as_transform_c, bp_transform_c)) =
                    detection::intersection_at(
                        *as_mass,
                        *b_mass,
                        *as_moment_of_inertia,
                        *b_moment_of_inertia,
                        *as_transform,
                        bp_global_transform,
                        *as_velocity,
                        *b_velocity,
                        *as_angular_velocity,
                        *b_angular_velocity,
                        as_collider,
                        bp_collider,
                        Res::clone(&meshes),
                        Res::clone(&time),
                    )
                {
                    // TODO
                    // contact.normal = (as_transform.translation - b_transform.translation)
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

                    // if !cache.contains(Collision(spaceship, b_id)) {
                    // println!("spaceship -- w1: {}", as_angular_velocity.0);
                    // println!("boss      -- w2: {}", b_angular_velocity.0);
                    response::compute(
                        &mut as_transform,
                        &mut b_transform,
                        *as_mass,
                        *b_mass,
                        *as_moment_of_inertia,
                        *b_moment_of_inertia,
                        &mut as_velocity,
                        &mut b_velocity,
                        &mut as_angular_velocity,
                        &mut b_angular_velocity,
                        contact,
                    );
                    [*as_transform, *b_transform] = [
                        transform::at(
                            time.delta_seconds() - time_c,
                            as_transform_c,
                            *as_velocity,
                            *as_angular_velocity,
                        ),
                        transform::at(
                            time.delta_seconds() - time_c,
                            bp_transform_c,
                            *b_velocity,
                            *b_angular_velocity,
                        ),
                    ];
                    debug!(
                        "\nMore precise response\n\
			 translation 1: {}, translation 2 :{}\n\
			 velocity 1: {}, velocity 2: {}\n",
                        as_transform.translation,
                        b_transform.translation,
                        as_velocity.0,
                        b_velocity.0,
                    );

                    // println!("spaceship -- w'1: {}", as_angular_velocity.0);
                    // println!("boss      -- w'2: {}", b_angular_velocity.0);
                    // println!("");
                    // }
                    // cache.add(Collision(spaceship, b_id));
                    // as_health.0 = 0;
                    return;
                }
            }
        }
    }
}

// pub fn intersection_with_other_at(
//     b_mass: Mass,
//     o_mass: Mass,
//     b_moment_of_inertia: MomentOfInertia,
//     o_moment_of_inertia: MomentOfInertia,
//     b_transform: Transform,
//     bp_transform: Transform,
//     o_transform: Transform,
//     b_velocity: Velocity,
//     o_velocity: Velocity,
//     b_angular_velocity: AngularVelocity,
//     o_angular_velocity: AngularVelocity,
//     bp_collider: &Collider,
//     o_collider: &Collider,
//     meshes: Res<Assets<Mesh>>,
//     time: Res<Time>,
// ) -> Option<(Contact, f32, Transform, Transform)> {
//     let bp_global_transform =
//         Transform::from_translation(b_transform.transform_point(bp_transform.translation))
//             .with_rotation(b_transform.rotation * bp_transform.rotation);
//     if let Some(mut contact_c) = detection::intersection(
//         bp_global_transform,
//         o_transform,
//         bp_collider,
//         o_collider,
//         Some(Res::clone(&meshes)),
//     ) {
//         let [mut time_a, mut time_c] = [0.0, time.delta_seconds()];
//         let [mut b_transform_a, mut o_transform_a] = [
//             transform::at(-time_c, b_transform, b_velocity, b_angular_velocity),
//             transform::at(-time_c, o_transform, o_velocity, o_angular_velocity),
//         ];
//         let mut bp_global_transform_a =
//             Transform::from_translation(b_transform_a.transform_point(bp_transform.translation))
//                 .with_rotation(b_transform_a.rotation * bp_transform.rotation);
//         let [mut b_transform_c, mut o_transform_c] = [b_transform, o_transform];

//         let [mut v1, mut v2] = [b_velocity, o_velocity];
//         let [mut w1, mut w2] = [b_angular_velocity, o_angular_velocity];
//         super::response::compute(
//             &transform1_c,
//             &transform2_c,
//             b_mass,
//             o_mass,
//             b_moment_of_inertia,
//             o_moment_of_inertia,
//             &mut v1,
//             &mut v2,
//             &mut w1,
//             &mut w2,
//             contact_c,
//         );
//         debug!(
//             "\nCollision detected at time tc\n\
//              translation 1c: {}, translation 2c: {}\n\
// 	     Standard response\n\
// 	     velocity 1c: {}, velocity 2c: {}\n\
// 	     Rewind\n\
//              translation 1a: {}, translation 2a: {}\n\
//              ta = {}, tc = {}, contact = {:?}",
//             bp_global_transform_c.translation,
//             o_transform_c.translation,
//             v1.0,
//             v2.0,
//             bp_global_transform_a.translation,
//             o_transform_a.translation,
//             time_a,
//             time_c,
//             contact_c
//         );

//         while time_c - time_a > detection::EPSILON {
//             let time_b = (time_a + time_c) / 2.0;
//             let [b_transform_b, o_transform_b] = [
//                 transform::at(
//                     time_b - time_a,
//                     b_transform_a,
//                     b_velocity,
//                     b_angular_velocity,
//                 ),
//                 transform::at(
//                     time_b - time_a,
//                     o_transform_a,
//                     o_velocity,
//                     o_angular_velocity,
//                 ),
//             ];
//             let bp_global_transform_b = Transform::from_translation(
//                 b_transform_b.transform_point(bp_transform.translation),
//             )
//             .with_rotation(b_transform_b.rotation * bp_transform.rotation);

//             if let Some(contact_b) = detection::intersection(
//                 bp_transform_b,
//                 o_transform_b,
//                 bp_collider,
//                 o_collider,
//                 Some(Res::clone(&meshes)),
//             ) {
//                 contact_c = contact_b;
//                 [b_transform_c, o_transform_c] = [b_transform_b, o_transform_b];
//                 time_c = time_b;
//                 debug!(
//                     "\nta = {}, tc = {}, contact = {:?}",
//                     time_a, time_c, contact_c
//                 );
//             } else {
//                 [b_transform_a, o_transform_a] = [b_transform_b, o_transform_b];
//                 time_a = time_b;
//                 debug!(
//                     "\nta = {}, tc = {}, contact = {:?}",
//                     time_a, time_c, contact_c
//                 );
//             }
//         }

//         Some((contact_c, time_c, transform1_c, transform2_c))
//     } else {
//         None
//     }
// }

// pub fn transform_at_time(
//     b_transform: Transform,
//     bp_transform: Transform,
//     time: f32,
//     b_velocity: Velocity,
//     b_angular_velocity: AngularVelocity,
// ) -> Transform {
//     Transform {
//         translation: transform.translation + velocity.0 * time,
//         rotation: transform.rotation * Quat::from_axis_angle(Vec3::Z, angular_velocity.0 * time),
//         scale: transform.scale,
//     }
// }
