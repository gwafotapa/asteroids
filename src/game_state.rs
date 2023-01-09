use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{keyboard::KeyboardBindings, Part};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    MainMenu,
    GameSetup,
    InGame,
    Paused,
    GameOver,
    TurnDownLight,
    TurnUpLight,
    Settings,
}

pub fn gamesetup_to_turnuplight(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::TurnUpLight));
}

pub fn ingame_to_paused(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query_spaceship: Query<(With<crate::spaceship::Spaceship>, Without<Part>)>,
    query_bindings: Query<&KeyboardBindings>,
) {
    if query_spaceship.get_single().is_ok()
        && input.any_just_pressed([KeyCode::Escape, query_bindings.single().pause()])
    {
        commands.insert_resource(NextState(GameState::Paused));
    }
}

pub fn gamesetup_or_ingame(current_state: Res<CurrentState<GameState>>) -> bool {
    current_state.0 == GameState::GameSetup || current_state.0 == GameState::InGame
}
