use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use iyes_loopless::prelude::*;

use crate::{
    game_state::GameState,
    keyboard::{KeyboardBindings, BINDINGS, KEYCODESTRING},
};

const FONT: &str = "fonts/FiraSans-Bold.ttf";
const SIZE: f32 = 24.0;
const COLOR_HIGHLIGHTED: Color = Color::ORANGE_RED;
const COLOR_DEFAULT: Color = Color::GRAY;
const SETTINGS_MENU_ITEMS: usize = BINDINGS;
const BACKGROUND_COLOR: Color = Color::NONE;

#[derive(Clone, Component, Copy, Debug)]
pub struct SettingsMenu(pub usize);

#[derive(Clone, Component, Copy, Debug)]
pub struct SettingsMenuItems;

#[derive(Clone, Component, Copy, Debug)]
pub struct SettingsMenuItem;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query_bindings: Query<&KeyboardBindings>,
    mut query_settings: Query<&mut Style, With<SettingsMenu>>,
) {
    if let Ok(mut settings) = query_settings.get_single_mut() {
        settings.display = Display::Flex;
        return;
    }

    let font = asset_server.load(FONT);
    let item_style = Style {
        margin: UiRect {
            top: Val::Px(5.0),
            bottom: Val::Px(5.0),
            ..Default::default()
        },
        ..Default::default()
    };

    let settings_menu = commands
        .spawn(SettingsMenu(0))
        .insert(NodeBundle {
            background_color: BACKGROUND_COLOR.into(),
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let settings_menu_left = commands
        .spawn(NodeBundle {
            background_color: BACKGROUND_COLOR.into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                size: Size {
                    width: Val::Percent(20.0),
                    height: Val::Percent(100.0),
                },
                margin: UiRect {
                    left: Val::Percent(30.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    const LEFT_SECTIONS: [&str; 7] = [
        "Accelerate",
        "Decelerate",
        "Rotate left",
        "Rotate right",
        "Fire",
        "Switch camera position",
        "Pause",
    ];

    for left_section in LEFT_SECTIONS {
        let item = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    left_section,
                    TextStyle {
                        font: font.clone(),
                        font_size: SIZE,
                        color: COLOR_DEFAULT,
                    },
                ),
                style: item_style.clone(),
                ..Default::default()
            })
            .id();
        commands.entity(settings_menu_left).add_child(item);
    }

    let settings_menu_right = commands
        .spawn(SettingsMenuItems)
        .insert(NodeBundle {
            background_color: BACKGROUND_COLOR.into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size {
                    width: Val::Percent(20.0),
                    height: Val::Percent(100.0),
                },
                margin: UiRect {
                    right: Val::Percent(30.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    const RIGHT_SECTIONS: [&str; 7] = [
        "Up or ",
        "Down or ",
        "Left or ",
        "Right or ",
        "",
        "",
        "Esc or ",
    ];
    let bindings = query_bindings.single().0;

    for (right_section, key_code) in RIGHT_SECTIONS.into_iter().zip(bindings) {
        let item = commands
            .spawn(SettingsMenuItem)
            .insert(TextBundle {
                text: Text::from_sections([
                    TextSection::new(
                        right_section,
                        TextStyle {
                            font: font.clone(),
                            font_size: SIZE,
                            color: COLOR_DEFAULT,
                        },
                    ),
                    TextSection::new(
                        KEYCODESTRING[key_code as usize],
                        TextStyle {
                            font: font.clone(),
                            font_size: SIZE,
                            color: if key_code == bindings[0] {
                                COLOR_HIGHLIGHTED
                            } else {
                                COLOR_DEFAULT
                            },
                        },
                    ),
                ]),
                style: item_style.clone(),
                ..Default::default()
            })
            .id();
        commands.entity(settings_menu_right).add_child(item);
    }

    commands
        .entity(settings_menu)
        .push_children(&[settings_menu_left, settings_menu_right]);
}

pub fn update(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut query_menu: Query<(&mut SettingsMenu, &mut Style)>,
    query_menu_items: Query<&Children, With<SettingsMenuItems>>,
    mut query_item: Query<&mut Text, With<SettingsMenuItem>>,
    mut query_bindings: Query<&mut KeyboardBindings>,
    mut settings_state: Local<SettingsState>,
    mut keyboard_events: EventReader<KeyboardInput>,
    query_main_menu: Query<With<crate::ui::main_menu::MainMenu>>,
) {
    let mut bindings = query_bindings.single_mut();
    let (mut menu, mut style) = query_menu.single_mut();
    let children = query_menu_items.single();

    if input.just_pressed(KeyCode::Escape) {
        style.display = Display::None;
        commands.insert_resource(NextState(if query_main_menu.get_single().is_ok() {
            GameState::MainMenu
        } else {
            GameState::Paused
        }));
    }

    match *settings_state {
        SettingsState::SelectItem => {
            if input.any_just_pressed([KeyCode::Up, bindings.accelerate()]) && menu.0 > 0 {
                query_item.get_mut(children[menu.0]).unwrap().sections[1]
                    .style
                    .color = COLOR_DEFAULT;
                menu.0 -= 1;
                query_item.get_mut(children[menu.0]).unwrap().sections[1]
                    .style
                    .color = COLOR_HIGHLIGHTED;
            } else if input.any_just_pressed([KeyCode::Down, bindings.decelerate()])
                && menu.0 < SETTINGS_MENU_ITEMS - 1
            {
                query_item.get_mut(children[menu.0]).unwrap().sections[1]
                    .style
                    .color = COLOR_DEFAULT;
                menu.0 += 1;
                query_item.get_mut(children[menu.0]).unwrap().sections[1]
                    .style
                    .color = COLOR_HIGHLIGHTED;
            } else if input.any_just_pressed([KeyCode::Return, bindings.fire()]) {
                query_item.get_mut(children[menu.0]).unwrap().sections[1].value = "_".to_string();
                *settings_state = SettingsState::BindKey;
            }
        }
        SettingsState::BindKey => {
            if let Some(KeyboardInput {
                scan_code: _,
                key_code: Some(key_code),
                state: ButtonState::Pressed,
            }) = keyboard_events.iter().next()
            {
                if !PERMANENT_BINDINGS.iter().any(|k| k == key_code) {
                    let mut i = 0;
                    while i < BINDINGS {
                        if i != menu.0 && bindings.0[i] == *key_code {
                            bindings.0[i] = bindings.0[menu.0];
                            query_item.get_mut(children[i]).unwrap().sections[1].value =
                                KEYCODESTRING[bindings.0[menu.0] as usize].to_string();
                            // "_".to_string();
                            break;
                        }
                        i += 1;
                    }
                    bindings.0[menu.0] = *key_code;
                    query_item.get_mut(children[menu.0]).unwrap().sections[1].value =
                        KEYCODESTRING[*key_code as usize].to_string();
                    *settings_state = SettingsState::SelectItem;
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum SettingsState {
    #[default]
    SelectItem,
    BindKey,
}

const PERMANENT_BINDINGS: [KeyCode; 5] = [
    KeyCode::Up,     // Accelerate
    KeyCode::Down,   // Decelerate
    KeyCode::Left,   // Rotate left
    KeyCode::Right,  // Rotate right
    KeyCode::Escape, // Pause
];
