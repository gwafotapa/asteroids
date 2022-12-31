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

pub mod cache;
pub mod detection;
pub mod impact;
pub mod response;

pub fn boss_and_fire(
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_fire: Query<
        (
            &Collider,
            &Handle<ColorMaterial>,
            &Transform,
            &mut Health,
            // &mut Velocity,
        ),
        (With<Fire>, Without<Enemy>),
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
            &GlobalTransform,
            &mut Health,
        ),
        (Or<(With<BossCore>, With<BossEdge>)>, Without<Fire>),
    >,
) {
    for (bp_core, bp_collider, bp_color, bp_transform, mut bp_health) in query_boss_part.iter_mut()
    {
        let bp_transform = bp_transform.compute_transform();
        for (f_collider, f_color, f_transform, mut f_health) in query_fire.iter_mut() {
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
            }
        }
    }
}

pub fn boss_and_asteroid_or_spaceship(
    // mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
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
        With<BossCore>,
    >,
    mut query_boss_edge: Query<(&Collider, Entity, &Transform), With<BossEdge>>,
    mut query_asteroid_spaceship: Query<
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
        (Without<BossCore>, Or<(With<Asteroid>, With<Spaceship>)>),
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
            if let Some(contact) = detection::collision(
                *as_transform,
                *bc_transform,
                &as_collider,
                &bc_collider,
                Some(&meshes),
            ) {
                if !cache.contains(Collision(spaceship, boss_core)) {
                    response::compute(
                        &mut as_transform,
                        &mut bc_transform,
                        *as_mass,
                        *bc_mass,
                        *as_moment_of_inertia,
                        *bc_moment_of_inertia,
                        &mut as_velocity,
                        &mut bc_velocity,
                        &mut as_angular_velocity,
                        &mut bc_angular_velocity,
                        contact,
                    );
                }
                cache.add(Collision(spaceship, boss_core));
                // as_health.0 = 0;
                return;
            }
            for (be_collider, boss_edge, be_transform) in query_boss_edge.iter_mut() {
                let be_global_transform = Transform::from_translation(
                    bc_transform.transform_point(be_transform.translation),
                )
                .with_rotation(bc_transform.rotation * be_transform.rotation);
                if let Some(contact) = detection::collision(
                    *as_transform,
                    be_global_transform,
                    &as_collider,
                    &be_collider,
                    Some(&meshes),
                ) {
                    // TODO
                    // contact.normal = (as_transform.translation - bc_transform.translation)
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
                        // println!("spaceship -- w1: {}", as_angular_velocity.0);
                        // println!("boss      -- w2: {}", bc_angular_velocity.0);
                        response::compute(
                            &mut as_transform,
                            &mut bc_transform,
                            *as_mass,
                            *bc_mass,
                            *as_moment_of_inertia,
                            *bc_moment_of_inertia,
                            &mut as_velocity,
                            &mut bc_velocity,
                            &mut as_angular_velocity,
                            &mut bc_angular_velocity,
                            contact,
                        );
                        // println!("spaceship -- w'1: {}", as_angular_velocity.0);
                        // println!("boss      -- w'2: {}", bc_angular_velocity.0);
                        // println!("");
                    }
                    cache.add(Collision(spaceship, boss_edge));
                    // as_health.0 = 0;
                    return;
                }
            }
        }
    }
}

pub fn asteroid_fire_spaceship(
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
        Or<(With<Asteroid>, With<Fire>, With<Spaceship>)>,
    >,
) {
    let mut i = 0;
    loop {
        let mut iter = query.iter_mut().skip(i);
        if let Some((
            mut angular_velocity1,
            collider1,
            entity1,
            mut health1,
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
                mut health2,
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
                    //     "m1(v1 - v2): {}\n",
                    //     (mass1.0 * velocity1.0 - mass2.0 * velocity2.0).length()
                    // );

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
                    // println!(
                    //     "normal: {}\nw1: {}\nw2: {}",
                    //     contact.normal, angular_velocity1.0, angular_velocity2.0
                    // );
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
                        println!("health1: {}, h1: {}", health1.0, h1);
                        println!("health2: {}, h2: {}", health2.0, h2);
                        health1.0 -= h1;
                        health2.0 -= h2;
                        // println!("m1: {}, v1: {}", mass1.0, velocity1.0);
                        // println!("m2: {}, v2: {}", mass2.0, velocity2.0);
                        // println!("dv = v1 - v2: {}", dv);
                        // println!("m2/m1 * dv: {}, m1/m2 * dv: {}", h1, h2);
                    }
                    // println!(
                    //     "w'1: {}\nw'2: {}\n",
                    //     angular_velocity1.0, angular_velocity2.0
                    // );
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
