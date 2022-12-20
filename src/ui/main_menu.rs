use bevy::{app::AppExit, prelude::*};
use iyes_loopless::prelude::*;

use crate::{game_state::GameState, keyboard::KeyboardBindings};

const BACKGROUND_COLOR: Color = Color::BLACK;
const FONT: &str = "fonts/FiraSans-Bold.ttf";
const SIZE: f32 = 24.0;
const COLOR_HIGHLIGHTED: Color = Color::ORANGE_RED;
const COLOR_DEFAULT: Color = Color::GRAY;
const MAIN_MENU_ITEMS: usize = 3;
const SECTIONS: [&str; MAIN_MENU_ITEMS] = ["Start new game", "Settings", "Quit"];

#[derive(Clone, Component, Copy, Debug)]
pub struct MainMenu(pub usize);

#[derive(Clone, Component, Copy, Debug)]
pub struct MainMenuItem;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Style, With<MainMenu>>,
) {
    if let Ok(mut main_menu) = query.get_single_mut() {
        main_menu.display = Display::Flex;
        return;
    }

    let main_menu = commands
        .spawn(MainMenu(0))
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

    for section in SECTIONS {
        let item = commands
            .spawn(MainMenuItem)
            .insert(TextBundle {
                text: Text::from_section(
                    section,
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: SIZE,
                        color: if section == SECTIONS[0] {
                            COLOR_HIGHLIGHTED
                        } else {
                            COLOR_DEFAULT
                        },
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        commands.entity(main_menu).add_child(item);
    }
}

pub fn update(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query_main_menu: Query<(&Children, &mut MainMenu, &mut Style)>,
    mut query_item: Query<&mut Text, With<MainMenuItem>>,
    query_bindings: Query<&KeyboardBindings>,
    mut exit: EventWriter<AppExit>,
) {
    let (children, mut menu, mut style) = query_main_menu.single_mut();
    let bindings = query_bindings.single();

    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
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
        if menu.0 < MAIN_MENU_ITEMS - 1 {
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
                commands.insert_resource(NextState(GameState::TurnDownLight));
            }
            1 => {
                style.display = Display::None;
                commands.insert_resource(NextState(GameState::Settings));
            }
            2 => {
                exit.send(AppExit);
            }
            _ => unreachable!(),
        }
    }
}
