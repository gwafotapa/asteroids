use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
// use std::time::Instant;
// use std::f32::consts::SQRT_2;

pub mod boss;
pub mod spaceship;

use spaceship::Spaceship;

pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
const INITIAL_COUNT_OF_STARS_BY_VELOCITY: usize = 10;
const MAX_SPEED_OF_STARS: usize = 10;
const MAX_SPEED_OF_ASTEROIDS: usize = 5;
const MAX_HEALTH_OF_ASTEROIDS: usize = 6;
const ALTITUDE: f32 = 100.0;
const INITIAL_DISTANCE_TO_BOSS: usize = 0;

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct Asteroid {
    radius: f32,
}

#[derive(Component)]
pub struct Fire;

#[derive(Component)]
pub struct Debris;

#[derive(Component)]
pub struct Impact;

#[derive(Component)]
pub struct Blast;

// #[derive(Component)]
// struct SpawnedTime(Instant);

#[derive(Component)]
pub struct Level {
    distance_to_boss: usize,
    boss_spawned: bool,
}

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
struct BossPart;

#[derive(Component)]
pub struct Health(usize);

#[derive(Component, Clone, Copy)]
pub struct RectangularEnvelop {
    pub half_x: f32,
    pub half_y: f32,
}

#[derive(Component)]
pub struct Attack {
    source: Vec3,
    color: Color,
    blast_radius: f32,
    blast_vertices: usize,
    fire_radius: f32,
    fire_vertices: usize,
}

pub fn camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                // format!("Distance: {:12}", INITIAL_DISTANCE),
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::GRAY,
                },
            )
            // Set the alignment of the Text
            // .with_text_alignment(TextAlignment::TOP_RIGHT)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                // align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.0),
                    left: Val::Px(WINDOW_WIDTH - 150.0),
                    // bottom: Val::Px(10.0),
                    // right: Val::Px(1000.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Level {
            distance_to_boss: INITIAL_DISTANCE_TO_BOSS,
            boss_spawned: false,
        });
}

pub fn update_distance_to_boss(mut query: Query<(&mut Text, &mut Level)>) {
    for (mut text, mut level) in &mut query {
        if level.distance_to_boss > 0 {
            level.distance_to_boss -= 1;
        }
        text.sections[0].value = format!("Distance: {:12}", level.distance_to_boss);
    }
}

pub fn setup_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for speed in 1..(MAX_SPEED_OF_STARS + 1) {
        let z = ALTITUDE - (MAX_SPEED_OF_STARS / 2 + speed) as f32 + 0.5;
        for _i in 0..INITIAL_COUNT_OF_STARS_BY_VELOCITY {
            let x = rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0);
            let y = rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0);

            commands
                .spawn()
                .insert(Star)
                .insert(Velocity(Vec3::from([-(speed as f32), 0., 0.])))
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle {
                            radius: 1.0,
                            vertices: 4,
                        }))
                        .into(),
                    transform: Transform::from_translation(Vec3 { x, y, z }),
                    material: materials.add(Color::rgb(1., 1., 1.).into()),
                    ..default()
                });
        }
    }
}

pub fn add_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let speed = rng.gen_range(1..MAX_SPEED_OF_STARS + 1) as f32;
    let velocity = Vec3::from([-speed, 0., 0.]);

    let y = rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0);
    let z = ALTITUDE - (MAX_SPEED_OF_STARS / 2) as f32 + speed + 0.5;

    commands
        .spawn()
        .insert(Star)
        .insert(Velocity(velocity))
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle {
                    radius: 1.0,
                    vertices: 4,
                }))
                .into(),
            transform: Transform::from_translation(Vec3 {
                x: WINDOW_WIDTH / 2.0,
                y,
                z,
            }),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            ..default()
        });
}

pub fn update_stars(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Transform, &Velocity, Entity), With<Star>>,
) {
    for (mut transform, velocity, star) in query.iter_mut() {
        transform.translation += velocity.0;
        //     for value in mesh.attributes() {
        //         println!("{}", value);
        //     }
        if transform.translation.x < -WINDOW_WIDTH / 2.0 {
            commands.entity(star).despawn();
        }
    }
    // for mesh in meshes.get_handle() {}
}

