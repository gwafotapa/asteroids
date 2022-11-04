use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
// use std::time::Instant;
use std::f32::consts::PI;

mod spaceship;

use spaceship::{Direction, Spaceship};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const INITIAL_COUNT_OF_STARS_BY_VELOCITY: usize = 10;
const MAX_SPEED_OF_STARS: usize = 10;
const MAX_SPEED_OF_ASTEROIDS: usize = 5;
const MAX_HEALTH_OF_ASTEROIDS: usize = 6;
const BULLET_RADIUS: f32 = 2.0;
const ALTITUDE: f32 = 100.0;
const INITIAL_DISTANCE: usize = 0;
const BOSS_ACCELERATION: f32 = 0.1;

#[derive(Component)]
struct Star;

#[derive(Component)]
pub struct Velocity(Vec3);

#[derive(Component)]
struct Asteroid {
    health: usize,
    radius: f32,
}

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Debris;

#[derive(Component)]
struct Impact;

// #[derive(Component)]
// struct SpawnedTime(Instant);

#[derive(Component)]
struct Distance(usize);

#[derive(Component)]
struct Boss;

#[derive(Component)]
struct BossPart;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Asteroids".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)
        .add_startup_system(spaceship::spaceship)
        .add_startup_system(setup_distance)
        .add_startup_system(setup_stars)
        .add_system(add_stars)
        .add_system(update_stars)
        .add_system(asteroids)
        .add_system(keyboard_input)
        .add_system(detect_collision_spaceship_asteroid)
        .add_system(update_bullets)
        .add_system(update_impacts)
        .add_system(detect_collision_bullet_asteroid)
        .add_system(update_debris)
        .add_system(update_distance)
        .add_system(add_boss)
        .add_system(move_boss)
        .add_system(bevy::window::close_on_esc)
        // .add_system_to_stage(
        //     CoreStage::PostUpdate,
        //     debug_globaltransform.after(TransformSystem::TransformPropagate),
        // )
        // .add_startyp_system(test)
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_distance(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(Distance(INITIAL_DISTANCE));
}

fn update_distance(mut query: Query<(&mut Text, &mut Distance)>) {
    for (mut text, mut distance) in &mut query {
        if distance.0 > 0 {
            distance.0 -= 1;
        }
        text.sections[0].value = format!("Distance: {:12}", distance.0);
    }
}

fn setup_stars(
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

fn add_stars(
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

fn update_stars(
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
// fn debug_globaltransform(query: Query<&GlobalTransform, With<Star>>) {
//     for mesh in query.iter() {
//         debug!("Mesh at: {:?}", mesh.translation());
//     }
// }

fn keyboard_input(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
) {
    // if keys.just_pressed(KeyCode::Space) {
    //     // Space was pressed
    // }

    // if keys.just_released(KeyCode::LControl) {
    //     // Left Ctrl was released
    // }

    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
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
            create_bullet(
                commands,
                meshes,
                materials,
                transform.translation + Vec3::from(spaceship::CANON_POSITION),
            );
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

fn create_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position: Vec3,
) {
    commands
        .spawn()
        .insert(Bullet)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle {
                    radius: BULLET_RADIUS,
                    vertices: 4,
                }))
                .into(),
            transform: Transform::from_translation(position),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            ..default()
        });
}

fn asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asteroid_query: Query<(&mut Transform, &Velocity, &Asteroid, Entity)>,
    distance_query: Query<&Distance>,
) {
    let mut rng = rand::thread_rng();

    if distance_query.single().0 > 0 && rng.gen_range(0..100) == 0 {
        let health = rng.gen_range(1..MAX_HEALTH_OF_ASTEROIDS + 1);
        let radius = (health * 20) as f32;
        let speed = rng.gen_range(1..MAX_SPEED_OF_ASTEROIDS + 1) as f32;
        let velocity = Vec3::from([-speed, 0., 0.]);

        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius,
                        vertices: 16,
                    }))
                    .into(),
                transform: Transform::from_xyz(450., rng.gen_range(-250..250) as f32, ALTITUDE),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            })
            .insert(Asteroid { health, radius })
            .insert(Velocity(velocity));
    }

    for (mut transform, velocity, asteroid, entity) in asteroid_query.iter_mut() {
        transform.translation += velocity.0;
        if transform.translation.x < -WINDOW_WIDTH / 2.0 - asteroid.radius {
            commands.entity(entity).despawn();
        }
    }
}

