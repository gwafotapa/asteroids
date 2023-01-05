use bevy::prelude::*;
// use iyes_loopless::prelude::*;

use crate::{
    asteroid::Asteroid,
    boss::{Boss, BossCore, BossEdge},
    fire::{Enemy, Fire},
    spaceship::Spaceship,
    AngularVelocity, Health, Mass, MomentOfInertia, Velocity,
};

use super::{
    cache::{Cache, Collision},
    detection::{self, Collider},
    response,
};

pub fn with_fire(
    meshes: Res<Assets<Mesh>>,
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
    query_boss: Query<(&Boss, &Transform)>,
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
) {
    let (boss, b_transform) = query_boss.single();
    for (bp_edge, bp_collider, bp_color, mut bp_health, bp_transform) in query_boss_part.iter_mut()
    {
        let bp_global_transform =
            Transform::from_translation(b_transform.transform_point(bp_transform.translation))
                .with_rotation(b_transform.rotation * bp_transform.rotation);
        for (f_collider, f_color, f_transform, mut f_health) in query_fire.iter_mut() {
            if detection::collision(
                *f_transform,
                bp_global_transform,
                f_collider,
                bp_collider,
                Some(Res::clone(&meshes)),
            )
            .is_some()
            {
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
            &Transform,
            &mut Velocity,
        ),
        With<Boss>,
    >,
    mut query_boss_part: Query<(&Collider, &Transform), Or<(With<BossCore>, With<BossEdge>)>>,
    mut query_asteroid_spaceship: Query<
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
        (Or<(With<Asteroid>, With<Spaceship>)>, Without<Boss>),
    >,
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
                if let Some(contact) = detection::collision(
                    *as_transform,
                    bp_global_transform,
                    &as_collider,
                    &bp_collider,
                    Some(Res::clone(&meshes)),
                ) {
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

                    if !cache.contains(Collision(spaceship, b_id)) {
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
                        // println!("spaceship -- w'1: {}", as_angular_velocity.0);
                        // println!("boss      -- w'2: {}", b_angular_velocity.0);
                        // println!("");
                    }
                    cache.add(Collision(spaceship, b_id));
                    // as_health.0 = 0;
                    return;
                }
            }
        }
    }
}