// /// Print the up-to-date global coordinates of the player as of **this frame**.
// pub fn debug_globaltransform(query: Query<&GlobalTransform, With<Star>>) {
//     for mesh in query.iter() {
//         debug!("Mesh at: {:?}", mesh.translation());
//     }
// }

pub fn keyboard_input(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &Attack), With<Spaceship>>,
) {
    // if keys.just_pressed(KeyCode::Space) {
    //     // Space was pressed
    // }

    // if keys.just_released(KeyCode::LControl) {
    //     // Left Ctrl was released
    // }

    if let Ok((spaceship, mut transform, mut velocity, attack)) = query.get_single_mut() {
        // // we can check multiple at once with `.any_*`
        // if keys.any_pressed([
        //     KeyCode::Left,
        //     KeyCode::Down,
        //     KeyCode::Up,
        //     KeyCode::Right,
        //     KeyCode::H,
        //     KeyCode::J,
        //     KeyCode::K,
        //     KeyCode::L,
        // ]) {
        if keys.any_just_pressed([KeyCode::Space, KeyCode::R]) {
            spaceship::attack(commands, meshes, materials, spaceship, &transform, attack);
        }
        // Either the left or right shift are being held down
        if keys.any_pressed([KeyCode::H, KeyCode::Left]) {
            // W is being held down
            // transform.translation += Vec3::from([-spaceship.acceleration(), 0., 0.]);
            Spaceship::accelerate(&mut velocity, Direction::Left);
        } else if keys.any_pressed([KeyCode::L, KeyCode::Right]) {
            // W is being held down
            Spaceship::accelerate(&mut velocity, Direction::Right);
        } else {
            Spaceship::decelerate_x(&mut velocity);
        }

        if keys.any_pressed([KeyCode::J, KeyCode::Down]) {
            // W is being held down
            Spaceship::accelerate(&mut velocity, Direction::Down);
        } else if keys.any_pressed([KeyCode::K, KeyCode::Up]) {
            // W is being held down
            Spaceship::accelerate(&mut velocity, Direction::Up);
        } else {
            Spaceship::decelerate_y(&mut velocity);
        }
        // } else {
        //     Spaceship::decelerate();
        // }
        // if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        //     // Either delete or backspace was just pressed
        // }
        transform.translation += velocity.0;
    }
}

pub fn asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asteroid_query: Query<(&mut Transform, &Velocity, &Asteroid, Entity)>,
    level_query: Query<&Level>,
) {
    let mut rng = rand::thread_rng();

    if level_query.single().distance_to_boss > 0 && rng.gen_range(0..100) == 0 {
        let health = rng.gen_range(1..MAX_HEALTH_OF_ASTEROIDS + 1);
        let radius = (health * 20) as f32;
        let speed = rng.gen_range(1..MAX_SPEED_OF_ASTEROIDS + 1) as f32;
        let velocity = Vec3::from([-speed, 0., 0.]);
        let x = WINDOW_WIDTH / 2.0 + (MAX_HEALTH_OF_ASTEROIDS * 20) as f32;
        let y = rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0);

        commands
            .spawn()
            .insert(Asteroid { radius })
            .insert(Health(health))
            .insert(Velocity(velocity))
            .insert(RectangularEnvelop {
                half_x: radius,
                half_y: radius,
            })
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius,
                        vertices: 16,
                    }))
                    .into(),
                transform: Transform::from_xyz(x, y, ALTITUDE),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            });
    }

    for (mut transform, velocity, asteroid, entity) in asteroid_query.iter_mut() {
        transform.translation += velocity.0;
        if transform.translation.x < -WINDOW_WIDTH / 2.0 - asteroid.radius {
            commands.entity(entity).despawn();
        }
    }
}

