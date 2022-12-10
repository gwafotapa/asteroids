use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    MainMenu,
    GameSetup,
    InGame,
    Paused,
    // GameCleanup,
}

pub mod main_menu;
pub mod pause_menu;

pub fn from_gamesetup_to_ingame(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::InGame));
}

// Warning: Should generate some double despawn (with debris::update for example)
pub fn game_cleanup(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query_spaceship: Query<With<crate::spaceship::Spaceship>>,
    query_all: Query<Entity, Without<Camera>>,
    mut query_camera: Query<&mut UiCameraConfig>,
) {
    if query_spaceship.get_single().is_err() && input.any_just_pressed([KeyCode::Space, KeyCode::C])
    {
        for id in &query_all {
            commands.entity(id).despawn();
        }
        commands.insert_resource(NextState(GameState::MainMenu));
        query_camera.single_mut().show_ui = true;
    }
}
