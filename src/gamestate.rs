use bevy::{app::AppExit, prelude::*};
use iyes_loopless::prelude::*;

const FONT: &str = "fonts/FiraSans-Bold.ttf";
const SIZE: f32 = 24.0;
const COLOR_HIGHLIGHTED: Color = Color::ORANGE_RED;
const COLOR_DEFAULT: Color = Color::GRAY;
const PAUSE_MENU_ITEMS: usize = 2;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    InGame,
    Paused,
}

#[derive(Clone, Component, Copy, Debug)]
pub struct PauseMenu(pub usize);

#[derive(Clone, Component, Copy, Debug)]
pub struct PauseMenuItem;

pub fn pause_menu_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT);
    // let item_textstyle = TextStyle {
    //     font,
    //     font_size: 24.0,
    //     color: Color::rgb(0.5, 0.5, 0.5),
    // };
    let item_style = Style {
        // align_self: AlignSelf::FlexStart,
        // justify_content: JustifyContent::FlexStart,
        // align_items: AlignItems::Center,
        // padding: UiRect::all(Val::Px(300.0)),
        margin: UiRect::all(Val::Px(10.0)),
        ..Default::default()
    };

    let pause_menu = commands
        .spawn(PauseMenu(0))
        .insert(NodeBundle {
            // background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
            background_color: Color::NONE.into(),
            style: Style {
                // size: Size::new(Val::Auto, Val::Percent(100.0)),
                size: Size::new(Val::Auto, Val::Auto),
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

    let resume = commands
        .spawn(PauseMenuItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Resume",
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

    let exit = commands
        .spawn(PauseMenuItem)
        .insert(TextBundle {
            text: Text::from_section(
                "Exit Game",
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

    commands.entity(pause_menu).push_children(&[resume, exit]);
}

pub fn pause_menu(
    input: Res<Input<KeyCode>>,
    game_state: Res<CurrentState<GameState>>,
    mut commands: Commands,
    mut query_camera: Query<&mut UiCameraConfig>,
    mut query_menu: Query<(&Children, &mut PauseMenu)>,
    mut query_item: Query<&mut Text, With<PauseMenuItem>>,
    mut exit: EventWriter<AppExit>,
) {
    let mut camera = query_camera.single_mut();
    if game_state.0 == GameState::InGame && input.just_pressed(KeyCode::P) {
        commands.insert_resource(NextState(GameState::Paused));
        camera.show_ui = true;
    } else if game_state.0 == GameState::Paused {
        let (children, mut menu) = query_menu.single_mut();
        if input.just_pressed(KeyCode::P) {
            commands.insert_resource(NextState(GameState::InGame));
            camera.show_ui = false;
            if menu.0 != 0 {
                query_item.get_mut(children[menu.0]).unwrap().sections[0]
                    .style
                    .color = COLOR_DEFAULT;
                menu.0 = 0;
                query_item.get_mut(children[menu.0]).unwrap().sections[0]
                    .style
                    .color = COLOR_HIGHLIGHTED;
            }
        } else if input.any_just_pressed([KeyCode::Up, KeyCode::O]) {
            if menu.0 > 0 {
                query_item.get_mut(children[menu.0]).unwrap().sections[0]
                    .style
                    .color = COLOR_DEFAULT;
                menu.0 -= 1;
                query_item.get_mut(children[menu.0]).unwrap().sections[0]
                    .style
                    .color = COLOR_HIGHLIGHTED;
            }
        } else if input.any_just_pressed([KeyCode::Down, KeyCode::L]) {
            if menu.0 < PAUSE_MENU_ITEMS - 1 {
                query_item.get_mut(children[menu.0]).unwrap().sections[0]
                    .style
                    .color = COLOR_DEFAULT;
                menu.0 += 1;
                query_item.get_mut(children[menu.0]).unwrap().sections[0]
                    .style
                    .color = COLOR_HIGHLIGHTED;
            }
        } else if input.any_just_pressed([KeyCode::Return, KeyCode::R]) {
            match menu.0 {
                0 => {
                    commands.insert_resource(NextState(GameState::InGame));
                    camera.show_ui = false;
                }
                1 => {
                    exit.send(AppExit);
                }
                _ => unreachable!(),
            }
        }
    }
}
