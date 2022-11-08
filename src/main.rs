use asteroids::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Asteroids".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)
        .add_startup_system(spaceship::spaceship)
        .add_startup_system(setup_level)
        .add_startup_system(star::setup_stars)
        .add_system(bevy::window::close_on_esc)
        .add_system(star::add_stars)
        .add_system(star::update_stars)
        .add_system(asteroid::asteroids)
        .add_system(keyboard_input)
        .add_system(collision::detect_collision_spaceship_asteroid)
        // .add_system(update_fires)
        .add_system(collision::update_impacts)
        .add_system(collision::detect_collision_fire_asteroid)
        .add_system(collision::update_debris)
        .add_system(update_distance_to_boss)
        .add_system(boss::add_boss)
        // .add_system(boss::add_boss_2)
        .add_system(boss::move_boss)
        .add_system(boss::attack_boss)
        .add_system(update_fire)
        .add_system(collision::detect_collision_fire_boss)
        .add_system(collision::detect_collision_fire_spaceship)
        .add_system(despawn_blast)
        // .add_system_to_stage(
        //     CoreStage::PostUpdate,
        //     debug_globaltransform.after(TransformSystem::TransformPropagate),
        // )
        // .add_startyp_system(test)
        .run();
}
