use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::Mesh2dHandle};

use crate::{
    asteroid::Asteroid,
    boss::{BossCore, BossEdge},
    fire::Fire,
    spaceship::Spaceship,
    Enemy, Health,
};

use impact::Impact;

pub mod impact;
pub mod math;

pub type Triangle = [Vec3; 3];

// #[derive(Clone, Component, Copy)]
// pub enum Topology {
//     Point,
//     Circle,
//     Triangles,
// }

// #[derive(Component, Clone)]
// pub struct Surface {
//     pub topology: Topology,
//     pub hitbox: HitBox,
// }

#[derive(Clone, Component, Copy)]
pub struct HitBox {
    pub half_x: f32,
    pub half_y: f32,
}

// fn collision(
//     transform1: &GlobalTransform,
//     surface1: &Surface,
//     transform2: &GlobalTransform,
//     surface2: &Surface,
// ) -> bool {
//     match (
//         transform1,
//         surface1.topology,
//         surface1.hitbox,
//         transform2,
//         surface2.topology,
//         surface2.hitbox,
//     ) {
//         (_, Topology::Point, _, _, Topology::Point, _) => {
//             transform1.translation() == transform2.translation()
//         }
//         (
//             circle1,
//             Topology::Circle(radius1),
//             hitbox1,
//             circle2,
//             Topology::Circle(radius2),
//             hitbox2,
//         ) => {
//             if !rectangles_intersect(
//                 circle1.translation().truncate(),
//                 hitbox1,
//                 circle2.translation().truncate(),
//                 hitbox2,
//             ) {
//                 return false;
//             }

//             circle1.translation().distance(circle2.translation()) < radius1 + radius2
//         }
//         (
//             transform1,
//             Topology::Triangles(_triangles1),
//             hitbox1,
//             transform2,
//             Topology::Triangles(_triangles2),
//             hitbox2,
//         ) => {
//             if !rectangles_intersect(
//                 transform1.translation().truncate(),
//                 hitbox1,
//                 transform2.translation().truncate(),
//                 hitbox2,
//             ) {
//                 return false;
//             }

//             // for &[a1, b1, c1] in triangles1.iter() {
//             //     for &[a2, b2, c2] in triangles2.iter() {
//             //     if point_in_triangle(
//             //         transform1.transform_point(a1).truncate(),
//             //         transform2.transform_point(a2).truncate(),
//             //         transform2.transform_point(b2).truncate(),
//             //         transform2.transform_point(c2).truncate(),
//             //     ) || point_in_triangle(
//             //         transform1.transform_point(b1).truncate(),
//             //         transform2.transform_point(a2).truncate(),
//             //         transform2.transform_point(b2).truncate(),
//             //         transform2.transform_point(c2).truncate(),
//             //     ) || point_in_triangle(
//             //         transform1.transform_point(c1).truncate(),
//             //         transform2.transform_point(a2).truncate(),
//             //         transform2.transform_point(b2).truncate(),
//             //         transform2.transform_point(c2).truncate(),
//             //     ) {
//             //         return true;
//             //     }
//             // }
//             // }
//             unimplemented!();
//         }
//         (point, Topology::Point, _, circle, Topology::Circle(radius), hitbox)
//         | (circle, Topology::Circle(radius), hitbox, point, Topology::Point, _) => {
//             if !point_in_rectangle(
//                 point.translation().truncate(),
//                 circle.translation().truncate(),
//                 hitbox.half_x,
//                 hitbox.half_y,
//             ) {
//                 return false;
//             }

//             point.translation().distance(circle.translation()) < radius
//         }
//         (point, Topology::Point, _, triangles, Topology::Triangles(triangles_list), hitbox)
//         | (triangles, Topology::Triangles(triangles_list), hitbox, point, Topology::Point, _) => {
//             if !point_in_rectangle(
//                 point.translation().truncate(),
//                 triangles.translation().truncate(),
//                 hitbox.half_x,
//                 hitbox.half_y,
//             ) {
//                 return false;
//             }

//             for &[a, b, c] in triangles_list.iter() {
//                 if point_in_triangle(
//                     triangles
//                         .to_scale_rotation_translation()
//                         .1
//                         .inverse()
//                         .mul_vec3(point.translation() - triangles.translation())
//                         .truncate(),
//                     a.truncate(),
//                     b.truncate(),
//                     c.truncate(),
//                 ) {
//                     return true;
//                 }
//             }

