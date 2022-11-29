use bevy::{prelude::*, render::mesh::PrimitiveTopology};
// use rand::Rng;

use crate::{boss::BossCore, spaceship::Spaceship, WINDOW_HEIGHT, WINDOW_WIDTH};

const COMPASS_POSITION: Vec3 = Vec3 {
    x: WINDOW_WIDTH / 2.0 - 150.0,
    y: WINDOW_HEIGHT / 2.0 - 10.0,
    z: 0.0,
    // z: -CAMERA_Z,
};
// Needle position relative to the compass position
const NEEDLE_POSITION: Vec3 = Vec3 {
    x: 120.0,
    y: 0.0,
    z: 0.0,
};
// const DISTANCE_TO_TARGET: f32 = 5000.0;
const FONT: &'static str = "fonts/FiraSans-Bold.ttf";
const FONT_SIZE: f32 = 20.0;
const COLOR: Color = Color::DARK_GRAY;
const NEEDLE_SCALE: f32 = 0.13;

// #[derive(Component)]
// pub struct Level {
//     target: Vec3,
//     // boss_spawned: bool,
// }

#[derive(Component)]
pub struct Compass;
// { pub target: Vec3, }

#[derive(Component)]
pub struct Needle;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    query_camera: Query<&Transform, With<Camera>>,
    query_boss: Query<&Transform, With<BossCore>>,
    query_spaceship: Query<&Transform, With<Spaceship>>,
) {
    let camera = query_camera.single();
    let boss = query_boss.single();
    let spaceship = query_spaceship.single();

    let font = asset_server.load(FONT);
    let text_style = TextStyle {
        font,
        font_size: FONT_SIZE,
        color: COLOR,
    };
    let text_alignment = TextAlignment::CENTER_LEFT;

    let c1 = Vec3::new(75.0, 0.0, 0.0);
    let c2 = Vec3::new(-50.0, 50.0, 0.0);
    let c3 = Vec3::new(-25.0, 0.0, 0.0);
    let c4 = Vec3::new(-50.0, -50.0, 0.0);
    let v_pos = vec![c1, c2, c3, c3, c4, c1];
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    let compass = commands
        .spawn(Compass)
        // { target: Vec3::new(x, y, BOSS_Z),
        // boss_spawned: false,
        // })
        .insert(Text2dBundle {
            text: Text::from_section("", text_style).with_alignment(text_alignment),
            transform: Transform::from_translation(camera.translation + COMPASS_POSITION),
            ..default()
        })
        .id();

    let needle = commands
        .spawn(Needle)
        .insert(ColorMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform: Transform::from_translation(NEEDLE_POSITION)
                .with_scale(Vec3::splat(NEEDLE_SCALE))
                .with_rotation(Quat::from_rotation_arc_2d(
                    Vec2::X,
                    (boss.translation - spaceship.translation)
                        .truncate()
                        .normalize(),
                )),
            material: materials.add(COLOR.into()),
            ..default()
        })
        .id();

    commands.entity(compass).add_child(needle);
}

pub fn update(
    mut query_compass: Query<(&mut Transform, &mut Text), With<Compass>>,
    mut query_needle: Query<&mut Transform, (With<Needle>, Without<Compass>)>,
    query_spaceship: Query<&Transform, (With<Spaceship>, Without<Compass>, Without<Needle>)>,
    query_boss: Query<&Transform, (With<BossCore>, Without<Compass>, Without<Needle>)>,
    query_camera: Query<&Transform, (With<Camera>, Without<Compass>, Without<Needle>)>,
) {
    if let Ok(spaceship) = query_spaceship.get_single() {
        if let Ok(boss) = query_boss.get_single() {
            let camera = query_camera.single();
            let (mut compass, mut text) = query_compass.single_mut();
            let mut needle = query_needle.single_mut();
            compass.translation = camera.translation + COMPASS_POSITION;
            needle.rotation = Quat::from_rotation_arc_2d(
                Vec2::X,
                (boss.translation - spaceship.translation)
                    .truncate()
                    .normalize(),
            );
            let distance = (boss.translation - spaceship.translation).length();
            text.sections[0].value = format!("Target: {:<7.0}", distance);
        }
    }
}
