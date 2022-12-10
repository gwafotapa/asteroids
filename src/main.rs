use asteroids::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

fn main() {
    // static SPAWN: &str = "spawn";
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
        // .add_stage_before(CoreStage::Update, SPAWN, SystemStage::parallel())
        .add_stage_after(CoreStage::Update, CLEANUP, SystemStage::parallel())
        .add_stage_after(CLEANUP, DESPAWN, SystemStage::parallel())
        .add_startup_system(camera::spawn)
        .add_loopless_state(game_state::GameState::MainMenu)
        .add_enter_system(GameState::MainMenu, game_state::main_menu::spawn)
        .add_system(bevy::window::close_on_esc)
        .add_system(game_state::main_menu::update.run_in_state(GameState::MainMenu))
        .add_enter_system_set(
            GameState::GameSetup,
            ConditionSet::new()
                // .run_in_state(GameState::GameSetup)
                .label("GameSetup -1")
                .with_system(game_state::pause_menu::spawn)
                .with_system(spaceship::spawn)
                .with_system(boss::spawn)
                .with_system(map::setup)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::GameSetup)
                .label("GameSetup")
                .with_system(spaceship::flame::front_spawn)
                .with_system(spaceship::flame::rear_spawn)
                .with_system(compass::setup)
                .with_system(game_state::from_gamesetup_to_ingame)
                .into(),
        )
        .add_system(game_state::pause_menu::in_game.run_in_state(GameState::InGame))
        .add_system(game_state::pause_menu::paused.run_in_state(GameState::Paused))
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("free")
                .with_system(map::update)
                .with_system(debris::update)
                .with_system(dim_light)
                .with_system(game_state::game_cleanup)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("movement")
                // .with_system(asteroid::asteroids)
                .with_system(boss::movement)
                .with_system(fire::update)
                .with_system(spaceship::movement)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .after("movement") // .after(spaceship::movement)
                .with_system(spaceship::flame::front_update)
                .with_system(spaceship::flame::rear_update)
                .into(),
        )
        .add_system(
            collision::impact::update // .after(boss::movement).after(spaceship::movement)
                .run_in_state(GameState::InGame)
                .after("movement"),
        ) // Stage of this and despawn ?
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("collision")
                .after("movement")
                .with_system(collision::spaceship_and_asteroid)
                .with_system(collision::fire_and_asteroid)
                .with_system(collision::fire_and_boss)
                .with_system(collision::fire_and_spaceship)
                .with_system(collision::spaceship_and_boss)
                // .with_system(collision::asteroid_and_asteroid),
                // .with_system(collision::boss_and_asteroid),
                // .with_system(collision::fire_and_fire),
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("attack")
                .after("movement")
                .with_system(spaceship::attack) // .after(spaceship::movement)
                .with_system(boss::attack) // .after(boss::movement)
                .into(),
        )
        .add_system(
            camera::update
                .run_not_in_state(GameState::MainMenu)
                .run_not_in_state(GameState::GameSetup)
                .label("camera")
                .after("movement"),
        ) // .after(spaceship::movement)
        .add_system(
            compass::update
                .run_in_state(GameState::InGame)
                .after("camera"),
        )
        .add_system_set_to_stage(
            CLEANUP,
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(spaceship::explode)
                .with_system(asteroid::explode) // this and despawn maybe not at this stage as long as there are no impact child.
                .with_system(boss::explode)
                .with_system(blast::update)
                // .with_system(fire::explode)
                .into(),
        )
        .add_system_set_to_stage(
            DESPAWN,
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(asteroid::despawn)
                .with_system(boss::despawn)
                .with_system(spaceship::despawn)
                .with_system(collision::impact::despawn)
                .with_system(fire::despawn) // Not necessarily at this stage (not a child)
                .with_system(blast::despawn)
                .into(),
        )
        .run();
}