//             false
//         }
//         (
//             circle_transform,
//             Topology::Circle(radius),
//             circle_hitbox,
//             triangles_transform,
//             Topology::Triangles(triangles),
//             triangles_hitbox,
//         )
//         | (
//             triangles_transform,
//             Topology::Triangles(triangles),
//             triangles_hitbox,
//             circle_transform,
//             Topology::Circle(radius),
//             circle_hitbox,
//         ) => {
//             if !rectangles_intersect(
//                 circle_transform.translation().truncate(),
//                 circle_hitbox,
//                 triangles_transform.translation().truncate(),
//                 triangles_hitbox,
//             ) {
//                 return false;
//             }

//             for triangle in triangles {
//                 if circle_intersects_triangle(
//                     triangles_transform
//                         .to_scale_rotation_translation()
//                         .1
//                         .inverse()
//                         .mul_vec3(
//                             circle_transform.translation() - triangles_transform.translation(),
//                         )
//                         .truncate(),
//                     radius,
//                     triangle[0].truncate(),
//                     triangle[1].truncate(),
//                     triangle[2].truncate(),
//                 ) {
//                     return true;
//                 }
//             }

//             false
//         }
//     }
// }

// pub fn collision(
//     t1: &GlobalTransform,
//     t2: &GlobalTransform,
//     h1: HitBox,
//     h2: HitBox,
//     topology1: Topology,
//     topology2: Topology,
//     triangles1: Option<Vec<[f32; 3]>>,
//     triangles2: Option<Vec<[f32; 3]>>,
// ) -> bool {
//     if !rectangles_intersect(
//         t1.translation().truncate(),
//         h1,
//         t2.translation().truncate(),
//         h2,
//     ) {
//         return false;
//     }

//     match (topology1, topology2) {
//         (Topology::Point, Topology::Point) => true,
//         (Topology::Point, Topology::Circle) | (Topology::Circle, Topology::Point) => {
//             t1.translation().distance(t2.translation()) < h1.half_x + h2.half_x
// 	}
//         (Topology::Point, Topology::Triangles) | (Topology::Triangles, Topology::Point) => {
//             unimplemented!()
//         }
//         (Topology::Circle, Topology::Circle) => {
//             t1.translation().distance(t2.translation()) < h1.half_x + h2.half_x
//         }
//         (Topology::Circle, Topology::Triangles) | (Topology::Triangles, Topology::Circle) => false,
//         (Topology::Triangles, Topology::Triangles) => false,
//     }
// }

pub fn spaceship_and_asteroid(
    meshes: Res<Assets<Mesh>>,
    mut query_spaceship: Query<(&Transform, &mut Health, &HitBox, &Mesh2dHandle), With<Spaceship>>,
    query_asteroid: Query<(&Asteroid, &GlobalTransform, &HitBox)>,
) {
    if let Ok((s_transform, mut s_health, s_hitbox, s_mesh)) = query_spaceship.get_single_mut() {
        if let Some(VertexAttributeValues::Float32x3(s_vertices)) = meshes
            .get(&s_mesh.0)
            .unwrap()
            .attribute(Mesh::ATTRIBUTE_POSITION)
        {
            for (asteroid, a_transform, a_hitbox) in query_asteroid.iter() {
                if math::collision_circle_triangles(
                    &a_transform.compute_transform(),
                    asteroid.radius,
                    *a_hitbox,
                    s_transform,
                    s_vertices,
                    *s_hitbox,
                ) {
                    s_health.0 = 0;
                    return;
                }
            }
        } else {
            panic!("Cannot find the spaceship's mesh to compute collision");
        }
    }
}

pub fn fire_and_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_fire: Query<(&Handle<ColorMaterial>, &Fire, &Transform, &mut Health)>,
    mut query_asteroid: Query<(&Asteroid, Entity, &GlobalTransform, &mut Health), Without<Fire>>,
) {
    for (f_color, fire, f_transform, mut f_health) in query_fire.iter_mut() {
        for (asteroid, _a_entity, a_transform, mut a_health) in query_asteroid.iter_mut() {
            if math::collision_point_circle(
                f_transform,
                &a_transform.compute_transform(),
                asteroid.radius,
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
                        transform: Transform::from_translation(
                            // f_transform.translation() - a_transform.translation(),
                            f_transform.translation,
                        ),
                        material: materials.add(color.into()),
                        ..default()
                    })
                    .id();

                // commands.entity(a_entity).add_child(impact);

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
            &Handle<ColorMaterial>,
            Entity,
            &Mesh2dHandle,
            &GlobalTransform,
            &mut Health,
            &HitBox,
        ),
        (Or<(With<BossEdge>, With<BossCore>)>, Without<Fire>),
    >,
) {
    // let (bc_children, bc_color, bc_entity, bc_transform, mut bc_health, bc_surface) =
    //     query_boss_core.single();
    // for (f_color, f_entity, fire, f_transform, mut f_health, f_surface, mut f_velocity) in
    //     query_fire.iter_mut()
    for (bp_core, bp_color, bp_entity, bp_mesh, bp_transform, mut bp_health, bp_hitbox) in
        query_boss_part.iter_mut()
    {
        if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
            .get(&bp_mesh.0)
            .unwrap()
            .attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let bp_transform = bp_transform.compute_transform();
            for (f_color, fire, f_transform, mut f_health) in query_fire.iter_mut() {
                if math::collision_point_triangles(f_transform, &bp_transform, vertices, *bp_hitbox)
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
                            material: materials.add(f_color.into()),
                            ..default()
                        })
                        .id();

                    commands.entity(bp_entity).add_child(impact);
                    // } else {
                    //     f_velocity.0 = -f_velocity.0;
                    //     commands.entity(f_entity).insert(Enemy);
                    // }
                    break;
                }
            }
        } else {
            panic!("Cannot find the boss's mesh to compute collision");
        }
    }
}

