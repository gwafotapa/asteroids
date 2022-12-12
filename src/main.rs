use asteroids::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

fn main() {
    // static SPAWN: &str = "spawn";
    static DESPAWN: &str = "despawn";
    static BEFORE_DESPAWN: &str = "cleanup";

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
        .add_stage_after(CoreStage::Update, BEFORE_DESPAWN, SystemStage::parallel())
        .add_stage_after(BEFORE_DESPAWN, DESPAWN, SystemStage::parallel())
        .add_loopless_state(GameState::MainMenu)
        .add_startup_system(camera::spawn)
        .add_enter_system(GameState::MainMenu, ui::main_menu::spawn)
        .add_system(bevy::window::close_on_esc)
        .add_system(ui::main_menu::update.run_in_state(GameState::MainMenu))
        .add_enter_system_set(
            GameState::GameSetup,
            ConditionSet::new()
                // .run_in_state(GameState::GameSetup)
                .label("GameSetup -1")
                .with_system(ui::pause_menu::spawn)
                .with_system(spaceship::spawn)
                .with_system(boss::spawn)
                .with_system(map::spawn)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::GameSetup)
                .label("GameSetup")
                .with_system(spaceship::flame::front_spawn)
                .with_system(spaceship::flame::rear_spawn)
                .with_system(compass::spawn)
                .with_system(from_gamesetup_to_ingame)
                .into(),
        )
        .add_system(ui::pause_menu::paused.run_in_state(GameState::Paused))
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("free")
                .with_system(map::update)
                .with_system(ui::pause_menu::in_game)
                .with_system(dim_light)
                .with_system(game_cleanup)
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
        // Remove parent/children component of an entity whose relative is about to be despawned
        .add_system_set_to_stage(
            BEFORE_DESPAWN,
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(spaceship::before_despawn)
                .with_system(blast::before_despawn)
                .with_system(boss::before_despawn)
                // .with_system(asteroid::before_despawn)
                .into(),
        )
        .add_system_set_to_stage(
            DESPAWN,
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(spaceship::explode)
                .with_system(asteroid::explode)
                .with_system(boss::explode)
                // .with_system(fire::explode)
                .with_system(debris::scale_down)
                .with_system(despawn_with::<blast::Blast>)
                .with_system(despawn_with::<fire::Fire>)
                .with_system(despawn_with::<asteroid::Asteroid>)
                .with_system(despawn_with::<boss::BossCore>)
                .with_system(despawn_with::<boss::BossEdge>)
                .with_system(despawn_with::<collision::impact::Impact>)
                .with_system(despawn_recursive_with::<spaceship::Spaceship>)
                // .with_system(despawn)
                .into(),
        )
        .run();
}
