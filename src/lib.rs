use bevy::prelude::*;
// use std::time::Instant;
// use std::f32::consts::SQRT_2;pub

pub mod asteroid;
pub mod boss;
pub mod collision;
pub mod spaceship;
pub mod star;

use spaceship::Spaceship;

pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

const ALTITUDE: f32 = 500.0;
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
const INITIAL_DISTANCE_TO_BOSS: usize = 00000;

#[derive(Component)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct Fire {
    color: Color,
    impact_radius: f32,
    impact_vertices: usize,
}

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
pub struct Health(usize);

// #[derive(Component)]
// pub struct Attack {
//     source: Vec3,
//     color: Color,
//     blast_radius: f32,
//     blast_vertices: usize,
//     fire_radius: f32,
//     fire_vertices: usize,
// }

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Debris;

pub fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
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
    mut query: Query<(Entity, &mut Transform, &mut Velocity), With<Spaceship>>,
) {
    // if keys.just_pressed(KeyCode::Space) {
    //     // Space was pressed
    // }

    // if keys.just_released(KeyCode::LControl) {
    //     // Left Ctrl was released
    // }

    if let Ok((spaceship, mut transform, mut velocity)) = query.get_single_mut() {
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
            spaceship::attack(commands, meshes, materials, spaceship, &transform);
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

// pub fn update_bullets(
//     mut commands: Commands,
//     mut query: Query<(&mut Transform, &Velocity, Entity), With<Fire>>,
// ) {
//     for (mut transform, velocity, entity) in query.iter_mut() {
//         transform.translation += velocity.0;
//         // transform.translation += Vec3 {
//         //     x: 4.0,
//         //     y: 0.0,
//         //     z: 0.0,
//         // };
//         if transform.translation.x > WINDOW_WIDTH / 2.0 {
//             commands.entity(entity).despawn();
//         }
//     }
// }

pub fn update_fire(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Velocity, Entity), With<Fire>>,
) {
    for (mut transform, velocity, fire) in query.iter_mut() {
        transform.translation += velocity.0;
        if transform.translation.x > WINDOW_WIDTH / 2.0
            || transform.translation.x < -WINDOW_WIDTH / 2.0
            || transform.translation.y > WINDOW_HEIGHT / 2.0
            || transform.translation.y < -WINDOW_HEIGHT / 2.0
        {
            commands.entity(fire).despawn();
        }
    }
}

pub fn despawn_blast(mut commands: Commands, query: Query<(Entity, &Parent), With<Blast>>) {
    for (blast, parent) in query.iter() {
        commands.entity(parent.get()).remove_children(&[blast]);
        commands.entity(blast).despawn();
    }
}