fn detect_collision_spaceship_asteroid(
    mut commands: Commands,
    spaceship_query: Query<(Entity, &Transform, &Spaceship, &Velocity)>,
    asteroid_query: Query<(&Transform, &Asteroid)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((spaceship_entity, spaceship_transform, spaceship, spaceship_velocity)) =
        spaceship_query.get_single()
    {
        for (asteroid_transform, asteroid) in asteroid_query.iter() {
            // if spaceship_transform
            //     .translation
            //     .distance(asteroid_transform.translation)
            //     < asteroid.radius + 40.0
            // {
            for &point in spaceship.envelop() {
                if asteroid_transform
                    .translation
                    // .distance((point + spaceship_transform.translation) * spaceship_transform.scale.x)
                    .distance(point + spaceship_transform.translation)
                    < asteroid.radius
                {
                    commands.entity(spaceship_entity).despawn();
                    let mut rng = rand::thread_rng();
                    for _ in 1..10 {
                        let debris_dx = rng.gen_range(-30.0..30.0);
                        let debris_x = spaceship_transform.translation.x + debris_dx;
                        let debris_dy = rng.gen_range(-20.0..20.0);
                        let debris_y = spaceship_transform.translation.y + debris_dy;

                        let velocity = Vec3 {
                            x: rng.gen_range(-0.5..0.5),
                            y: rng.gen_range(-0.5..0.5),
                            z: 0.0,
                        };

                        commands
                            .spawn()
                            .insert(Debris)
                            .insert(Velocity(spaceship_velocity.0 + velocity))
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
            // }
        }
    }
}

fn update_bullets(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity), With<Bullet>>,
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

fn detect_collision_bullet_asteroid(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut asteroid_query: Query<(Entity, &Transform, &mut Asteroid, &Velocity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (asteroid_entity, asteroid_transform, mut asteroid, asteroid_velocity) in
            asteroid_query.iter_mut()
        {
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
                        material: materials.add(Color::YELLOW.into()),
                        ..default()
                    });

                commands.entity(bullet_entity).despawn();

                asteroid.health -= 1;
                if asteroid.health == 0 {
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

fn update_debris(
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

fn update_impacts(
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

fn add_boss(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    distance_query: Query<&Distance>,
    asteroid_query: Query<&Asteroid>,
    boss_query: Query<&Boss>,
) {
    if distance_query.single().0 == 0 && boss_query.is_empty() && asteroid_query.is_empty() {
        let boss = commands
            .spawn()
            .insert(Boss)
            .insert(Velocity(Vec3::ZERO))
            .insert_bundle(SpatialBundle {
                transform: Transform::from_xyz(300.0, 0.0, ALTITUDE),
                ..default()
            })
            .id();

        let boss_part1 = commands
            .spawn()
            .insert(BossPart)
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: (200.0, 200.0).into(),
                        flip: false,
                    }))
                    .into(),
                material: materials.add(Color::ORANGE.into()),
                ..default()
            })
            .id();

        let boss_part2 = commands
            .spawn()
            .insert(BossPart)
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: (200.0, 200.0).into(),
                        flip: false,
                    }))
                    .into(),
                transform: Transform::identity().with_rotation(Quat::from_rotation_z(PI / 4.0)),
                material: materials.add(Color::ORANGE.into()),
                ..default()
            })
            .id();

        commands
            .entity(boss)
            .push_children(&[boss_part1, boss_part2]);
    }
}

fn move_boss(mut query: Query<(&mut Transform, &mut Velocity), With<Boss>>) {
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        let mut rng = rand::thread_rng();
        velocity.0 += match rng.gen_range(0..4) {
            0 => Vec3::from([BOSS_ACCELERATION, 0.0, 0.0]),
            1 => Vec3::from([-BOSS_ACCELERATION, 0.0, 0.0]),
            2 => Vec3::from([0.0, BOSS_ACCELERATION, 0.0]),
            3 => Vec3::from([0.0, -BOSS_ACCELERATION, 0.0]),
            _ => unreachable!(),
        };
        transform.translation += velocity.0;
    }
}