pub fn detect_collision_spaceship_asteroid(
    mut commands: Commands,
    spaceship_query: Query<(Entity, &Transform, &Velocity, &RectangularEnvelop)>,
    asteroid_query: Query<(&Transform, &Asteroid, &RectangularEnvelop)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((s_entity, s_transform, s_velocity, s_rectangular_envelop)) =
        spaceship_query.get_single()
    {
        for (a_transform, asteroid, a_rectangular_envelop) in asteroid_query.iter() {
            if rectangles_intersect(
                s_transform.translation,
                *s_rectangular_envelop,
                a_transform.translation,
                *a_rectangular_envelop,
            ) {
                for point in spaceship::ENVELOP {
                    if a_transform
                        .translation
                        // .distance((point + s_transform.translation) * s_transform.scale.x)
                        .distance(point * s_transform.scale + s_transform.translation)
                        < asteroid.radius
                    {
                        commands.entity(s_entity).despawn();
                        let mut rng = rand::thread_rng();
                        for _ in 1..10 {
                            let debris_dx = rng.gen_range(-30.0..30.0);
                            let debris_x = s_transform.translation.x + debris_dx;
                            let debris_dy = rng.gen_range(-20.0..20.0);
                            let debris_y = s_transform.translation.y + debris_dy;

                            let velocity = Vec3 {
                                x: rng.gen_range(-0.5..0.5),
                                y: rng.gen_range(-0.5..0.5),
                                z: 0.0,
                            };

                            commands
                                .spawn()
                                .insert(Debris)
                                .insert(Velocity(s_velocity.0 + velocity))
                                .insert_bundle(MaterialMesh2dBundle {
                                    mesh: meshes
                                        .add(Mesh::from(shape::Circle {
                                            radius: 10.0,
                                            vertices: 4,
                                        }))
                                        .into(),
                                    transform: Transform::from_xyz(debris_x, debris_y, ALTITUDE)
                                        .with_scale(Vec3::splat(4.0)),
                                    material: materials.add(Color::BLUE.into()),
                                    ..default()
                                });
                        }

                        return;
                    }
                }
            }
        }
    }
}

pub fn update_bullets(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity), With<Fire>>,
) {
    for (mut transform, entity) in query.iter_mut() {
        transform.translation += Vec3 {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };
        if transform.translation.x > WINDOW_WIDTH / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn detect_collision_bullet_asteroid(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Fire>>,
    mut asteroid_query: Query<(
        Entity,
        &Transform,
        &Asteroid,
        &mut Health,
        &Velocity,
        &RectangularEnvelop,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (
            asteroid_entity,
            asteroid_transform,
            asteroid,
            mut asteroid_health,
            asteroid_velocity,
            asteroid_envelop,
        ) in asteroid_query.iter_mut()
        {
            if rectangles_intersect(
                bullet_transform.translation,
                RectangularEnvelop {
                    half_x: 0.0,
                    half_y: 0.0,
                },
                asteroid_transform.translation,
                *asteroid_envelop,
            ) {
                if bullet_transform
                    .translation
                    .distance(asteroid_transform.translation)
                    < asteroid.radius
                {
                    commands
                        .spawn()
                        .insert(Impact)
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: 4.0,
                                    vertices: 8,
                                }))
                                .into(),
                            transform: bullet_transform.clone().with_scale(Vec3::splat(5.0)),
                            material: materials.add(spaceship::ATTACK_COLOR.into()),
                            ..default()
                        });

                    commands.entity(bullet_entity).despawn();

                    asteroid_health.0 -= 1;
                    if asteroid_health.0 == 0 {
                        commands.entity(asteroid_entity).despawn();
                        let mut rng = rand::thread_rng();
                        for _ in 1..asteroid.radius as usize {
                            let debris_dx = rng.gen_range(-asteroid.radius..asteroid.radius);
                            let debris_x = asteroid_transform.translation.x + debris_dx;
                            let dy_max = (asteroid.radius.powi(2) - debris_dx.powi(2)).sqrt();
                            let debris_dy = rng.gen_range(-dy_max..dy_max);
                            let debris_y = asteroid_transform.translation.y + debris_dy;
                            // let z = rng.gen_range(
                            //     asteroid_transform.translation.z - asteroid.radius
                            //         ..asteroid_transform.translation.z + asteroid.radius,
                            // );

                            let velocity = Vec3 {
                                x: rng.gen_range(-0.5..0.5),
                                y: rng.gen_range(-0.5..0.5),
                                // z: rng.gen_range(-0.5..0.5),
                                z: 0.0,
                            };

                            commands
                                .spawn()
                                .insert(Debris)
                                .insert(Velocity(asteroid_velocity.0 + velocity))
                                // .insert(Velocity(asteroid_velocity.0 * 0.5))
                                .insert_bundle(MaterialMesh2dBundle {
                                    mesh: meshes
                                        .add(Mesh::from(shape::Circle {
                                            radius: rng.gen_range(
                                                asteroid.radius / 100.0..asteroid.radius / 20.0,
                                            ),
                                            vertices: 8,
                                        }))
                                        .into(),
                                    transform: Transform::from_xyz(
                                        debris_x,
                                        debris_y,
                                        ALTITUDE + if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
                                    )
                                    .with_scale(Vec3::splat(4.0)),
                                    material: materials.add(Color::PURPLE.into()),
                                    ..default()
                                });
                        }
                    }
                    break;
                }
            }
        }
    }
}

