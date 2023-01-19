// Tests in this file have to be run individually with the option -- --test-threads=1
// Test 'asteroid_spaceship' needs to be reworked

#[allow(dead_code)]
pub mod collision {
    // Encapsulate in a module to avoid warnings
    use asteroids::{
        asteroid::Asteroid,
        collision::detection::{triangle::Triangle, *},
        *,
    };
    use bevy::{
        app::PluginGroupBuilder, prelude::*, render::mesh::PrimitiveTopology, sprite::Mesh2dHandle,
    };
    use iyes_loopless::prelude::*;
    use std::f32::consts::PI;

    const BLUE: &str = "\x1b[94m";
    const RED: &str = "\x1b[91m";
    const WHITE: &str = "\x1b[97m";

    #[test] // Has to be run with 'cargo test asteroids_dimension_1 -- --test-threads=1'
    #[ignore]
    fn asteroids_dimension_1() {
        let mut app = App::new();
        app.add_plugins(TestPlugins)
            .add_system(collision::generic::with::<asteroid::Asteroid>);

        let health = Health(100);
        let radius: f32 = 100.0;
        let mass = Mass(PI * radius.powi(2));
        let v = Vec3::new(10.0, 0.0, 0.0);

        let mut asteroids = Vec::new();
        asteroids.push(spawn_asteroid(
            &mut app,
            health,
            radius,
            Some(mass),
            Transform::from_xyz(-radius, 0.0, 0.0),
            Velocity(v),
            AngularVelocity(2.0 * PI),
        ));
        asteroids.push(spawn_asteroid(
            &mut app,
            health,
            radius,
            Some(mass),
            Transform::from_xyz(radius, 0.0, 0.0),
            Velocity(-v),
            AngularVelocity(0.0),
        ));

        // let mut asteroids = Vec::new();
        // for (id, _) in app
        //     .world
        //     .query::<(Entity, (With<Asteroid>, Without<Part>))>()
        //     .iter(&app.world)
        // {
        //     asteroids.push(id);
        // }

        entity_print_static(&app, asteroids[0], Some(BLUE));
        entity_print_static(&app, asteroids[1], Some(RED));

        println!("PRE COLLISION");
        entity_print_dynamic(&app, asteroids[0], Some(BLUE));
        entity_print_dynamic(&app, asteroids[1], Some(RED));

        app.update();

        println!("POST COLLISION");
        entity_print_dynamic(&app, asteroids[0], Some(BLUE));
        entity_print_dynamic(&app, asteroids[1], Some(RED));

        assert_eq!(app.world.get::<Velocity>(asteroids[0]).unwrap().0, -v);
        assert_eq!(app.world.get::<Velocity>(asteroids[1]).unwrap().0, v);
        assert_eq!(
            app.world.get::<AngularVelocity>(asteroids[0]).unwrap().0,
            2.0 * PI
        );
        assert_eq!(
            app.world.get::<AngularVelocity>(asteroids[1]).unwrap().0,
            0.0
        );
    }

    #[test] // Has to be run with 'cargo test asteroids_dimension_2 -- --test-threads=1'
    #[ignore]
    fn asteroids_dimension_2() {
        let mut app = App::new();
        app.add_plugins(TestPlugins)
            .add_system(collision::generic::with::<asteroid::Asteroid>);

        let epsilon: f32 = 0.01;
        let health = Health(100);
        let radius: f32 = 100.0;
        let mass = Mass(PI * radius.powi(2));
        // Add epsilon so asteroids are not just tangent but intersect properly
        let x1 = -radius / 2.0f32.sqrt() + epsilon;
        let x2 = radius / 2.0f32.sqrt();

        let mut asteroids = Vec::new();
        asteroids.push(spawn_asteroid(
            &mut app,
            health,
            radius,
            Some(mass),
            Transform::from_xyz(x1, 0.0, 0.0),
            Velocity(Vec3::new(10.0, 0.0, 0.0)),
            AngularVelocity(0.0),
        ));
        asteroids.push(spawn_asteroid(
            &mut app,
            health,
            radius,
            Some(mass),
            Transform::from_xyz(x2, x2 - x1, 0.0),
            Velocity(Vec3::new(-30.0, 0.0, 0.0)),
            AngularVelocity(-PI),
        ));

        // let mut asteroids = Vec::new();
        // for (id, _) in app
        //     .world
        //     .query::<(Entity, (With<Asteroid>, Without<Part>))>()
        //     .iter(&app.world)
        // {
        //     asteroids.push(id);
        // }

        entity_print_static(&app, asteroids[0], Some(BLUE));
        entity_print_static(&app, asteroids[1], Some(RED));

        println!("PRE COLLISION");
        entity_print_dynamic(&app, asteroids[0], Some(BLUE));
        entity_print_dynamic(&app, asteroids[1], Some(RED));

        app.update();

        println!("POST COLLISION");
        entity_print_dynamic(&app, asteroids[0], Some(BLUE));
        entity_print_dynamic(&app, asteroids[1], Some(RED));

        assert!(
            (app.world.get::<Velocity>(asteroids[0]).unwrap().0 - Vec3::new(-10.0, -20.0, 0.0))
                .length()
                < epsilon
        );
        assert!(
            (app.world.get::<Velocity>(asteroids[1]).unwrap().0 - Vec3::new(-10.0, 20.0, 0.0))
                .length()
                < epsilon
        );
        assert!((app.world.get::<AngularVelocity>(asteroids[0]).unwrap().0 - 0.0).abs() < epsilon);
        assert!(
            (app.world.get::<AngularVelocity>(asteroids[1]).unwrap().0 - (-PI)).abs() < epsilon
        );
    }

