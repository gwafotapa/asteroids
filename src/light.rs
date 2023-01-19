use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{game_state::GameState, keyboard_bindings::KeyboardBindings};

const DIM_FACTOR: f32 = 0.92;
const DIM_TIMER: u32 = 50;

pub fn turn_down(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_camera: Query<(&mut Camera, &mut UiCameraConfig)>,
    mut query_visible_mesh: Query<(&Handle<ColorMaterial>, &ComputedVisibility)>,
    mut query_visible_text: Query<(&ComputedVisibility, &mut Text)>,
    mut timer: Local<u32>,
    query_main_menu: Query<Entity, With<super::ui::main_menu::MainMenu>>,
    query_reset: Query<Entity, (Without<KeyboardBindings>, Without<Camera>)>,
    query_settings_menu: Query<Entity, With<super::ui::settings_menu::SettingsMenu>>,
) {
    for (color_material, visibility) in &mut query_visible_mesh {
        if visibility.is_visible() {
            materials.get_mut(color_material).unwrap().color *=
                [DIM_FACTOR, DIM_FACTOR, DIM_FACTOR];
        }
    }

    for (visibility, mut text) in &mut query_visible_text {
        if visibility.is_visible() {
            text.sections[0].style.color *= [DIM_FACTOR, DIM_FACTOR, DIM_FACTOR];
        }
    }

    let (mut camera, mut config) = query_camera.single_mut();
    *timer += 1;
    if *timer == DIM_TIMER {
        if let Ok(main_menu) = query_main_menu.get_single() {
            commands.entity(main_menu).despawn_recursive();
            if let Ok(settings_menu) = query_settings_menu.get_single() {
                commands.entity(settings_menu).despawn_recursive();
            }
            commands.insert_resource(NextState(GameState::GameSetup));
            camera.is_active = false;
            config.show_ui = false;
        } else {
            for id in &query_reset {
                commands.entity(id).despawn();
            }
            commands.insert_resource(NextState(GameState::MainMenu));
            config.show_ui = true;
        }
        *timer = 0;
    }
}

pub fn turn_up(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_visible_mesh: Query<(&Handle<ColorMaterial>, &ComputedVisibility)>,
    mut query_visible_text: Query<(&ComputedVisibility, &mut Text)>,
    mut timer: Local<u32>,
) {
    for (color_material, visibility) in &mut query_visible_mesh {
        if visibility.is_visible() {
            materials.get_mut(color_material).unwrap().color *=
                [1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR];
        }
    }

    for (visibility, mut text) in &mut query_visible_text {
        if visibility.is_visible() {
            text.sections[0].style.color *= [1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR, 1.0 / DIM_FACTOR];
        }
    }

    *timer += 1;
    if *timer == DIM_TIMER {
        commands.insert_resource(NextState(GameState::InGame));
        *timer = 0;
    }
}

pub fn kill(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_visible_mesh: Query<(&Handle<ColorMaterial>, &ComputedVisibility)>,
    mut query_visible_text: Query<(&ComputedVisibility, &mut Text)>,
) {
    let kill_factor = DIM_FACTOR.powi(DIM_TIMER as i32);

    for (color_material, visibility) in &mut query_visible_mesh {
        if visibility.is_visible() {
            materials.get_mut(color_material).unwrap().color *=
                [kill_factor, kill_factor, kill_factor];
        }
    }

    for (visibility, mut text) in &mut query_visible_text {
        if visibility.is_visible() {
            text.sections[0].style.color *= [kill_factor, kill_factor, kill_factor];
        }
    }
}
