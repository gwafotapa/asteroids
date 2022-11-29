use bevy::{prelude::*, render::mesh::PrimitiveTopology};
use rand::Rng;

use crate::{boss::BOSS_Z, spaceship::Spaceship, CAMERA_Z, WINDOW_HEIGHT, WINDOW_WIDTH};

const COMPASS_POSITION: Vec3 = Vec3 {
    x: WINDOW_WIDTH / 2.0 - 10.0,
    y: WINDOW_HEIGHT / 2.0 - 10.0,
    z: -CAMERA_Z,
};
const DISTANCE_TO_TARGET: f32 = 5000.0;

#[derive(Component)]
pub struct Level {
    target: Vec3,
    // boss_spawned: bool,
}

#[derive(Component)]
pub struct Compass;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<Camera>>,
) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-DISTANCE_TO_TARGET..DISTANCE_TO_TARGET);
    let y_max = (DISTANCE_TO_TARGET.powi(2) - x.powi(2)).sqrt();
    let y = rng.gen_range(-y_max..y_max);

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
            target: Vec3::new(x, y, BOSS_Z),
            // boss_spawned: false,
        });

    let camera = query.single();
    let c1 = Vec3::new(75.0, 0.0, 0.0);
    let c2 = Vec3::new(-50.0, 50.0, 0.0);
    let c3 = Vec3::new(-25.0, 0.0, 0.0);
    let c4 = Vec3::new(-50.0, -50.0, 0.0);
    let v_pos = vec![c1, c2, c3, c3, c4, c1];
    let mut compass = Mesh::new(PrimitiveTopology::TriangleList);
    compass.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    commands.spawn(Compass).insert(ColorMesh2dBundle {
        mesh: meshes.add(compass).into(),
        transform: Transform::from_translation(camera.translation + COMPASS_POSITION)
            .with_scale(Vec3::splat(0.13)),
        material: materials.add(Color::WHITE.into()),
        ..default()
    });
}

pub fn update(
    mut query_level: Query<(&Level, &mut Text)>,
    query_spaceship: Query<&Transform, With<Spaceship>>,
    query_camera: Query<&Transform, With<Camera>>,
    mut query_compass: Query<&mut Transform, (With<Compass>, Without<Spaceship>, Without<Camera>)>,
) {
    if let Ok(spaceship) = query_spaceship.get_single() {
        let (level, mut text) = query_level.single_mut();
        let camera = query_camera.single();
        let mut compass = query_compass.single_mut();
        compass.translation = camera.translation + COMPASS_POSITION;
        compass.rotation = Quat::from_rotation_arc_2d(
            Vec2::X,
            (level.target - spaceship.translation)
                .truncate()
                .normalize(),
        );
        let distance = (level.target - spaceship.translation).length();
        text.sections[0].value = format!("Target: {:>7.0}", distance);
    }
}
