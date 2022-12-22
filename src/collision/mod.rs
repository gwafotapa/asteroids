use bevy::prelude::*;

use crate::{
    asteroid::Asteroid,
    boss::{BossCore, BossEdge},
    fire::{Enemy, Fire},
    spaceship::Spaceship,
    Health, Mass, Velocity,
};

use impact::Impact;
pub use math::{Aabb, Collider, Topology};

pub mod aftermath;
pub mod impact;
pub mod math;

pub fn spaceship_and_asteroid(
    meshes: Res<Assets<Mesh>>,
    mut query_spaceship: Query<
        (&mut Collider, &mut Health, &Mass, &Transform, &mut Velocity),
        With<Spaceship>,
    >,
    mut query_asteroid: Query<
        (&mut Collider, &GlobalTransform, &Mass, &mut Velocity),
        (With<Asteroid>, Without<Spaceship>),
    >,
) {
    if let Ok((mut s_collider, mut _s_health, s_mass, s_transform, mut s_velocity)) =
        query_spaceship.get_single_mut()
    {
        for (mut a_collider, a_transform, a_mass, mut a_velocity) in query_asteroid.iter_mut() {
            if math::collision(
                a_transform.compute_transform(),
                *s_transform,
                &a_collider,
                &s_collider,
                Some(&meshes),
            ) {
                if !a_collider.last || !s_collider.last {
                    aftermath::compute(
                        &mut a_velocity,
                        &mut s_velocity,
                        &a_transform.compute_transform(),
                        s_transform,
                        *a_mass,
                        *s_mass,
                    );
                }
                a_collider.now = true;
                s_collider.now = true;
                return;
            }
        }
    }
}

// pub fn awaken(mut query: Query<&mut Collider>) {
//     for mut collider in &mut query {
//         if collider.sleep > 0 {
//             collider.sleep -= 1;
//         }
//     }
// }

pub fn update(mut query: Query<&mut Collider>) {
    for mut collider in &mut query {
        collider.last = collider.now;
        collider.now = false;
    }
}

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
    mut query_spaceship: Query<
        (&mut Collider, &mut Health, &Mass, &Transform, &mut Velocity),
        With<Spaceship>,
    >,
    mut query_boss_edge: Query<
        (&mut Collider, &GlobalTransform),
        (With<BossEdge>, Without<Spaceship>),
    >,
    mut query_boss_core: Query<
        (&mut Collider, &Mass, &Transform, &mut Velocity),
        (With<BossCore>, Without<BossEdge>, Without<Spaceship>),
    >,
) {
    if let Ok((mut s_collider, mut _s_health, s_mass, s_transform, mut s_velocity)) =
        query_spaceship.get_single_mut()
    {
        if let Ok((mut bc_collider, bc_mass, bc_transform, mut bc_velocity)) =
            query_boss_core.get_single_mut()
        {
            for (mut be_collider, be_transform) in query_boss_edge.iter_mut() {
                if math::collision(
                    *s_transform,
                    be_transform.compute_transform(),
                    &s_collider,
                    &be_collider,
                    Some(&meshes),
                ) {
                    s_collider.now = true;
                    be_collider.now = true;
                    if !s_collider.last || !be_collider.last {
                        aftermath::compute(
                            &mut s_velocity,
                            &mut bc_velocity,
                            s_transform,
                            &be_transform.compute_transform(),
                            *s_mass,
                            *bc_mass,
                        );
                    }
                    // s_health.0 = 0;
                    return;
                }
            }
            if math::collision(
                *s_transform,
                *bc_transform,
                &s_collider,
                &bc_collider,
                Some(&meshes),
            ) {
                s_collider.now = true;
                bc_collider.now = true;
                if !s_collider.last || !bc_collider.last {
                    aftermath::compute(
                        &mut s_velocity,
                        &mut bc_velocity,
                        s_transform,
                        bc_transform,
                        *s_mass,
                        *bc_mass,
                    );
                }
                // s_health.0 = 0;
                return;
            }
        }
    }
}

pub fn boss_and_asteroid(
    meshes: Res<Assets<Mesh>>,
    mut query_asteroid: Query<
        (&mut Collider, &mut Health, &Mass, &Transform, &mut Velocity),
        With<Asteroid>,
    >,
    mut query_boss_edge: Query<
        (&mut Collider, &GlobalTransform),
        (With<BossEdge>, Without<Asteroid>),
    >,
    mut query_boss_core: Query<
        (&mut Collider, &Mass, &Transform, &mut Velocity),
        (With<BossCore>, Without<BossEdge>, Without<Asteroid>),
    >,
) {
    if let Ok((mut bc_collider, bc_mass, bc_transform, mut bc_velocity)) =
        query_boss_core.get_single_mut()
    {
        for (mut a_collider, mut _a_health, a_mass, a_transform, mut a_velocity) in
            query_asteroid.iter_mut()
        {
            for (mut be_collider, be_transform) in query_boss_edge.iter_mut() {
                if math::collision(
                    *a_transform,
                    be_transform.compute_transform(),
                    &a_collider,
                    &be_collider,
                    Some(&meshes),
                ) {
                    a_collider.now = true;
                    be_collider.now = true;
                    if !a_collider.last || !be_collider.last {
                        aftermath::compute(
                            &mut a_velocity,
                            &mut bc_velocity,
                            a_transform,
                            &be_transform.compute_transform(),
                            *a_mass,
                            *bc_mass,
                        );
                    }
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
                a_collider.now = true;
                bc_collider.now = true;
                if !a_collider.last || !bc_collider.last {
                    aftermath::compute(
                        &mut a_velocity,
                        &mut bc_velocity,
                        a_transform,
                        bc_transform,
                        *a_mass,
                        *bc_mass,
                    );
                }
                // a_health.0 = 0;
                return;
            }
        }
    }
}

pub fn asteroid_and_asteroid(
    mut query: Query<
        (&mut Collider, &mut Health, &Mass, &Transform, &mut Velocity),
        With<Asteroid>,
    >,
) {
    let mut i = 0;
    loop {
        let mut iter = query.iter_mut().skip(i);
        if let Some((mut collider1, mut _health1, mass1, transform1, mut velocity1)) = iter.next() {
            for (mut collider2, mut _health2, mass2, transform2, mut velocity2) in iter {
                if math::collision(*transform1, *transform2, &collider1, &collider2, None) {
                    if !collider1.last || !collider2.last {
                        aftermath::compute(
                            &mut velocity1,
                            &mut velocity2,
                            transform1,
                            transform2,
                            *mass1,
                            *mass2,
                        );
                    }
                    collider1.now = true;
                    collider2.now = true;
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
