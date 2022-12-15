#![allow(clippy::type_complexity)]
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    render::mesh::VertexAttributeValues,
    sprite::Mesh2dHandle,
};
use iyes_loopless::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::{
    collision::math::{triangle::TriangleXY, Collider, Topology},
    debris::Debris,
};

pub mod asteroid;
pub mod blast;
pub mod boss;
pub mod camera;
pub mod collision;
pub mod compass;
pub mod debris;
pub mod fire;
pub mod map;
pub mod spaceship;
pub mod ui;

const PLANE_Z: f32 = 500.0;
// pub const WINDOW_WIDTH: f32 = 1920.0;
// pub const WINDOW_HEIGHT: f32 = 1080.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
// pub const WINDOW_WIDTH: f32 = 800.0;
// pub const WINDOW_HEIGHT: f32 = 600.0;
// const SHINE_FACTOR: f32 = 1.0 / DIM_FACTOR;
const DIM_FACTOR: f32 = 0.92;
const DIM_TIMER: u32 = 50;

#[derive(Component)]
pub struct Velocity(Vec3);

// #[derive(Component)]
// struct SpawnedTime(Instant);

#[derive(Component)]
pub struct Health(i32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    MainMenu,
    GameSetup,
    InGame,
    Paused,
    // GameOver,
    TurnDownLight,
    TurnUpLight,
}

pub fn turn_down_light(
    mut query_visible_mesh: Query<(&Handle<ColorMaterial>, &ComputedVisibility)>,
    mut query_visible_text: Query<(&ComputedVisibility, &mut Text)>,
    mut timer: Local<u32>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_main_menu: Query<Entity, With<ui::main_menu::MainMenu>>,
    mut query_camera: Query<(&mut Camera, &mut UiCameraConfig)>,
    query_without_camera: Query<Entity, Without<Camera>>,
) {
    for (color_material, visibility) in &mut query_visible_mesh {
        if visibility.is_visible() {
            materials.get_mut(color_material).unwrap().color *=
                [DIM_FACTOR, DIM_FACTOR, DIM_FACTOR];
        }
    }

    for (visibility, mut text) in &mut query_visible_text {
        if visibility.is_visible() {
            text.sections[0].style.color *= [DIM_FACTOR, DIM_FACTOR, DIM_FACTOR];
        }
    }

    let (mut camera, mut config) = query_camera.single_mut();
    *timer += 1;
    if *timer == DIM_TIMER {
        if let Ok(main_menu) = query_main_menu.get_single() {
            commands.entity(main_menu).despawn_recursive();
            commands.insert_resource(NextState(GameState::GameSetup));
            camera.is_active = false;
            config.show_ui = false;
        } else {
            for id in &query_without_camera {
                commands.entity(id).despawn();
            }
            commands.insert_resource(NextState(GameState::MainMenu));
            config.show_ui = true;
        }
        *timer = 0;
    }
}

pub fn turn_up_light(
    mut query_visible_mesh: Query<(&Handle<ColorMaterial>, &ComputedVisibility)>,
    mut query_visible_text: Query<(&ComputedVisibility, &mut Text)>,
    mut timer: Local<u32>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (color_material, visibility) in &mut query_visible_mesh {
        if visibility.is_visible() {
            materials.get_mut(color_material).unwrap().color *=
                [1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR];
        }
    }

    for (visibility, mut text) in &mut query_visible_text {
        if visibility.is_visible() {
            text.sections[0].style.color *= [1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR];
        }
    }

    *timer += 1;
    if *timer == DIM_TIMER {
        commands.insert_resource(NextState(GameState::InGame));
        // query_camera.single_mut().show_ui = true;
        *timer = 0;
    }
}

pub fn kill_light(
    mut query_visible_mesh: Query<(&Handle<ColorMaterial>, &ComputedVisibility)>,
    mut query_visible_text: Query<(&ComputedVisibility, &mut Text)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let kill_factor = DIM_FACTOR.powi(DIM_TIMER as i32);

    for (color_material, visibility) in &mut query_visible_mesh {
        if visibility.is_visible() {
            materials.get_mut(color_material).unwrap().color *=
                [kill_factor, kill_factor, kill_factor];
        }
    }

    for (visibility, mut text) in &mut query_visible_text {
        if visibility.is_visible() {
            text.sections[0].style.color *= [kill_factor, kill_factor, kill_factor];
        }
    }
}

pub fn exit_game_setup(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::TurnUpLight));
}

pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (id, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(id).despawn();
        }
    }
}

pub fn despawn_with<C: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<C>>,
) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_recursive_with<C: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<C>>,
) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// // Warning: Should generate some double despawn (with debris::update for example)
// pub fn exit_game(
//     mut commands: Commands,
//     query_all: Query<Entity, Without<Camera>>,
//     // query_all: Query<Entity>,
//     // mut query_camera: Query<&mut UiCameraConfig>,
// ) {
//     for id in &query_all {
//         commands.entity(id).despawn();
//     }
//     // query_camera.single_mut().show_ui = true;
// }

pub fn game_over(
    query: Query<With<spaceship::Spaceship>>,
    mut keyboard_activity: EventReader<KeyboardInput>,
    mut commands: Commands,
) {
    if query.get_single().is_err()
        && keyboard_activity
            .iter()
            .any(|key| key.state == ButtonState::Pressed)
    {
        commands.insert_resource(NextState(GameState::TurnDownLight))
    }
}

