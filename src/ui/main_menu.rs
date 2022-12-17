use bevy::{app::AppExit, prelude::*};
use iyes_loopless::prelude::*;

use crate::{keyboard::KeyboardBindings, GameState};

const FONT: &str = "fonts/FiraSans-Bold.ttf";
const SIZE: f32 = 24.0;
const COLOR_HIGHLIGHTED: Color = Color::ORANGE_RED;
const COLOR_DEFAULT: Color = Color::GRAY;
const MAIN_MENU_ITEMS: usize = 3;
const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(Clone, Component, Copy, Debug)]
pub struct MainMenu(pub usize);

#[derive(Clone, Component, Copy, Debug)]
pub struct MainMenuItem;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT);
    let item_style = Style {
        margin: UiRect::all(Val::Px(10.0)),
        ..Default::default()
    };

    let main_menu = commands
        .spawn(MainMenu(0))
        .insert(NodeBundle {
            // background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
            background_color: BACKGROUND_COLOR.into(),
            style: Style {
                // size: Size::new(Val::Auto, Val::Auto),
                // size: Size::new(Val::Auto, Val::Auto),
                // position_type: PositionType::Absolute,
                margin: UiRect::all(Val::Auto),
                // padding: UiRect::all(Val::Px(300.0)),
                // margin: UiRect::all(Val::Px(100.0)),
                // align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let start_new_game = commands
        .spawn(MainMenuItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Start new game",
                TextStyle {
                    font: font.clone(),
                    font_size: SIZE,
                    color: COLOR_HIGHLIGHTED,
                },
            ),
            style: item_style.clone(),
            ..Default::default()
        })
        .id();

    let settings = commands
        .spawn(MainMenuItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Settings",
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

    let exit = commands
        .spawn(MainMenuItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Exit",
                TextStyle {
                    font,
                    font_size: SIZE,
                    color: COLOR_DEFAULT,
                },
            ),
            style: item_style,
            ..Default::default()
        })
        .id();

    commands
        .entity(main_menu)
        .push_children(&[start_new_game, settings, exit]);
}

pub fn update(
    input: Res<Input<KeyCode>>,
    game_state: Res<CurrentState<GameState>>,
    mut commands: Commands,
    // mut query_camera: Query<&mut UiCameraConfig>,
    mut query_main_menu: Query<(&Children, Entity, &mut MainMenu, &mut Style)>,
    mut query_item: Query<&mut Text, With<MainMenuItem>>,
    query_bindings: Query<&KeyboardBindings>,
    mut exit: EventWriter<AppExit>,
) {
    if game_state.0 != GameState::MainMenu {
        panic!("Trying to update the main menu in the wrong game state");
    }
    // let mut camera = query_camera.single_mut();
    let (children, menu_id, mut menu, mut style) = query_main_menu.single_mut();
    let bindings = query_bindings.single();
    if input.any_just_pressed([KeyCode::Up, bindings.accelerate()]) {
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
                // commands.entity(id).despawn_recursive();
                commands.insert_resource(NextState(GameState::TurnDownLight));
                // camera.show_ui = false;
            }
            1 => {
                // style.display = Display::None;
                commands.entity(menu_id).despawn_recursive();
                commands.insert_resource(NextState(GameState::Settings));
            }
            2 => {
                exit.send(AppExit);
            }
            _ => unreachable!(),
        }
    }
}
