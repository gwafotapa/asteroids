use asteroids::{
    asteroid::Asteroid,
    collision::{cache::Cache, detection::*},
    *,
};
use bevy::{prelude::*, sprite::Mesh2dHandle};
use std::f32::consts::PI;

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

    let radius1 = app.world.get::<Asteroid>(asteroids[0]).unwrap().radius;
    let mass1 = app.world.get::<Mass>(asteroids[0]).unwrap().0;
    let moment_of_inertia1 = app.world.get::<MomentOfInertia>(asteroids[0]).unwrap().0;
    print!("\x1b[94m");
    println!("radius 1: {:?}", radius1);
    println!("mass 1: {:?}", mass1);
    println!("moment of inertia 1: {:?}", moment_of_inertia1);
    print!("\x1b[97m");
    let radius2 = app.world.get::<Asteroid>(asteroids[1]).unwrap().radius;
    let mass2 = app.world.get::<Mass>(asteroids[1]).unwrap().0;
    let moment_of_inertia2 = app.world.get::<MomentOfInertia>(asteroids[1]).unwrap().0;
    print!("\x1b[91m");
    println!("radius 2: {:?}", radius2);
    println!("mass 2: {:?}", mass2);
    println!("moment of inertia 2: {:?}", moment_of_inertia2);
    print!("\x1b[97m");

    println!("PRE COLLISION");
    let Transform {
        translation: translation1,
        rotation: rotation1,
        scale: _,
    } = app.world.get::<Transform>(asteroids[0]).unwrap();
    let velocity1 = app.world.get::<Velocity>(asteroids[0]).unwrap().0;
    let angular_velocity1 = app.world.get::<AngularVelocity>(asteroids[0]).unwrap().0;
    print!("\x1b[94m");
    println!("transform 1: {:?} {:?}", translation1, rotation1);
    println!("velocity 1: {:?}", velocity1);
    println!("angular velocity 1: {:?}", angular_velocity1);
    print!("\x1b[97m");
    let Transform {
        translation: translation2,
        rotation: rotation2,
        scale: _,
    } = app.world.get::<Transform>(asteroids[1]).unwrap();
    let velocity2 = app.world.get::<Velocity>(asteroids[1]).unwrap().0;
    let angular_velocity2 = app.world.get::<AngularVelocity>(asteroids[1]).unwrap().0;
    print!("\x1b[91m");
    println!("transform 2: {:?} {:?}", translation2, rotation2);
    println!("velocity 2: {:?}", velocity2);
    println!("angular velocity 2: {:?}", angular_velocity2);
    print!("\x1b[97m");

    app.update();

    println!("POST COLLISION");
    let Transform {
        translation: translation1,
        rotation: rotation1,
        scale: _,
    } = app.world.get::<Transform>(asteroids[0]).unwrap();
    let velocity1 = app.world.get::<Velocity>(asteroids[0]).unwrap().0;
    let angular_velocity1 = app.world.get::<AngularVelocity>(asteroids[0]).unwrap().0;
    print!("\x1b[94m");
    println!("transform 1: {:?} {:?}", translation1, rotation1);
    println!("velocity 1: {:?}", velocity1);
    println!("angular velocity 1: {:?}", angular_velocity1);
    print!("\x1b[97m");
    let Transform {
        translation: translation2,
        rotation: rotation2,
        scale: _,
    } = app.world.get::<Transform>(asteroids[1]).unwrap();
    let velocity2 = app.world.get::<Velocity>(asteroids[1]).unwrap().0;
    let angular_velocity2 = app.world.get::<AngularVelocity>(asteroids[1]).unwrap().0;
    print!("\x1b[91m");
    println!("transform 2: {:?} {:?}", translation2, rotation2);
    println!("velocity 2: {:?}", velocity2);
    println!("angular velocity 2: {:?}", angular_velocity2);
    print!("\x1b[97m");

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
