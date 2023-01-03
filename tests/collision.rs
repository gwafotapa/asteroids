use asteroids::{
    asteroid::Asteroid,
    collision::{cache::Cache, detection::*},
    *,
};
use bevy::{prelude::*, sprite::Mesh2dHandle};
use std::f32::consts::PI;

const BLUE: &str = "\x1b[94m";
const RED: &str = "\x1b[91m";
const WHITE: &str = "\x1b[97m";

#[test]
fn collision() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(Cache::default())
        .add_startup_system(camera::spawn)
        .add_system(asteroid::update)
        .add_system(
            collision::generic::among::<asteroid::Asteroid, fire::Fire, spaceship::Spaceship>
                .after(asteroid::update),
        );

    let healths = [Health(100), Health(100)];
    let radii = [100.0, 100.0];
    let v = Vec3::new(10.0, 0.0, 0.0);
    let velocities = [Velocity(v), Velocity(-v)];
    let angular_velocities = [AngularVelocity(2.0 * PI), AngularVelocity(0.0)];
    let transforms = [
        Transform::from_translation(Vec3::new(-radii[0], 0.0, 0.0) + velocities[0].0),
        Transform::from_translation(Vec3::new(radii[1], 0.0, 0.0) + velocities[1].0),
    ];

    let asteroids = spawn_asteroids::<2>(
        &mut app,
        healths,
        radii,
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

    let cache = app.world.resource::<Cache>();
    println!("cache: {:?}", cache);
    assert!(!cache.new.is_empty());
}

fn spawn_asteroids<const N: usize>(
    app: &mut App,
    health: [Health; N],
    radius: [f32; N],
    // mass: Option<Mass>,
    transform: [Transform; N],
    velocity: [Velocity; N],
    angular_velocity: [AngularVelocity; N],
) -> [Entity; N] {
    let mut asteroids = [Entity::from_raw(0); N];
    for i in 0..N {
        asteroids[i] = spawn_asteroid(
            app,
            health[i],
            radius[i],
            transform[i],
            velocity[i],
            angular_velocity[i],
        );
    }

    asteroids
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

fn spawn_asteroid(
    app: &mut App,
    health: Health,
    radius: f32,
    // mass: Option<Mass>,
    transform: Transform,
    velocity: Velocity,
    angular_velocity: AngularVelocity,
) -> Entity {
    let area = PI * radius.powi(2);
    let mass = Mass(area);
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