// pub fn fire_and_spaceship(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut query_fire: Query<
//         (
//             &Handle<ColorMaterial>,
//             &Fire,
//             &GlobalTransform,
//             &mut Health,
//             &Surface,
//         ),
//         (With<Enemy>, Without<Spaceship>),
//     >,
//     mut query_spaceship: Query<(Entity, &GlobalTransform, &mut Health, &Surface), With<Spaceship>>,
// ) {
//     if let Ok((s_entity, s_transform, mut s_health, s_surface)) = query_spaceship.get_single_mut() {
//         for (f_color, fire, f_transform, mut f_health, f_surface) in query_fire.iter_mut() {
//             if collision(f_transform, f_surface, s_transform, s_surface) {
//                 f_health.0 = 0;
//                 s_health.0 -= 1;
//                 let f_color = materials.get(f_color).unwrap().color;

//                 let impact = commands
//                     .spawn_empty()
//                     .insert(Impact)
//                     .insert(Health(10))
//                     .insert(ColorMesh2dBundle {
//                         mesh: meshes
//                             .add(Mesh::from(shape::Circle {
//                                 radius: fire.impact_radius,
//                                 vertices: fire.impact_vertices,
//                             }))
//                             .into(),
//                         transform: Transform::from_translation(
//                             f_transform.translation() - s_transform.translation(),
//                         ),
//                         // transform: Transform::from_translation(f_transform.translation()),
//                         material: materials.add(f_color.into()),
//                         ..default()
//                     })
//                     .id();

//                 commands.entity(s_entity).add_child(impact);
//             }
//         }
//     }
// }

// pub fn asteroid_and_asteroid(
//     mut query: Query<(&Asteroid, &GlobalTransform, &mut Health, &Surface)>,
// ) {
//     let mut i = 0;
//     loop {
//         let mut iter = query.iter_mut().skip(i);
//         if let Some((asteroid1, transform1, mut health1, surface1)) = iter.next() {
//             for (asteroid2, transform2, mut health2, surface2) in iter {
//                 if collision(transform1, surface1, transform2, surface2) {
//                     if asteroid1.radius < asteroid2.radius {
//                         health1.0 = 0;
//                         break;
//                     } else {
//                         health2.0 = 0;
//                     }
//                 }
//             }
//             i += 1;
//         } else {
//             break;
//         }
//     }
// }

pub fn spaceship_and_boss(
    meshes: Res<Assets<Mesh>>,
    mut query_spaceship: Query<(&mut Health, &HitBox, &Mesh2dHandle, &Transform), With<Spaceship>>,
    query_boss: Query<
        (&GlobalTransform, &HitBox, &Mesh2dHandle),
        Or<(With<BossCore>, With<BossEdge>)>,
    >,
) {
    if let Ok((mut s_health, s_hitbox, s_mesh, s_transform)) = query_spaceship.get_single_mut() {
        if let Some(VertexAttributeValues::Float32x3(s_vertices)) = meshes
            .get(&s_mesh.0)
            .unwrap()
            .attribute(Mesh::ATTRIBUTE_POSITION)
        {
            for (b_transform, b_hitbox, b_mesh) in query_boss.iter() {
                if let Some(VertexAttributeValues::Float32x3(b_vertices)) = meshes
                    .get(&b_mesh.0)
                    .unwrap()
                    .attribute(Mesh::ATTRIBUTE_POSITION)
                {
                    if math::collision_triangles_triangles(
                        s_transform,
                        s_vertices,
                        *s_hitbox,
                        &b_transform.compute_transform(),
                        b_vertices,
                        *b_hitbox,
                    ) {
                        s_health.0 = 0;
                        return;
                    }
                }
            }
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