pub fn spaceship_exists(query: Query<With<spaceship::Spaceship>>) -> bool {
    !query.is_empty()
}

pub fn ingame_or_paused(game_state: Res<CurrentState<GameState>>) -> bool {
    game_state.0 == GameState::InGame || game_state.0 == GameState::Paused
}

pub fn wreck_with<C: Component>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<
        (
            &Handle<ColorMaterial>,
            &Collider,
            Entity,
            // Option<&Parent>,
            // &GlobalTransform,
            &Health,
            // Option<&Velocity>,
        ),
        With<C>,
    >,
) {
    // for (color, collider, maybe_parent, transform, health, maybe_velocity) in &query {
    for (color, collider, id, health) in &query {
        if health.0 > 0 {
            continue;
        }

        let mut rng = rand::thread_rng();
        let color = materials.get(color).unwrap().color;
        // let velocity = maybe_parent
        //     .map_or(maybe_velocity, |parent| {
        //         query.get_component::<Velocity>(**parent).ok()
        //     })
        //     .map_or(Vec3::ZERO, |v| v.0);

        commands.entity(id).insert(Wreckage);
        commands.entity(id).remove::<C>();
        commands.entity(id).remove::<Mesh2dHandle>();

        // let wreck = commands
        //     .spawn(Wreckage)
        //     .insert(Health(WRECK_HEALTH))
        //     .insert(Velocity(velocity))
        //     .insert(SpatialBundle {
        //         transform: transform.compute_transform(),
        //         ..default()
        //     })
        //     .id();

        match &collider.topology {
            Topology::Triangles { mesh_handle } => {
                if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
                    .get(&mesh_handle.0)
                    .unwrap()
                    .attribute(Mesh::ATTRIBUTE_POSITION)
                {
                    for triplet in vertices.clone().chunks_exact(3) {
                        let triangle: TriangleXY =
                            <[_; 3]>::try_from(triplet).expect("3 items").into();

                        // Arbitrary number of debris per triangle : area/16
                        for _ in 0..(triangle.area() / 16.0).round() as usize {
                            let p = triangle.random_point();
                            let debris =
                                Vec3::new(p.x, p.y, if rng.gen_bool(0.5) { 1.0 } else { -1.0 });
                            // let debris = transform.transform_point(debris_relative);

                            let dv =
                                Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0);

                            let debris = commands
                                // .spawn(Debris)
                                .spawn(WreckageDebris)
                                .insert(Velocity(dv))
                                .insert(ColorMesh2dBundle {
                                    mesh: meshes
                                        .add(Mesh::from(shape::Circle {
                                            radius: rng.gen_range(1.0..10.0),
                                            vertices: 4 * rng.gen_range(1..5),
                                        }))
                                        .into(),
                                    transform: Transform::from_translation(debris),
                                    material: materials.add(color.into()),
                                    ..default()
                                })
                                .id();

                            commands.entity(id).add_child(debris);
                        }
                    }
                }
            }
            Topology::Disk { radius } => {
                let area = PI * radius * radius;
                for _ in 0..(area / 16.0).round() as usize {
                    let rho = rng.gen_range(0.0..*radius);
                    let theta = rng.gen_range(0.0..2.0 * PI);
                    let (sin, cos) = theta.sin_cos();
                    let (x, y) = (rho * cos, rho * sin);
                    let z = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
                    let debris = Vec3::new(x, y, z);

                    let dv = Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0);

                    let debris = commands
                        // .spawn(Debris)
                        .spawn(WreckageDebris)
                        .insert(Velocity(dv))
                        .insert(ColorMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Circle {
                                    radius: rng.gen_range(1.0..radius / 10.0),
                                    vertices: 4 * rng.gen_range(1..5),
                                }))
                                .into(),
                            transform: Transform::from_translation(debris),
                            material: materials.add(color.into()),
                            ..default()
                        })
                        .id();

                    commands.entity(id).add_child(debris);
                }
            }
            Topology::Point => panic!("Found point topology for explosion."),
        }
    }
}

#[derive(Component)]
pub struct Wreckage;

#[derive(Component)]
pub struct WreckageDebris;

const WRECKAGE_HEALTH: i32 = 100;

pub fn wreckage_debris_update(mut query: Query<(&mut Transform, &Velocity), With<WreckageDebris>>) {
    for (mut transform, velocity) in &mut query {
        transform.scale -= 1.0 / WRECKAGE_HEALTH as f32;
        transform.translation += velocity.0;
    }
}

pub fn wreckage_update(
    mut query: Query<(&mut Health, &mut Transform, Option<&Velocity>), With<Wreckage>>,
) {
    for (mut health, mut transform, maybe_velocity) in &mut query {
        health.0 -= 1;
        if let Some(velocity) = maybe_velocity {
            transform.translation += velocity.0;
        }
    }
}

pub fn wreckage_despawn(mut commands: Commands, query: Query<(Entity, &Health), With<Wreckage>>) {
    for (id, health) in &query {
        if health.0 <= -WRECKAGE_HEALTH {
            commands.entity(id).despawn_recursive();
        }
    }
}