pub fn update_debris(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Velocity, Entity), With<Debris>>,
) {
    for (mut transform, velocity, debris) in query.iter_mut() {
        transform.translation += velocity.0;
        transform.scale -= 0.1;
        // if transform.translation.x < -WINDOW_WIDTH / 2.0
        //     || transform.translation.x > WINDOW_WIDTH / 2.0
        //     || transform.translation.y < -WINDOW_HEIGHT / 2.0
        //     || transform.translation.y > WINDOW_HEIGHT / 2.0
        if transform.scale.x < 0.05 {
            commands.entity(debris).despawn();
        }
    }
}

pub fn update_impacts(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity), With<Impact>>,
) {
    for (mut transform, impact) in query.iter_mut() {
        // transform.scale -= Vec3::ONE;
        transform.scale -= 0.5;
        // println!("{}", transform.scale);
        // if transform.scale == Vec3::ONE {
        if transform.scale.x < 0.25 {
            commands.entity(impact).despawn();
        }
    }
}

pub fn update_boss_bullets(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Velocity, Entity), With<boss::Fire>>,
) {
    for (mut transform, velocity, bullet) in query.iter_mut() {
        transform.translation += velocity.0;
        if transform.translation.x > WINDOW_WIDTH / 2.0
            || transform.translation.x < -WINDOW_WIDTH / 2.0
            || transform.translation.y > WINDOW_HEIGHT / 2.0
            || transform.translation.y < -WINDOW_HEIGHT / 2.0
        {
            commands.entity(bullet).despawn();
        }
    }
}

