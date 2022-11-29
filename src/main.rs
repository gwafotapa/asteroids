use asteroids::*;
use bevy::prelude::*;

fn main() {
    static DESPAWN: &str = "despawn";
    static CLEANUP: &str = "cleanup";

    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Asteroids".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                // mode: WindowMode::SizedFullscreen,
                ..default()
            },
            ..default()
        }))
        // .add_plugin(flame::ColoredMesh2dPlugin)
        .add_stage_after(CoreStage::Update, CLEANUP, SystemStage::parallel())
        // .add_stage_after(CLEANUP, DESPAWN, SystemStage::single_threaded())
        .add_stage_after(CLEANUP, DESPAWN, SystemStage::parallel())
        .add_startup_system(camera)
        .add_startup_system(spaceship::spawn)
        .add_startup_system(boss::spawn)
        .add_startup_system_to_stage(StartupStage::PostStartup, spaceship::flame::spawn)
        .add_startup_system_to_stage(StartupStage::PostStartup, compass::setup)
        .add_startup_system(map::setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(map::update)
        .add_system(spaceship::flame::update)
        .add_system(compass::update.after(keyboard_input))
        .add_system_set(
            SystemSet::new()
                .label("movement")
                // .with_system(asteroid::asteroids)
                // .with_system(boss::advance)
                .with_system(fire::update)
                .with_system(keyboard_input),
        )
        .add_system_set(
            SystemSet::new()
                .label("collision")
                .after("movement")
                // .with_system(collision::asteroid_and_asteroid)
                .with_system(collision::spaceship_and_asteroid)
                .with_system(collision::fire_and_asteroid),
            // .with_system(collision::fire_and_boss)
            // .with_system(collision::fire_and_spaceship),
        )
        .add_system(spaceship::attack.after("movement"))
        .add_system(collision::impact::update) // Stage of this and despawn ?
        .add_system(debris::update)
        // .add_system(boss::attack.after(boss::advance))
        .add_system_to_stage(CLEANUP, spaceship::explode)
        .add_system_to_stage(CLEANUP, asteroid::explode) // this and despawn maybe not at this stage as long as there are no impact child.
        // .add_system_to_stage(CLEANUP, boss::explode)
        .add_system_to_stage(CLEANUP, blast::update)
        .add_system_to_stage(DESPAWN, asteroid::despawn)
        // .add_system_to_stage(DESPAWN, boss::despawn)
        .add_system_to_stage(DESPAWN, spaceship::despawn)
        .add_system_to_stage(DESPAWN, collision::impact::despawn)
        .add_system_to_stage(DESPAWN, fire::despawn) // Not necessarily at this stage (not a child)
        .add_system_to_stage(DESPAWN, blast::despawn)
        .run();
}
