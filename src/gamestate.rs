use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    InGame,
    Paused,
}

pub fn pause(
    input: Res<Input<KeyCode>>,
    game_state: Res<CurrentState<GameState>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::P) {
        if game_state.0 == GameState::InGame {
            commands.insert_resource(NextState(GameState::Paused));
        } else {
            commands.insert_resource(NextState(GameState::InGame));
        }
    }
}