    // TODO: Needs to be reworked now that time advances time.delta_seconds() per frame
    #[test]
    #[ignore]
    fn asteroid_spaceship() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins)
            .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
            .add_event::<SpacePressedEvent>()
            .add_startup_system(spawn_camera)
            .add_system(bevy::window::close_on_esc)
            .add_system(asteroid::spawn)
            .add_system(pause)
            .add_system_set(
                ConditionSet::new()
                    .run_on_event::<SpacePressedEvent>()
                    .label("Movement")
                    .with_system(asteroid::movement)
                    .with_system(move_spaceship)
                    .into(),
            )
            .add_system(
                collision::generic::among::<asteroid::Asteroid, spaceship::Spaceship>
                    // .run_on_event::<SpacePressedEvent>()
                    .after("Movement"),
            );

        let center_of_gravity = Vec3::new(100.0 / 3.0, 0.0, 0.0);
        let triangle = Triangle(
            Vec3::new(0.0, 100.0, 0.0) - center_of_gravity,
            Vec3::new(0.0, -100.0, 0.0) - center_of_gravity,
            Vec3::new(100.0, 0.0, 0.0) - center_of_gravity,
        );
        let radius: f32 = 100.0;
        let mass = Mass(PI * radius.powi(2));
        let va = Vec3::new(10.0, 0.0, 0.0);

        let asteroid = spawn_asteroid(
            &mut app,
            Health(100),
            radius,
            Some(mass),
            Transform::from_xyz(-radius, 0.0, 0.0),
            Velocity(va),
            AngularVelocity(PI / 10.0),
        );

        let vs = Vec3::ZERO;
        let avs = PI / 10.0;

        let spaceship = spawn_spaceship_triangle(
            &mut app,
            Health(100),
            None,
            None,
            triangle,
            Transform::from_translation(center_of_gravity + Vec3::new(0.0, 100.0, 0.0) - vs)
                .with_rotation(Quat::from_axis_angle(Vec3::Z, -avs)),
            Velocity(vs),
            AngularVelocity(avs),
        );

        entity_print_static(&app, asteroid, Some(BLUE));
        entity_print_static(&app, spaceship, Some(RED));

        println!("PRE COLLISION");
        entity_print_dynamic(&app, asteroid, Some(BLUE));
        entity_print_dynamic(&app, spaceship, Some(RED));

        // app.update();
        app.run();

        println!("POST COLLISION");
        entity_print_dynamic(&app, asteroid, Some(BLUE));
        entity_print_dynamic(&app, spaceship, Some(RED));

        // assert!(
        //     (app.world.get::<Velocity>(asteroids[0]).unwrap().0 - Vec3::new(-10.0, -20.0, 0.0))
        //         .length()
        //         < epsilon
        // );
        // assert!(
        //     (app.world.get::<Velocity>(asteroids[1]).unwrap().0 - Vec3::new(-10.0, 20.0, 0.0)).length()
        //         < epsilon
        // );
        // assert!(
        //     (app.world.get::<AngularVelocity>(asteroids[0]).unwrap().0 - angular_velocities[0].0).abs()
        //         < epsilon
        // );
        // assert!(
        //     (app.world.get::<AngularVelocity>(asteroids[1]).unwrap().0 - angular_velocities[1].0).abs()
        //         < epsilon
        // );
    }

    struct SpacePressedEvent;

    fn pause(input: Res<Input<KeyCode>>, mut ev_space: EventWriter<SpacePressedEvent>) {
        if input.just_pressed(KeyCode::Space) {
            ev_space.send(SpacePressedEvent);
        }
    }

    fn move_spaceship(
        mut query: Query<(&AngularVelocity, &mut Transform, &Velocity), With<Spaceship>>,
    ) {
        let (angular_velocity, mut transform, velocity) = query.single_mut();
        transform.translation += velocity.0;
        transform.rotation *= Quat::from_axis_angle(Vec3::Z, angular_velocity.0);
    }

    fn spawn_spaceship_triangle(
        app: &mut App,
        health: Health,
        maybe_mass: Option<Mass>,
        maybe_moment_of_inertia: Option<MomentOfInertia>,
        triangle: Triangle,
        transform: Transform,
        velocity: Velocity,
        angular_velocity: AngularVelocity,
    ) -> Entity {
        let mass = maybe_mass.unwrap_or_else(|| Mass(triangle.area()));
        let moment_of_inertia = maybe_moment_of_inertia
            .unwrap_or_else(|| MomentOfInertia(0.5 * mass.0 * triangle.area() / PI));

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices = triangle.to_array().map(|v| [v.x, v.y, v.z]).to_vec();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        let mesh_handle = app.world.resource_mut::<Assets<Mesh>>().add(mesh);

        let [mut hw, mut hh]: [f32; 2] = [0.0, 0.0];
        for vertex in triangle.to_array() {
            let Vec3 { x, y, z: _ } = transform
                .rotation
                .inverse()
                .mul_vec3(vertex - transform.translation);
            hw = hw.max(x.abs());
            hh = hh.max(y.abs());
        }

        let color = app
            .world
            .resource_mut::<Assets<ColorMaterial>>()
            .add(Color::RED.into())
            .clone();

        let spaceship_part = app
            .world
            .spawn((Spaceship, Part))
            .insert(health)
            .insert(Collider {
                aabb: Aabb { hw, hh },
                topology: Topology::Triangles {
                    mesh_handle: Mesh2dHandle(mesh_handle.clone_weak()),
                },
            })
            .insert(ColorMesh2dBundle {
                mesh: mesh_handle.into(),
                transform,
                material: color,
                ..Default::default()
            })
            .id();

        app.world
            .spawn(Spaceship)
            .insert(mass)
            .insert(moment_of_inertia)
            .insert(velocity)
            .insert(angular_velocity)
            .insert(SpatialBundle::default())
            .push_children(&[spaceship_part])
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
                vertices: 64,
            },
        )));
        let color = app
            .world
            .resource_mut::<Assets<ColorMaterial>>()
            .add(Color::BLUE.into())
            .clone();

        let asteroid_part = app
            .world
            .spawn((Asteroid, Part))
            .insert(health)
            .insert(Collider {
                aabb: Aabb {
                    hw: radius,
                    hh: radius,
                },
                topology: Topology::Disk { radius },
            })
            .insert(ColorMesh2dBundle {
                mesh: mesh.clone(),
                material: color,
                ..Default::default()
            })
            .id();

        app.world
            .spawn(Asteroid)
            .insert(mass)
            .insert(moment_of_inertia)
            .insert(velocity)
            .insert(angular_velocity)
            .insert(SpatialBundle {
                transform,
                ..Default::default()
            })
            .push_children(&[asteroid_part])
            .id()
    }

    fn entity_print_static(app: &App, entity: Entity, maybe_color: Option<&str>) {
        if let Some(color) = maybe_color {
            print!("{}", color);
        }
        let mass = app.world.get::<Mass>(entity).unwrap().0;
        let moment_of_inertia = app.world.get::<MomentOfInertia>(entity).unwrap().0;
        println!("mass: {}", mass);
        println!("moment of inertia: {}", moment_of_inertia);
        if maybe_color.is_some() {
            print!("{}", WHITE);
        }
    }

    fn entity_print_dynamic(app: &App, entity: Entity, maybe_color: Option<&str>) {
        if let Some(color) = maybe_color {
            print!("{}", color);
        }
        let Transform {
            translation,
            rotation,
            scale: _,
        } = app.world.get::<Transform>(entity).unwrap();
        let velocity = app.world.get::<Velocity>(entity).unwrap().0;
        let angular_velocity = app.world.get::<AngularVelocity>(entity).unwrap().0;
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
                .add(bevy::sprite::SpritePlugin::default())
            // .add(bevy::text::TextPlugin::default())
            // .add(bevy::ui::UiPlugin::default())
            // .add(bevy::pbr::PbrPlugin::default())
            // .add(bevy::gltf::GltfPlugin::default())
            // .add(bevy::audio::AudioPlugin::default())
            // .add(bevy::gilrs::GilrsPlugin::default())
            // .add(bevy::animation::AnimationPlugin::default())
        }
    }
}
