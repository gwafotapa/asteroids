use asteroids::{spaceship, *};
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
        .add_startup_system(setup_stars)
        .add_system(add_stars)
        .add_system(update_stars)
        .add_system(asteroids)
        .add_system(keyboard_input)
        .add_system(detect_collision_spaceship_asteroid)
        .add_system(update_bullets)
        .add_system(update_impacts)
        .add_system(detect_collision_bullet_asteroid)
        .add_system(update_debris)
        .add_system(update_distance_to_boss)
        // .add_system(add_boss)
        .add_system(add_boss_2)
        .add_system(move_boss)
        .add_system(attack_boss)
        .add_system(update_boss_bullets)
        .add_system(detect_collision_bullet_boss)
        .add_system(bevy::window::close_on_esc)
        // .add_system_to_stage(
        //     CoreStage::PostUpdate,
        //     debug_globaltransform.after(TransformSystem::TransformPropagate),
        // )
        // .add_startyp_system(test)
        .run();
}
