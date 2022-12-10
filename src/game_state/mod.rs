use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    MainMenu,
    PreGame,
    InGame,
    Paused,
}

pub mod main_menu;
pub mod pause_menu;

pub fn from_pregame_to_ingame(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::InGame));
}
