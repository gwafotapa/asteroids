use bevy::prelude::*;

use crate::{
    asteroid::Asteroid,
    boss::{BossCore, BossEdge},
    fire::{Enemy, Fire},
    spaceship::Spaceship,
    AngularVelocity, Health, Mass, Velocity,
};

use cache::{Cache, Collision};
use impact::Impact;
pub use math::{Aabb, Collider, Topology};

pub mod cache;
pub mod impact;
pub mod math;
pub mod response;

pub fn spaceship_and_asteroid(
    meshes: Res<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query_spaceship: Query<
        (
            &mut AngularVelocity,
            &Collider,
            Entity,
            &mut Health,
            &Mass,
            &Transform,
            &mut Velocity,
        ),
        With<Spaceship>,
    >,
    mut query_asteroid: Query<
        (&Collider, Entity, &Transform, &Mass, &mut Velocity),
        (With<Asteroid>, Without<Spaceship>),
    >,
) {
    if let Ok((
        mut s_angular_velocity,
        s_collider,
        s_entity,
        mut _s_health,
        s_mass,
        s_transform,
        mut s_velocity,
    )) = query_spaceship.get_single_mut()
    {
        for (a_collider, a_entity, a_transform, a_mass, mut a_velocity) in query_asteroid.iter_mut()
        {
            if math::collision(
                *a_transform,
                *s_transform,
                &a_collider,
                &s_collider,
                Some(&meshes),
            ) {
                // println!(
                //     "{}",
                //     s_mass.0 * s_velocity.0.length() + a_mass.0 * a_velocity.0.length()
                // );
                // let tmp_a_velocity = *a_velocity;
                // let tmp_s_velocity = *s_velocity;
                // a_collider.now = true;
                // s_collider.now = true;
                println!("Collision");
                if !cache.contains(Collision(a_entity, s_entity)) {
                    response::compute(
                        a_transform,
                        s_transform,
                        *a_mass,
                        *s_mass,
                        &mut a_velocity, // &mut Velocity(Vec3::ZERO),
                        &mut s_velocity,
                        Some(&mut s_angular_velocity),
                        None,
                    );
                }
                cache.add(Collision(a_entity, s_entity));
                // println!(
                //     "{}\nSpaceship [vx: {:.2}, vy: {:.2}] / [vx: {:.2}, vy: {:.2}]\nAsteroid [vx: {:.2}, vy: {:.2}] / [vx: {:.2}, vy: {:.2}]\n",
                //     s_collider.last,
                //     tmp_s_velocity.0.x,
                //     tmp_s_velocity.0.y,
                //     s_velocity.0.x,
                //     s_velocity.0.y,
                //     tmp_a_velocity.0.x,
                //     tmp_a_velocity.0.y,
                //     a_velocity.0.x,
                //     a_velocity.0.y
                // );
                return;
            }
        }
    }
}

