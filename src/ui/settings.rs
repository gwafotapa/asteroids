use bevy::{
    app::AppExit,
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use iyes_loopless::prelude::*;

use crate::GameState;

const FONT: &str = "fonts/FiraSans-Bold.ttf";
const SIZE: f32 = 24.0;
const COLOR_HIGHLIGHTED: Color = Color::ORANGE_RED;
const COLOR_DEFAULT: Color = Color::GRAY;
const SETTINGS_MENU_ITEMS: usize = BINDINGS;
const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(Clone, Component, Copy, Debug)]
pub struct SettingsMenu(pub usize);

#[derive(Clone, Component, Copy, Debug)]
pub struct SettingsMenuItem;

#[derive(Clone, Component, Copy, Debug)]
pub struct SettingsMenuText;

#[derive(Clone, Component, Copy, Debug)]
pub struct SettingsMenuTextItem;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&KeyboardBindings>,
) {
    let font = asset_server.load(FONT);
    let item_style = Style {
        margin: UiRect::all(Val::Px(10.0)),
        ..Default::default()
    };

    let settings_menu_left = commands
        .spawn(SettingsMenuText)
        .insert(NodeBundle {
            // background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
            background_color: BACKGROUND_COLOR.into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                // margin: UiRect::all(Val::Auto),
                margin: UiRect {
                    left: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Auto,
                    right: Val::Percent(5.0),
                    ..default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let accelerate = commands
        .spawn(SettingsMenuTextItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Accelerate",
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

    let decelerate = commands
        .spawn(SettingsMenuTextItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Decelerate",
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

    let rotate_left = commands
        .spawn(SettingsMenuTextItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Rotate left",
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

    let rotate_right = commands
        .spawn(SettingsMenuTextItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Rotate right",
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

    let fire = commands
        .spawn(SettingsMenuTextItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Fire",
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

    let camera = commands
        .spawn(SettingsMenuTextItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Switch camera mode",
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

    commands.entity(settings_menu_left).push_children(&[
        accelerate,
        decelerate,
        rotate_left,
        rotate_right,
        fire,
        camera,
    ]);

    let settings_menu_right = commands
        .spawn(SettingsMenu(0))
        .insert(NodeBundle {
            // background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
            background_color: BACKGROUND_COLOR.into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                // margin: UiRect::all(Val::Auto),
                margin: UiRect {
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Auto,
                    left: Val::Percent(5.0),
                    // right: Val::Auto,
                    ..default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let bindings = query.single().0;

    for key_code in bindings {
        let item = commands
            .spawn(SettingsMenuItem)
            .insert(TextBundle {
                text: Text::from_section(
                    KeyCodeString[key_code as usize],
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
                style: item_style.clone(),
                ..Default::default()
            })
            .id();
        commands.entity(settings_menu_right).add_child(item);
    }
}

pub fn spawn_bindings(mut commands: Commands) {
    commands.spawn(KeyboardBindings::default());
}

pub fn update(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    game_state: Res<CurrentState<GameState>>,
    query_text_menu: Query<Entity, With<SettingsMenuText>>,
    mut query_menu: Query<(&Children, Entity, &mut SettingsMenu, &mut Style)>,
    mut query_item: Query<&mut Text, With<SettingsMenuItem>>,
    mut query_bindings: Query<&mut KeyboardBindings>,
    mut settings_state: Local<SettingsState>,
    mut keyboard_events: EventReader<KeyboardInput>,
) {
    if game_state.0 != GameState::Settings {
        panic!("Wrong game state. Should be Settings.");
    }

    let mut bindings = query_bindings.single_mut();
    let (children, id, mut menu, mut style) = query_menu.single_mut();
    if input.just_pressed(KeyCode::Escape) {
        commands.entity(id).despawn_recursive();
        commands
            .entity(query_text_menu.single())
            .despawn_recursive();
        commands.insert_resource(NextState(GameState::MainMenu));
    }
    match *settings_state {
        SettingsState::SelectItem => {
            if input.any_just_pressed([KeyCode::Up, bindings.accelerate()]) && menu.0 > 0 {
                query_item.get_mut(children[menu.0]).unwrap().sections[0]
                    .style
                    .color = COLOR_DEFAULT;
                menu.0 -= 1;
                query_item.get_mut(children[menu.0]).unwrap().sections[0]
                    .style
                    .color = COLOR_HIGHLIGHTED;
            } else if input.any_just_pressed([KeyCode::Down, bindings.decelerate()])
                && menu.0 < SETTINGS_MENU_ITEMS - 1
            {
                query_item.get_mut(children[menu.0]).unwrap().sections[0]
                    .style
                    .color = COLOR_DEFAULT;
                menu.0 += 1;
                query_item.get_mut(children[menu.0]).unwrap().sections[0]
                    .style
                    .color = COLOR_HIGHLIGHTED;
            } else if input.any_just_pressed([KeyCode::Return, bindings.fire()]) {
                query_item.get_mut(children[menu.0]).unwrap().sections[0].value = "_".to_string();
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
                let mut i = 0;
                while i < BINDINGS {
                    if i != menu.0 && bindings.0[i] == *key_code {
                        bindings.0[i] = bindings.0[menu.0];
                        query_item.get_mut(children[i]).unwrap().sections[0].value =
                            "_".to_string();
                        break;
                    }
                    i += 1;
                }
                bindings.0[menu.0] = *key_code;
                query_item.get_mut(children[menu.0]).unwrap().sections[0].value =
                    KeyCodeString[*key_code as usize].to_string();
                *settings_state = SettingsState::SelectItem;
            }
        }
    }
}

const BINDINGS: usize = 6;

#[derive(Default)]
pub enum SettingsState {
    #[default]
    SelectItem,
    BindKey,
}

#[derive(Component)]
pub struct KeyboardBindings([KeyCode; BINDINGS]);

impl Default for KeyboardBindings {
    fn default() -> KeyboardBindings {
        KeyboardBindings([
            KeyCode::O,
            KeyCode::L,
            KeyCode::K,
            KeyCode::M,
            KeyCode::R,
            KeyCode::Space,
        ])
    }
}

impl KeyboardBindings {
    // fn get(&self) -> &[KeyCode] {
    //     &self.0
    // }

    fn accelerate(&self) -> KeyCode {
        self.0[0]
    }

    fn decelerate(&self) -> KeyCode {
        self.0[1]
    }

    fn rotate_left(&self) -> KeyCode {
        self.0[2]
    }

    fn rotate_right(&self) -> KeyCode {
        self.0[3]
    }

    fn fire(&self) -> KeyCode {
        self.0[4]
    }

    fn camera(&self) -> KeyCode {
        self.0[5]
    }
}

const KeyCodeString: [&str; 163] = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "Escape",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",
    "F13",
    "F14",
    "F15",
    "F16",
    "F17",
    "F18",
    "F19",
    "F20",
    "F21",
    "F22",
    "F23",
    "F24",
    "Snapshot",
    "Scroll",
    "Pause",
    "Insert",
    "Home",
    "Delete",
    "End",
    "PageDown",
    "PageUp",
    "Left",
    "Up",
    "Right",
    "Down",
    "Back",
    "Return",
    "Space",
    "Compose",
    "Caret",
    "Numlock",
    "Numpad0",
    "Numpad1",
    "Numpad2",
    "Numpad3",
    "Numpad4",
    "Numpad5",
    "Numpad6",
    "Numpad7",
    "Numpad8",
    "Numpad9",
    "AbntC1",
    "AbntC2",
    "NumpadAdd",
    "Apostrophe",
    "Apps",
    "Asterisk",
    "Plus",
    "At",
    "Ax",
    "Backslash",
    "Calculator",
    "Capital",
    "Colon",
    "Comma",
    "Convert",
    "NumpadDecimal",
    "NumpadDivide",
    "Equals",
    "Grave",
    "Kana",
    "Kanji",
    "LAlt",
    "LBracket",
    "LControl",
    "LShift",
    "LWin",
    "Mail",
    "MediaSelect",
    "MediaStop",
    "Minus",
    "NumpadMultiply",
    "Mute",
    "MyComputer",
    "NavigateForward",
    "NavigateBackward",
    "NextTrack",
    "NoConvert",
    "NumpadComma",
    "NumpadEnter",
    "NumpadEquals",
    "Oem102",
    "Period",
    "PlayPause",
    "Power",
    "PrevTrack",
    "RAlt",
    "RBracket",
    "RControl",
    "RShift",
    "RWin",
    "Semicolon",
    "Slash",
    "Sleep",
    "Stop",
    "NumpadSubtract",
    "Sysrq",
    "Tab",
    "Underline",
    "Unlabeled",
    "VolumeDown",
    "VolumeUp",
    "Wake",
    "WebBack",
    "WebFavorites",
    "WebForward",
    "WebHome",
    "WebRefresh",
    "WebSearch",
    "WebStop",
    "Yen",
    "Copy",
    "Paste",
    "Cut",
];
