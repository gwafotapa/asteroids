use asteroids::{
    asteroid::Asteroid,
    collision::{
        cache::Cache,
        detection::{triangle::Triangle, *},
    },
    *,
};
use bevy::{app::PluginGroupBuilder, prelude::*, sprite::Mesh2dHandle};
use std::f32::consts::PI;

const BLUE: &str = "\x1b[94m";
const RED: &str = "\x1b[91m";
const WHITE: &str = "\x1b[97m";

#[test]
fn asteroids_dimension_1() {
    let mut app = App::new();
    app.add_plugins(TestPlugins)
        .insert_resource(Cache::default())
        .add_startup_system(spawn_camera)
        .add_system(asteroid::update)
        .add_system(
            collision::generic::among::<asteroid::Asteroid, fire::Fire, spaceship::Spaceship>
                .after(asteroid::update),
        );

    let healths = [Health(100), Health(100)];
    let radii = [100.0, 100.0];
    let maybe_masses = [None, None];
    let v = Vec3::new(10.0, 0.0, 0.0);
    let velocities = [Velocity(v), Velocity(-v)];
    let angular_velocities = [AngularVelocity(2.0 * PI), AngularVelocity(0.0)];
    let transforms = [
        Transform::from_translation(Vec3::new(-radii[0], 0.0, 0.0) - velocities[0].0),
        Transform::from_translation(Vec3::new(radii[1], 0.0, 0.0) - velocities[1].0),
    ];

    let asteroids = spawn_asteroids::<2>(
        &mut app,
        healths,
        radii,
        maybe_masses,
        transforms,
        velocities,
        angular_velocities,
    );

    asteroid_print_static(&app, asteroids[0], Some(BLUE));
    asteroid_print_static(&app, asteroids[1], Some(RED));

    println!("PRE COLLISION");
    asteroid_print_dynamic(&app, asteroids[0], Some(BLUE));
    asteroid_print_dynamic(&app, asteroids[1], Some(RED));

    app.update();

    println!("POST COLLISION");
    asteroid_print_dynamic(&app, asteroids[0], Some(BLUE));
    asteroid_print_dynamic(&app, asteroids[1], Some(RED));

    assert!(app.world.resource::<Cache>().new.len() == 1);
    assert!(app.world.get::<Velocity>(asteroids[0]).unwrap().0 == -velocities[0].0);
    assert!(app.world.get::<Velocity>(asteroids[1]).unwrap().0 == -velocities[1].0);
    assert!(app.world.get::<AngularVelocity>(asteroids[0]).unwrap().0 == angular_velocities[0].0);
    assert!(app.world.get::<AngularVelocity>(asteroids[1]).unwrap().0 == angular_velocities[1].0);
}

#[test]
fn asteroids_dimension_2() {
    let mut app = App::new();
    app.add_plugins(TestPlugins)
        .insert_resource(Cache::default())
        .add_startup_system(spawn_camera)
        .add_system(asteroid::update)
        .add_system(
            collision::generic::among::<asteroid::Asteroid, fire::Fire, spaceship::Spaceship>
                .after(asteroid::update),
        );

    let epsilon: f32 = 0.01;
    let healths = [Health(100), Health(100)];
    let radii = [100.0, 100.0];
    let maybe_masses = [None, None];
    let velocities = [
        Velocity(Vec3::new(10.0, 0.0, 0.0)),
        Velocity(Vec3::new(-30.0, 0.0, 0.0)),
    ];
    let angular_velocities = [AngularVelocity(0.0), AngularVelocity(-PI)];
    // Add epsilon so asteroids are not just tangent but intersect properly
    let x1 = -radii[0] / 2.0f32.sqrt() + epsilon;
    let x2 = radii[1] / 2.0f32.sqrt();
    let transforms = [
        Transform::from_translation(Vec3::new(x1, 0.0, 0.0) - velocities[0].0),
        Transform::from_translation(Vec3::new(x2, -x1 + x2, 0.0) - velocities[1].0),
    ];

    let asteroids = spawn_asteroids::<2>(
        &mut app,
        healths,
        radii,
        maybe_masses,
        transforms,
        velocities,
        angular_velocities,
    );

    asteroid_print_static(&app, asteroids[0], Some(BLUE));
    asteroid_print_static(&app, asteroids[1], Some(RED));

    println!("PRE COLLISION");
    asteroid_print_dynamic(&app, asteroids[0], Some(BLUE));
    asteroid_print_dynamic(&app, asteroids[1], Some(RED));

    app.update();

    println!("POST COLLISION");
    asteroid_print_dynamic(&app, asteroids[0], Some(BLUE));
    asteroid_print_dynamic(&app, asteroids[1], Some(RED));

    assert!(app.world.resource::<Cache>().new.len() == 1);
    assert!(
        (app.world.get::<Velocity>(asteroids[0]).unwrap().0 - Vec3::new(-10.0, -20.0, 0.0))
            .length()
            < epsilon
    );
    assert!(
        (app.world.get::<Velocity>(asteroids[1]).unwrap().0 - Vec3::new(-10.0, 20.0, 0.0)).length()
            < epsilon
    );
    assert!(
        (app.world.get::<AngularVelocity>(asteroids[0]).unwrap().0 - angular_velocities[0].0).abs()
            < epsilon
    );
    assert!(
        (app.world.get::<AngularVelocity>(asteroids[1]).unwrap().0 - angular_velocities[1].0).abs()
            < epsilon
    );
}

