use asteroids::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

fn main() {
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
        .add_stage_after(CoreStage::Update, CLEANUP, SystemStage::parallel())
        .add_loopless_state(GameState::MainMenu)
        .add_event::<BlastEvent>()
        .add_event::<ImpactEvent>()
        .add_event::<FireEvent>()
        .add_event::<StarsEvent>()
        .add_startup_system(camera::spawn)
        .add_startup_system(keyboard_bindings::spawn)
        .add_enter_system(GameState::MainMenu, ui::main_menu::spawn)
        .add_enter_system(GameState::Settings, ui::settings_menu::spawn)
        .add_enter_system(GameState::Paused, ui::pause_menu::spawn)
        .add_system(ui::main_menu::update.run_in_state(GameState::MainMenu))
        .add_system(ui::settings_menu::update.run_in_state(GameState::Settings))
        .add_system(ui::pause_menu::update.run_in_state(GameState::Paused))
        .add_enter_system_set(
            GameState::GameSetup,
            ConditionSet::new()
                .with_system(spaceship::spawn)
                .with_system(boss::spawn)
                .with_system(map::spawn)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::GameSetup)
                .with_system(spaceship::flame::front_spawn)
                .with_system(spaceship::flame::rear_spawn)
                .with_system(compass::spawn)
                .with_system(health_bar::spawn)
                .with_system(game_state::gamesetup_to_turnuplight)
                .into(),
        )
        .add_system(star::spawn.run_if(game_state::gamesetup_or_ingame))
        .add_exit_system(GameState::GameSetup, light::kill)
        .add_enter_system(GameState::TurnUpLight, camera::setup)
        .add_system(light::turn_up.run_in_state(GameState::TurnUpLight))
        .add_system(light::turn_down.run_in_state(GameState::TurnDownLight))
        .add_exit_system(GameState::GameSetup, objective::spawn_text)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("free")
                .with_system(game_state::ingame_to_paused)
                .with_system(asteroid::spawn)
                .with_system(intercepter::spawn)
                .with_system(blast::update)
                .with_system(impact::update)
                .with_system(map::update)
                .with_system(wreckage::update)
                .with_system(wreckage::update_debris)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("movement")
                .with_system(boss::movement)
                .with_system(fire::movement)
                .with_system(spaceship::movement)
                .with_system(asteroid::movement)
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
                .with_system(collision::generic::with::<Asteroid>)
                .with_system(collision::generic::with::<Intercepter>)
                .with_system(collision::generic::between::<Asteroid, Boss>)
                .with_system(collision::generic::between::<Asteroid, Fire>)
                .with_system(collision::generic::between::<Asteroid, Intercepter>)
                .with_system(collision::generic::between::<Asteroid, Spaceship>)
                .with_system(collision::generic::between::<Boss, Fire>)
                .with_system(collision::generic::between::<Boss, Intercepter>)
                .with_system(collision::generic::between::<Boss, Spaceship>)
                .with_system(collision::generic::between::<Spaceship, Fire>)
                .with_system(collision::generic::between::<Spaceship, Intercepter>)
                .with_system(collision::generic::between::<Intercepter, Fire>)
                .into(),
        )
        .add_system(
            fire::impact
                .run_in_state(GameState::InGame)
                .label("impact event")
                .after("collision"),
        )
        .add_system(
            impact::spawn
                .run_in_state(GameState::InGame)
                .after("impact event"),
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
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .after("attack")
                .with_system(blast::spawn)
                .with_system(fire::spawn)
                .into(),
        )
        .add_system(
            camera::update
                .run_in_state(GameState::InGame)
                .label("camera")
                .after("movement"),
        ) // .after(spaceship::movement)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .after("camera")
                .with_system(compass::update)
                .with_system(health_bar::update)
                .with_system(objective::update_text)
                .with_system(game_over::update_text)
                .into(),
        )
        .add_system_set_to_stage(
            CLEANUP,
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(game_over::spawn_text)
                .with_system(boss::lone_core)
                .with_system(wreckage::wreck_with::<Spaceship>)
                .with_system(wreckage::wreck_with::<Asteroid>)
                .with_system(wreckage::wreck_with::<Boss>)
                .with_system(wreckage::wreck_with::<Intercepter>)
                .with_system(despawn::with::<Blast>)
                .with_system(despawn::with::<Impact>)
                .with_system(despawn::recursive_with::<Wreckage>)
                .with_system(fire::despawn)
                .into(),
        )
        .run();
}