// pub fn update(mut query: Query<&mut Collider>) {
//     for mut collider in &mut query {
//         collider.last = collider.now;
//         collider.now = false;
//     }
// }

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
            // if math::collision_point_circle(
            //     f_transform,
            //     &a_transform.compute_transform(),
            //     asteroid.radius,
            if math::collision(
                *f_transform,
                a_transform.compute_transform(),
                f_collider,
                a_collider,
                None,
            ) {
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
            // if math::collision_point_triangles(
            //     f_transform,
            //     &bp_transform,
            //     vertices,
            //     bp_collider.aabb,
            if math::collision(
                *f_transform,
                bp_transform,
                f_collider,
                bp_collider,
                Some(&meshes),
            ) {
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
            // if math::collision_point_triangles(
            //     f_transform,
            //     s_transform,
            //     vertices,
            //     s_collider.aabb,
            if math::collision(
                *f_transform,
                *s_transform,
                f_collider,
                s_collider,
                Some(&meshes),
            ) {
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
    meshes: Res<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query_spaceship: Query<
        (
            &Collider,
            Entity,
            &mut Health,
            &Mass,
            &Transform,
            &mut Velocity,
        ),
        With<Spaceship>,
    >,
    mut query_boss_edge: Query<
        (&Collider, Entity, &Transform),
        (With<BossEdge>, Without<Spaceship>),
    >,
    mut query_boss_core: Query<
        (&Collider, Entity, &Mass, &Transform, &mut Velocity),
        (With<BossCore>, Without<BossEdge>, Without<Spaceship>),
    >,
) {
    if let Ok((s_collider, spaceship, mut _s_health, s_mass, s_transform, mut s_velocity)) =
        query_spaceship.get_single_mut()
    {
        if let Ok((bc_collider, boss_core, bc_mass, bc_transform, mut bc_velocity)) =
            query_boss_core.get_single_mut()
        {
            if math::collision(
                *s_transform,
                *bc_transform,
                &s_collider,
                &bc_collider,
                Some(&meshes),
            ) {
                if !cache.contains(Collision(spaceship, boss_core)) {
                    response::compute(
                        s_transform,
                        bc_transform,
                        *s_mass,
                        *bc_mass,
                        &mut s_velocity,
                        &mut bc_velocity,
                        None,
                        None,
                    );
                }
                cache.add(Collision(spaceship, boss_core));
                // s_health.0 = 0;
                return;
            }
            for (be_collider, boss_edge, be_transform) in query_boss_edge.iter_mut() {
                let be_global_transform = Transform::from_translation(
                    bc_transform.transform_point(be_transform.translation),
                );
                if math::collision(
                    *s_transform,
                    be_global_transform,
                    &s_collider,
                    &be_collider,
                    Some(&meshes),
                ) {
                    if !cache.contains(Collision(spaceship, boss_edge)) {
                        response::compute(
                            s_transform,
                            &be_global_transform,
                            *s_mass,
                            *bc_mass,
                            &mut s_velocity,
                            &mut bc_velocity,
                            None,
                            None,
                        );
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
            &Collider,
            Entity,
            &mut Health,
            &Mass,
            &Transform,
            &mut Velocity,
        ),
        With<Asteroid>,
    >,
    mut query_boss_edge: Query<
        (&Collider, Entity, &GlobalTransform),
        (With<BossEdge>, Without<Asteroid>),
    >,
    mut query_boss_core: Query<
        (&Collider, Entity, &Mass, &Transform, &mut Velocity),
        (With<BossCore>, Without<BossEdge>, Without<Asteroid>),
    >,
) {
    if let Ok((bc_collider, boss_core, bc_mass, bc_transform, mut bc_velocity)) =
        query_boss_core.get_single_mut()
    {
        for (a_collider, asteroid, mut _a_health, a_mass, a_transform, mut a_velocity) in
            query_asteroid.iter_mut()
        {
            for (be_collider, boss_edge, be_transform) in query_boss_edge.iter_mut() {
                if math::collision(
                    *a_transform,
                    be_transform.compute_transform(),
                    &a_collider,
                    &be_collider,
                    Some(&meshes),
                ) {
                    println!("Collision boss / asteroid");
                    if !cache.contains(Collision(asteroid, boss_edge)) {
                        response::compute(
                            a_transform,
                            &be_transform.compute_transform(),
                            *a_mass,
                            *bc_mass,
                            &mut a_velocity,
                            &mut bc_velocity,
                            None,
                            None,
                        );
                    }
                    cache.add(Collision(asteroid, boss_edge));
                    // a_health.0 = 0;
                    return;
                }
            }
            if math::collision(
                *a_transform,
                *bc_transform,
                &a_collider,
                &bc_collider,
                Some(&meshes),
            ) {
                if !cache.contains(Collision(asteroid, boss_core)) {
                    response::compute(
                        a_transform,
                        bc_transform,
                        *a_mass,
                        *bc_mass,
                        &mut a_velocity,
                        &mut bc_velocity,
                        None,
                        None,
                    );
                }
                cache.add(Collision(asteroid, boss_core));
                // a_health.0 = 0;
                return;
            }
        }
    }
}

pub fn asteroid_and_asteroid(
    mut cache: ResMut<Cache>,
    mut query: Query<
        (
            &Collider,
            Entity,
            &mut Health,
            &Mass,
            &Transform,
            &mut Velocity,
        ),
        With<Asteroid>,
    >,
) {
    let mut i = 0;
    loop {
        let mut iter = query.iter_mut().skip(i);
        if let Some((collider1, asteroid1, mut _health1, mass1, transform1, mut velocity1)) =
            iter.next()
        {
            for (collider2, asteroid2, mut _health2, mass2, transform2, mut velocity2) in iter {
                if math::collision(*transform1, *transform2, &collider1, &collider2, None) {
                    if !cache.contains(Collision(asteroid1, asteroid2)) {
                        response::compute(
                            transform1,
                            transform2,
                            *mass1,
                            *mass2,
                            &mut velocity1,
                            &mut velocity2,
                            None,
                            None,
                        );
                    }
                    cache.add(Collision(asteroid1, asteroid2));
                    break;
                }
            }
            i += 1;
        } else {
            break;
        }
    }
}

pub fn asteroids_and_spaceship(
    meshes: Res<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    mut query: Query<
        (
            Option<&mut AngularVelocity>,
            &Collider,
            Entity,
            &mut Health,
            &Mass,
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
                transform2,
                mut velocity2,
            ) in iter
            {
                if math::collision(
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
                    if !cache.contains(Collision(entity1, entity2)) {
                        response::compute(
                            transform1,
                            transform2,
                            *mass1,
                            *mass2,
                            &mut velocity1,
                            &mut velocity2,
                            angular_velocity1.as_deref_mut(),
                            angular_velocity2.as_deref_mut(),
                        );
                    }
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
