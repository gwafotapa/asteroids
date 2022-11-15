use asteroids::*;
use bevy::prelude::*;

fn main() {
    static DESPAWN: &str = "despawn";
    static REMOVE_COMPONENTS: &str = "remove components";

    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Asteroids".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                // present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_stage_after(
            CoreStage::Update,
            REMOVE_COMPONENTS,
            SystemStage::single_threaded(),
        )
        .add_stage_after(REMOVE_COMPONENTS, DESPAWN, SystemStage::single_threaded())
        .add_startup_system(camera)
        .add_startup_system(spaceship::spaceship)
        .add_startup_system(setup_level)
        .add_startup_system(star::setup_stars)
        .add_system(bevy::window::close_on_esc)
        .add_system(star::add_stars)
        .add_system(star::update_stars)
        .add_system(asteroid::asteroids)
        .add_system(keyboard_input)
        .add_system(collision::detect_collision_asteroid_asteroid)
        .add_system(collision::detect_collision_spaceship_asteroid)
        .add_system(collision::update_impacts)
        .add_system(collision::update_debris)
        .add_system(update_distance_to_boss)
        // .add_system(boss::add_boss)
        .add_system(boss::add_boss_parts)
        .add_system(boss::move_boss)
        // .add_system(boss::attack_boss)
        .add_system(boss::attack_boss_parts)
        // .add_system(collision::detect_collision_fire_boss)
        .add_system(move_fire)
        .add_system(collision::detect_collision_fire_asteroid)
        .add_system(collision::detect_collision_fire_boss_parts)
        .add_system(collision::detect_collision_fire_spaceship)
        .add_system(spaceship::explode)
        .add_system(despawn_blast)
        .add_system_to_stage(REMOVE_COMPONENTS, asteroid::explode)
        .add_system_to_stage(DESPAWN, asteroid::despawn)
        .add_system_to_stage(DESPAWN, collision::despawn_impacts)
        .add_system_to_stage(DESPAWN, despawn_fire)
        .run();
}
