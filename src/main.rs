use asteroids::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

fn main() {
    // static SPAWN: &str = "spawn";
    // static DESPAWN: &str = "despawn";
    // static WRECK: &str = "wreck";
    static CLEANUP: &str = "cleanup";

    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Asteroids".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                resizable: false,
                // mode: WindowMode::Fullscreen,
                ..default()
            },
            ..default()
        }))
        // .add_stage_before(CoreStage::Update, SPAWN, SystemStage::parallel())
        .add_stage_after(CoreStage::Update, CLEANUP, SystemStage::parallel())
        // .add_stage_after(WRECK, DESPAWN, SystemStage::parallel())
        .add_loopless_state(GameState::MainMenu)
        // .add_event::<asteroid::AsteroidEvent>()
        .add_event::<blast::BlastEvent>()
        // .add_event::<collision::damages::DamageEvent>()
        .add_event::<collision::impact::ImpactEvent>()
        .add_event::<fire::FireEvent>()
        .add_event::<map::star::StarsEvent>()
        // .add_startup_system(camera::spawn)
        .add_startup_system(camera::spawn)
        .add_startup_system(keyboard::spawn_bindings)
        // .add_system(bevy::window::close_on_esc)
        .add_enter_system(GameState::MainMenu, ui::main_menu::spawn)
        .add_enter_system(GameState::Settings, ui::settings_menu::spawn)
        .add_enter_system(GameState::Paused, ui::pause_menu::spawn)
        .add_system(ui::main_menu::update.run_in_state(GameState::MainMenu))
        .add_system(ui::settings_menu::update.run_in_state(GameState::Settings))
        .add_system(ui::pause_menu::update.run_in_state(GameState::Paused))
        .add_enter_system_set(
            GameState::GameSetup,
            ConditionSet::new()
                // .run_in_state(GameState::GameSetup)
                .label("GameSetup -1")
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
                .with_system(health_bar::spawn)
                .with_system(game_state::gamesetup_to_turnuplight)
                // .with_system(map::star::spawn)
                .into(),
        )
        .add_system(map::star::spawn.run_if(game_state::gamesetup_or_ingame))
        .add_exit_system(GameState::GameSetup, light::kill)
        .add_enter_system(GameState::TurnUpLight, camera::setup)
        .add_system(light::turn_up.run_in_state(GameState::TurnUpLight))
        .add_system(light::turn_down.run_in_state(GameState::TurnDownLight))
        // .add_enter_system(GameState::InGame, intercepter::spawn)
        .add_exit_system(GameState::GameSetup, objective::spawn_text)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("free")
                .with_system(asteroid::spawn)
                .with_system(map::update)
                // .with_system(map::star::spawn)
                .with_system(game_state::ingame_to_paused)
                .with_system(wreckage::update)
                .with_system(wreckage::update_debris)
                .with_system(blast::update)
                .with_system(collision::impact::update)
                .with_system(intercepter::spawn)
                // .with_system(count_entities)
                // .with_system(count_asteroids)
                // .with_system(count_stars)
                // .with_system(count_wreckages)
                // .with_system(count_debris)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("movement")
                // .with_system(asteroid::asteroids)
                .with_system(boss::movement)
                .with_system(fire::movement)
                .with_system(spaceship::movement)
                .with_system(asteroid::update)
                .with_system(intercepter::movement)
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
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("collision")
                .after("movement")
                .with_system(collision::generic::with::<asteroid::Asteroid>)
                .with_system(collision::generic::among::<asteroid::Asteroid, boss::Boss>)
                .with_system(collision::generic::among::<asteroid::Asteroid, spaceship::Spaceship>)
                .with_system(
                    collision::generic::between::<asteroid::Asteroid, intercepter::Intercepter>,
                )
                .with_system(collision::generic::between::<asteroid::Asteroid, fire::Fire>)
                .with_system(collision::generic::among::<boss::Boss, spaceship::Spaceship>)
                .with_system(collision::generic::between::<boss::Boss, fire::Fire>)
                .with_system(collision::generic::between::<boss::Boss, intercepter::Intercepter>)
                .with_system(collision::generic::between::<spaceship::Spaceship, fire::Fire>)
                .with_system(
                    collision::generic::between::<spaceship::Spaceship, intercepter::Intercepter>,
                )
                .with_system(collision::generic::between::<intercepter::Intercepter, fire::Fire>)
                .with_system(collision::generic::with::<intercepter::Intercepter>)
                // .with_system(collision::generic::between::<fire::Fire, asteroid::Asteroid>)
                //     collision::generic::among::<asteroid::Asteroid, fire::Fire, spaceship::Spaceship>,
                // )
                // .with_system(collision::generic::with::<asteroid::Asteroid>)
                // .with_system(collision::asteroid_fire_spaceship)
                // .with_system(collision::with::<asteroid::Asteroid>)
                // .with_system(collision::between::<asteroid::Asteroid, fire::Fire>)
                // .with_system(collision::between::<asteroid::Asteroid, spaceship::Spaceship>)
                // .with_system(collision::with::<fire::Fire>)
                // .with_system(collision::between::<fire::Fire, spaceship::Spaceship>)
                // .with_system(collision::spaceship_and_asteroid)
                // .with_system(collision::asteroids_and_spaceship)
                // .with_system(collision::fire_and_asteroid_or_spaceship)
                // .with_system(collision::fire_and_asteroid)
                // .with_system(collision::fire_and_spaceship)
                // .with_system(collision::boss::with_fire)
                // .with_system(collision::spaceship_and_boss)
                // .with_system(collision::asteroid_and_asteroid)
                // .with_system(collision::boss_and_asteroid)
                // .with_system(collision::boss::with_asteroid_or_spaceship)
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
                .with_system(intercepter::attack)
                .into(),
        )
        .add_system(
            camera::update
                // .run_not_in_state(GameState::MainMenu)
                // .run_not_in_state(GameState::GameSetup)
                // .run_if(ingame_or_paused)
                // .run_if(spaceship_exists)
                .run_in_state(GameState::InGame)
                .label("camera")
                .after("movement"),
        ) // .after(spaceship::movement)
        .add_system(blast::spawn.after("attack"))
        .add_system(fire::spawn.after("attack"))
        // .add_system(collision::damages::apply.after("collision"))
        // .add_system(fire::impact.after(collision::damages::apply))
        .add_system(fire::impact.after("collision"))
        .add_system(collision::impact::spawn.after(fire::impact))
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                // .run_if(spaceship_exists)
                // .run_not_in_state(GameState::GameSetup)
                // .run_if(ingame_or_paused)
                .after("camera")
                .with_system(compass::update)
                .with_system(health_bar::update)
                .with_system(objective::update_text)
                .with_system(game_over::update_text)
                .into(),
        )
        // .add_exit_system(GameState::GameOver, exit_game)
        // Remove parent/children component of an entity whose relative is about to be despawned
        .add_system_set_to_stage(
            CLEANUP,
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                // .with_system(spaceship::before_despawn)
                .with_system(game_over::spawn_text)
                .with_system(boss::lone_core)
                // .with_system(wreckage::wreck_with::<boss::Boss>)
                .with_system(wreckage::wreck_with::<spaceship::Spaceship>)
                .with_system(wreckage::wreck_with::<asteroid::Asteroid>)
                .with_system(wreckage::wreck_with::<boss::Boss>)
                .with_system(wreckage::wreck_with::<intercepter::Intercepter>)
                // .with_system(asteroid::before_despawn)
                // .with_system(spaceship::wreck)
                // .with_system(asteroid::wreck)
                // .with_system(boss::wreck)
                // .with_system(wreckage::wreck_with::<boss::BossCore>)
                // .with_system(wreckage::wreck_with::<boss::BossEdge>)
                // .with_system(fire::wreck)
                // .with_system(debris::scale_down)
                .with_system(despawn_with::<blast::Blast>)
                // .with_system(despawn_with::<fire::Fire>)
                .with_system(fire::despawn)
                // .with_system(despawn_with::<asteroid::Asteroid>)
                // .with_system(despawn_with::<boss::BossCore>)
                // .with_system(despawn_with::<boss::BossEdge>)
                .with_system(despawn_with::<collision::impact::Impact>)
                // .with_system(despawn_recursive_with::<spaceship::Spaceship>)
                // .with_system(spaceship::flame::despawn)
                .with_system(despawn_recursive_with::<wreckage::Wreckage>)
                // .with_system(despawn)
                .into(),
        )
        .run();
}