fn spawn_spaceship_triangle(
    app: &mut App,
    maybe_health: Option<Health>,
    maybe_mass: Option<Mass>,
    maybe_moment_of_inertia: Option<MomentOfInertia>,
    triangle: Triangle,
    transform: Transform,
    velocity: Velocity,
    angular_velocity: AngularVelocity,
) -> Entity {
    let area = 1.0;
    let mass = maybe_mass.unwrap_or(Mass(area));
    let moment_of_inertia =
        maybe_moment_of_inertia.unwrap_or_else(|| MomentOfInertia(0.5 * mass.0 * area / PI));
    app.world
        .spawn(Spaceship)
        .insert(mass)
        .insert(moment_of_inertia)
        .insert(maybe_health.unwrap_or(Health(100)))
        .insert(transform)
        .insert(velocity)
        .insert(angular_velocity)
        .id()
}

fn spawn_asteroids<const N: usize>(
    app: &mut App,
    healths: [Health; N],
    radii: [f32; N],
    maybe_masses: [Option<Mass>; N],
    transforms: [Transform; N],
    velocities: [Velocity; N],
    angular_velocities: [AngularVelocity; N],
) -> [Entity; N] {
    let mut asteroids = [Entity::from_raw(0); N];
    for i in 0..N {
        asteroids[i] = spawn_asteroid(
            app,
            healths[i],
            radii[i],
            maybe_masses[i],
            transforms[i],
            velocities[i],
            angular_velocities[i],
        );
    }

    asteroids
}

fn spawn_asteroid(
    app: &mut App,
    health: Health,
    radius: f32,
    maybe_mass: Option<Mass>,
    transform: Transform,
    velocity: Velocity,
    angular_velocity: AngularVelocity,
) -> Entity {
    let mass = maybe_mass.unwrap_or_else(|| Mass(PI * radius.powi(2)));
    let moment_of_inertia = MomentOfInertia(0.5 * mass.0 * radius.powi(2));
    let mesh = Mesh2dHandle(app.world.resource_mut::<Assets<Mesh>>().add(Mesh::from(
        shape::Circle {
            radius,
            vertices: 16,
        },
    )));

    app.world
        .spawn(Asteroid { radius })
        .insert(health)
        .insert(mass)
        .insert(moment_of_inertia)
        .insert(velocity)
        .insert(angular_velocity)
        .insert(Collider {
            aabb: Aabb {
                hw: radius,
                hh: radius,
            },
            topology: Topology::Disk { radius },
        })
        .insert(ColorMesh2dBundle {
            mesh: mesh.clone(),
            transform,
            ..Default::default()
        })
        .id()
}

fn asteroid_print_static(app: &App, asteroid: Entity, maybe_color: Option<&str>) {
    if let Some(color) = maybe_color {
        print!("{}", color);
    }
    let radius = app.world.get::<Asteroid>(asteroid).unwrap().radius;
    let mass = app.world.get::<Mass>(asteroid).unwrap().0;
    let moment_of_inertia = app.world.get::<MomentOfInertia>(asteroid).unwrap().0;
    println!("radius: {}", radius);
    println!("mass: {}", mass);
    println!("moment of inertia: {}", moment_of_inertia);
    if maybe_color.is_some() {
        print!("{}", WHITE);
    }
}

fn asteroid_print_dynamic(app: &App, asteroid: Entity, maybe_color: Option<&str>) {
    if let Some(color) = maybe_color {
        print!("{}", color);
    }
    let Transform {
        translation,
        rotation,
        scale: _,
    } = app.world.get::<Transform>(asteroid).unwrap();
    let velocity = app.world.get::<Velocity>(asteroid).unwrap().0;
    let angular_velocity = app.world.get::<AngularVelocity>(asteroid).unwrap().0;
    println!("transform: {:?} {:?}", translation, rotation);
    println!("velocity: {:?}", velocity);
    println!("angular velocity: {}", angular_velocity);
    if maybe_color.is_some() {
        print!("{}", WHITE);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

struct TestPlugins;

impl PluginGroup for TestPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // .add(bevy::log::LogPlugin::default())
            .add(bevy::core::CorePlugin::default())
            .add(bevy::time::TimePlugin::default())
            // .add(bevy::transform::TransformPlugin::default())
            // .add(bevy::hierarchy::HierarchyPlugin::default())
            // .add(bevy::diagnostic::DiagnosticsPlugin::default())
            // .add(bevy::input::InputPlugin::default())
            .add(bevy::window::WindowPlugin::default())
            .add(bevy::asset::AssetPlugin::default())
            // .add(bevy::asset::debug_asset_server::DebugAssetServerPlugin::default())
            // .add(bevy::scene::ScenePlugin::default())
            // .add(bevy::winit::WinitPlugin::default())
            .add(bevy::render::RenderPlugin::default())
            .add(bevy::render::texture::ImagePlugin::default())
            .add(bevy::core_pipeline::CorePipelinePlugin::default())
        // .add(bevy::sprite::SpritePlugin::default())
        // .add(bevy::text::TextPlugin::default())
        // .add(bevy::ui::UiPlugin::default())
        // .add(bevy::pbr::PbrPlugin::default())
        // .add(bevy::gltf::GltfPlugin::default())
        // .add(bevy::audio::AudioPlugin::default())
        // .add(bevy::gilrs::GilrsPlugin::default())
        // .add(bevy::animation::AnimationPlugin::default())
    }
}
