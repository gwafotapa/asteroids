#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub use crate::{
    asteroid::Asteroid,
    blast::{Blast, BlastEvent},
    boss::Boss,
    collision::impact::{Impact, ImpactEvent},
    component::*,
    constant::{WINDOW_HEIGHT, WINDOW_WIDTH, WINDOW_Z},
    fire::{Fire, FireEvent},
    game_state::GameState,
    intercepter::Intercepter,
    map::star::StarsEvent,
    spaceship::Spaceship,
    wreckage::Wreckage,
};

pub mod asteroid;
pub mod blast;
pub mod boss;
pub mod camera;
pub mod collision;
pub mod compass;
pub mod component;
pub mod constant;
pub mod despawn;
pub mod fire;
pub mod game_over;
pub mod game_state;
pub mod health_bar;
pub mod intercepter;
pub mod keyboard_bindings;
pub mod light;
pub mod map;
pub mod objective;
pub mod spaceship;
pub mod transform;
pub mod ui;
pub mod wreckage;