pub fn detect_collision_bullet_boss(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Fire>>,
    mut boss_query: Query<(Entity, &Transform, &mut Health, &RectangularEnvelop), With<Boss>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((boss, boss_transform, mut boss_health, boss_envelop)) = boss_query.get_single_mut() {
        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            if rectangles_intersect(
                bullet_transform.translation,
                RectangularEnvelop {
                    half_x: 0.0,
                    half_y: 0.0,
                },
                boss_transform.translation,
                *boss_envelop,
            ) {
                let boss_polygon = boss::POLYGON.map(|x| x + boss_transform.translation);
                let triangle_list = boss::create_triangle_list_from_polygon(
                    &boss_polygon,
                    boss_transform.translation,
                );
                let mut iter_triangle = triangle_list.iter();
                let mut collision = false;
                let mut p1 = iter_triangle.next();
                let mut p2 = iter_triangle.next();
                let mut p3 = iter_triangle.next();
                while !collision && p3.is_some() {
                    collision = point_in_triangle_2d(
                        *p1.unwrap(),
                        *p2.unwrap(),
                        *p3.unwrap(),
                        bullet_transform.translation,
                    );
                    p1 = iter_triangle.next();
                    p2 = iter_triangle.next();
                    p3 = iter_triangle.next();
                }
                if collision {
                    commands
                        .spawn()
                        .insert(Impact)
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: 4.0,
                                    vertices: 8,
                                }))
                                .into(),
                            transform: bullet_transform.clone().with_scale(Vec3::splat(5.0)),
                            material: materials.add(spaceship::ATTACK_COLOR.into()),
                            ..default()
                        });

                    commands.entity(bullet_entity).despawn();

                    boss_health.0 -= 1;
                    if boss_health.0 == 0 {
                        commands.entity(boss).despawn_recursive();
                        //                 commands.entity(asteroid_entity).despawn();
                        //                 let mut rng = rand::thread_rng();
                        //                 for _ in 1..asteroid.radius as usize {
                        //                     let debris_dx = rng.gen_range(-asteroid.radius..asteroid.radius);
                        //                     let debris_x = asteroid_transform.translation.x + debris_dx;
                        //                     let dy_max = (asteroid.radius.powi(2) - debris_dx.powi(2)).sqrt();
                        //                     let debris_dy = rng.gen_range(-dy_max..dy_max);
                        //                     let debris_y = asteroid_transform.translation.y + debris_dy;
                        //                     // let z = rng.gen_range(
                        //                     //     asteroid_transform.translation.z - asteroid.radius
                        //                     //         ..asteroid_transform.translation.z + asteroid.radius,
                        //                     // );

                        //                     let velocity = Vec3 {
                        //                         x: rng.gen_range(-0.5..0.5),
                        //                         y: rng.gen_range(-0.5..0.5),
                        //                         // z: rng.gen_range(-0.5..0.5),
                        //                         z: 0.0,
                        //                     };

                        //                     commands
                        //                         .spawn()
                        //                         .insert(Debris)
                        //                         .insert(Velocity(asteroid_velocity.0 + velocity))
                        //                         // .insert(Velocity(asteroid_velocity.0 * 0.5))
                        //                         .insert_bundle(MaterialMesh2dBundle {
                        //                             mesh: meshes
                        //                                 .add(Mesh::from(shape::Circle {
                        //                                     radius: rng.gen_range(
                        //                                         asteroid.radius / 100.0..asteroid.radius / 20.0,
                        //                                     ),
                        //                                     vertices: 8,
                        //                                 }))
                        //                                 .into(),
                        //                             transform: Transform::from_xyz(
                        //                                 debris_x,
                        //                                 debris_y,
                        //                                 ALTITUDE + if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
                        //                             )
                        //                             .with_scale(Vec3::splat(4.0)),
                        //                             material: materials.add(Color::PURPLE.into()),
                        //                             ..default()
                        //                         });
                        //                 }
                        break;
                    }
                }
            }
        }
    }
}

