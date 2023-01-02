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

    let asteroids = spawn_asteroids(&mut app);
    let Transform {
        translation: translation1,
        rotation: rotation1,
        scale: _,
    } = app.world.get::<Transform>(asteroids[0]).unwrap();
    let velocity1 = app.world.get::<Velocity>(asteroids[0]).unwrap();
    print!("\x1b[94m");
    println!("transform1: {:?} {:?}", translation1, rotation1);
    println!("velocity1: {:?}", velocity1);
    app.update();
    let Transform {
        translation: translation2,
        rotation: rotation2,
        scale: _,
    } = app.world.get::<Transform>(asteroids[0]).unwrap();
    let velocity2 = app.world.get::<Velocity>(asteroids[0]).unwrap();
    print!("\x1b[91m");
    println!("transform2: {:?} {:?}", translation2, rotation2);
    println!("velocity2: {:?}", velocity2);
    print!("\x1b[97m");

    let cache = app.world.resource::<Cache>();
    println!("cache: {:?}", cache);
    assert!(!cache.new.is_empty());
}

fn spawn_asteroids(app: &mut App) -> Vec<Entity> {
    let [radius1, radius2] = [100.0, 100.0];
    let [health1, health2] = [100, 100];
    let [velocity1, velocity2] = [Vec3::new(10.0, 0.0, 0.0), Vec3::new(-10.0, 0.0, 0.0)];
    let [angular_velocity1, angular_velocity2] = [0.0, 0.0];
    let [transform1, transform2] = [
        Transform::from_translation(Vec3::new(-radius1, 0.0, 0.0) - velocity1),
        Transform::from_translation(Vec3::new(radius2, 0.0, 0.0) - velocity2),
    ];
    let mut meshes = app.world.resource_mut::<Assets<Mesh>>();
    let mesh1: Mesh2dHandle = meshes
        .add(Mesh::from(shape::Circle {
            radius: radius1,
            vertices: 16,
        }))
        .into();
    let mesh2: Mesh2dHandle = meshes
        .add(Mesh::from(shape::Circle {
            radius: radius2,
            vertices: 16,
        }))
        .into();

    let mut asteroids = Vec::new();
    asteroids.push((
        Asteroid { radius: radius1 },
        mesh1,
        Health(health1),
        transform1,
        Velocity(velocity1),
        AngularVelocity(angular_velocity1),
    ));
    asteroids.push((
        Asteroid { radius: radius2 },
        mesh2,
        Health(health2),
        transform2,
        Velocity(velocity2),
        AngularVelocity(angular_velocity2),
    ));

    let mut ids = Vec::new();
    for (asteroid, mesh, health, transform, velocity, angular_velocity) in asteroids.into_iter() {
        let radius = asteroid.radius;
        let area = PI * radius.powi(2);
        let mass = Mass(area);
        let moment_of_inertia = MomentOfInertia(0.5 * mass.0 * radius.powi(2));

        let id = app
            .world
            .spawn(asteroid)
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
            .id();

        ids.push(id);
    }

    ids
}
