use bevy::{app::AppExit, prelude::*};
use iyes_loopless::prelude::*;

use crate::{game_state::GameState, keyboard_bindings::KeyboardBindings};

const FONT: &str = "fonts/FiraSans-Bold.ttf";
const SIZE: f32 = 24.0;
const COLOR_HIGHLIGHTED: Color = Color::ORANGE_RED;
const COLOR_DEFAULT: Color = Color::GRAY;
const PAUSE_MENU_ITEMS: usize = 4;
const BACKGROUND_COLOR: Color = Color::NONE;

#[derive(Clone, Component, Copy, Debug)]
pub struct PauseMenu(pub usize);

#[derive(Clone, Component, Copy, Debug)]
pub struct PauseMenuItem;

pub fn spawn(
    mut commands: Commands,
    mut query_camera: Query<&mut UiCameraConfig>,
    mut query_menu: Query<&mut Style, With<PauseMenu>>,
    asset_server: Res<AssetServer>,
) {
    query_camera.single_mut().show_ui = true;
    if let Ok(mut pause_menu) = query_menu.get_single_mut() {
        pause_menu.display = Display::Flex;
        return;
    }

    let font = asset_server.load(FONT);
    let item_style = Style {
        margin: UiRect::all(Val::Px(10.0)),
        ..Default::default()
    };

    let pause_menu = commands
        .spawn(PauseMenu(0))
        .insert(NodeBundle {
            background_color: BACKGROUND_COLOR.into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    const SECTIONS: [&str; PAUSE_MENU_ITEMS] = ["Resume", "Settings", "Exit game", "Quit"];

    for section in SECTIONS {
        let item = commands
            .spawn(PauseMenuItem)
            .insert(TextBundle {
                text: Text::from_section(
                    section,
                    TextStyle {
                        font: font.clone(),
                        font_size: SIZE,
                        color: if section == SECTIONS[0] {
                            COLOR_HIGHLIGHTED
                        } else {
                            COLOR_DEFAULT
                        },
                    },
                ),
                style: item_style.clone(),
                ..Default::default()
            })
            .id();
        commands.entity(pause_menu).add_child(item);
    }
}

pub fn update(
    mut commands: Commands,
    mut query_camera: Query<&mut UiCameraConfig>,
    mut query_item: Query<&mut Text, With<PauseMenuItem>>,
    mut query_menu_pause: Query<(&Children, &mut PauseMenu, &mut Style)>,
    mut exit: EventWriter<AppExit>,
    input: Res<Input<KeyCode>>,
    query_bindings: Query<&KeyboardBindings>,
) {
    let (children, mut menu, mut style) = query_menu_pause.single_mut();
    let bindings = query_bindings.single();
    if input.any_just_pressed([KeyCode::Escape, bindings.pause()]) {
        commands.insert_resource(NextState(GameState::InGame));
        query_camera.single_mut().show_ui = false;
    } else if input.any_just_pressed([KeyCode::Up, bindings.accelerate()]) {
        if menu.0 > 0 {
            query_item.get_mut(children[menu.0]).unwrap().sections[0]
                .style
                .color = COLOR_DEFAULT;
            menu.0 -= 1;
            query_item.get_mut(children[menu.0]).unwrap().sections[0]
                .style
                .color = COLOR_HIGHLIGHTED;
        }
    } else if input.any_just_pressed([KeyCode::Down, bindings.decelerate()]) {
        if menu.0 < PAUSE_MENU_ITEMS - 1 {
            query_item.get_mut(children[menu.0]).unwrap().sections[0]
                .style
                .color = COLOR_DEFAULT;
            menu.0 += 1;
            query_item.get_mut(children[menu.0]).unwrap().sections[0]
                .style
                .color = COLOR_HIGHLIGHTED;
        }
    } else if input.any_just_pressed([KeyCode::Return, bindings.fire()]) {
        match menu.0 {
            0 => {
                commands.insert_resource(NextState(GameState::InGame));
                style.display = Display::None;
                query_camera.single_mut().show_ui = false;
            }
            1 => {
                commands.insert_resource(NextState(GameState::Settings));
                style.display = Display::None;
            }
            2 => {
                commands.insert_resource(NextState(GameState::TurnDownLight));
            }
            3 => {
                exit.send(AppExit);
            }
            _ => unreachable!(),
        }
    }
}