pub fn detect_collision_bullet_spaceship(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<boss::Fire>>,
    mut spaceship_query: Query<
        (Entity, &Transform, &mut Health, &RectangularEnvelop),
        With<Spaceship>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((spaceship, spaceship_transform, mut spaceship_health, spaceship_envelop)) =
        spaceship_query.get_single_mut()
    {
        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            if rectangles_intersect(
                bullet_transform.translation,
                RectangularEnvelop {
                    half_x: 0.0,
                    half_y: 0.0,
                },
                spaceship_transform.translation,
                RectangularEnvelop {
                    half_x: spaceship_envelop.half_x * spaceship_transform.scale.x,
                    half_y: spaceship_envelop.half_y * spaceship_transform.scale.y,
                },
            ) {
                let mut iter_triangle = spaceship::TRIANGLE_LIST
                    .into_iter()
                    .map(|x| x.mul_add(spaceship_transform.scale, spaceship_transform.translation));
                let mut collision = false;
                let mut p1 = iter_triangle.next();
                let mut p2 = iter_triangle.next();
                let mut p3 = iter_triangle.next();
                while !collision && p3.is_some() {
                    collision = point_in_triangle_2d(
                        p1.unwrap(),
                        p2.unwrap(),
                        p3.unwrap(),
                        bullet_transform.translation,
                    );
                    p1 = iter_triangle.next();
                    p2 = iter_triangle.next();
                    p3 = iter_triangle.next();
                }

                if collision {
                    commands
                        .spawn()
                        .insert(Impact)
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: 4.0,
                                    vertices: 8,
                                }))
                                .into(),
                            transform: bullet_transform.clone().with_scale(Vec3::splat(5.0)),
                            material: materials.add(boss::BULLET_COLOR.into()),
                            ..default()
                        });

                    commands.entity(bullet_entity).despawn();

                    spaceship_health.0 -= 1;
                    if spaceship_health.0 == 0 {
                        commands.entity(spaceship).despawn_recursive();
                        //                 commands.entity(asteroid_entity).despawn();
                        //                 let mut rng = rand::thread_rng();
                        //                 for _ in 1..asteroid.radius as usize {
                        //                     let debris_dx = rng.gen_range(-asteroid.radius..asteroid.radius);
                        //                     let debris_x = asteroid_transform.translation.x + debris_dx;
                        //                     let dy_max = (asteroid.radius.powi(2) - debris_dx.powi(2)).sqrt();
                        //                     let debris_dy = rng.gen_range(-dy_max..dy_max);
                        //                     let debris_y = asteroid_transform.translation.y + debris_dy;
                        //                     // let z = rng.gen_range(
                        //                     //     asteroid_transform.translation.z - asteroid.radius
                        //                     //         ..asteroid_transform.translation.z + asteroid.radius,
                        //                     // );

                        //                     let velocity = Vec3 {
                        //                         x: rng.gen_range(-0.5..0.5),
                        //                         y: rng.gen_range(-0.5..0.5),
                        //                         // z: rng.gen_range(-0.5..0.5),
                        //                         z: 0.0,
                        //                     };

                        //                     commands
                        //                         .spawn()
                        //                         .insert(Debris)
                        //                         .insert(Velocity(asteroid_velocity.0 + velocity))
                        //                         // .insert(Velocity(asteroid_velocity.0 * 0.5))
                        //                         .insert_bundle(MaterialMesh2dBundle {
                        //                             mesh: meshes
                        //                                 .add(Mesh::from(shape::Circle {
                        //                                     radius: rng.gen_range(
                        //                                         asteroid.radius / 100.0..asteroid.radius / 20.0,
                        //                                     ),
                        //                                     vertices: 8,
                        //                                 }))
                        //                                 .into(),
                        //                             transform: Transform::from_xyz(
                        //                                 debris_x,
                        //                                 debris_y,
                        //                                 ALTITUDE + if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
                        //                             )
                        //                             .with_scale(Vec3::splat(4.0)),
                        //                             material: materials.add(Color::PURPLE.into()),
                        //                             ..default()
                        //                         });
                        //                 }
                        break;
                    }
                }
            }
        }
    }
}

pub fn point_in_triangle_2d(p1: Vec3, p2: Vec3, p3: Vec3, p: Vec3) -> bool {
    let denominator = (p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y);
    let a = ((p2.y - p3.y) * (p.x - p3.x) + (p3.x - p2.x) * (p.y - p3.y)) / denominator;
    let b = ((p3.y - p1.y) * (p.x - p3.x) + (p1.x - p3.x) * (p.y - p3.y)) / denominator;
    let c = 1.0 - a - b;

    a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0 && c >= 0.0 && c <= 1.0
}

pub fn rectangles_intersect(
    center1: Vec3,
    envelop1: RectangularEnvelop,
    center2: Vec3,
    envelop2: RectangularEnvelop,
) -> bool {
    let intersect_x = (center1.x - center2.x).abs() <= envelop1.half_x + envelop2.half_x;
    let intersect_y = (center1.y - center2.y).abs() <= envelop1.half_y + envelop2.half_y;

    return intersect_x && intersect_y;
}

pub fn despawn_blast(mut commands: Commands, query: Query<(Entity, &Parent), With<Blast>>) {
    for (blast, parent) in query.iter() {
        commands.entity(parent.get()).remove_children(&[blast]);
        commands.entity(blast).despawn();
    }
}
