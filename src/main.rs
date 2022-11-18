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
                // present_mode: PresentMode::AutoVsync,
                // mode: WindowMode::SizedFullscreen,
                ..default()
            },
            ..default()
        }))
        .add_stage_after(CoreStage::Update, CLEANUP, SystemStage::single_threaded())
        .add_stage_after(CLEANUP, DESPAWN, SystemStage::single_threaded())
        .add_startup_system(camera)
        .add_startup_system(spaceship::spaceship)
        .add_startup_system(setup_level)
        .add_startup_system(star::setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(update_distance_to_boss)
        .add_system_set(
            SystemSet::new()
                .label("movement")
                .with_system(asteroid::asteroids)
                .with_system(boss::move_boss)
                .with_system(move_fire)
                .with_system(keyboard_input),
        )
        .add_system_set(
            SystemSet::new()
                .label("collision")
                .after("movement")
                .with_system(collision::detect_collision_asteroid_asteroid)
                .with_system(collision::detect_collision_spaceship_asteroid)
                .with_system(collision::detect_collision_fire_asteroid)
                .with_system(collision::detect_collision_fire_boss_part)
                .with_system(collision::detect_collision_fire_spaceship),
        )
        .add_system(star::spawn)
        .add_system(star::update)
        .add_system(collision::update_impacts)
        .add_system(collision::update_debris)
        // .add_system(boss::add_boss)
        .add_system(boss::add_boss_parts)
        // .add_system(boss::attack_boss.after(boss::move_boss))
        .add_system(boss::attack_boss_parts.after(boss::move_boss))
        // .add_system(collision::detect_collision_fire_boss)
        .add_system_to_stage(CLEANUP, spaceship::explode)
        .add_system_to_stage(CLEANUP, asteroid::explode)
        .add_system_to_stage(CLEANUP, boss::explode)
        .add_system_to_stage(CLEANUP, update_blast)
        .add_system_to_stage(DESPAWN, asteroid::despawn)
        .add_system_to_stage(DESPAWN, boss::despawn)
        .add_system_to_stage(DESPAWN, spaceship::despawn)
        .add_system_to_stage(DESPAWN, collision::despawn_impacts)
        .add_system_to_stage(DESPAWN, despawn_fire)
        .add_system_to_stage(DESPAWN, despawn_blast)
        .run();
}
