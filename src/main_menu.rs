use bevy::prelude::*;
use bevy::app::AppExit;
use crate::AppState;

const BACKGROUND_COLOR: Color = Color::rgb(0.976, 0.973, 0.910);

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ExitButton;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(spawn_menu)
            )
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(button_system)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                    .with_system(despawn_menu)
            );
    }
}

fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(MainMenu);
    commands.spawn_bundle(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: BACKGROUND_COLOR.into(),
            ..Default::default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Game Of Life",
                    TextStyle {
                        font: asset_server.load("times.ttf"),
                        font_size: 50.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(300.0), Val::Px(250.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: Color::rgba(1.0, 0.0, 0.0, 0.3).into(),
                ..Default::default()
            })
            .with_children(|parent|{
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(70.0), Val::Px(70.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: Rect::all(Val::Px(20.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(StartButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Start",
                            TextStyle {
                                font: asset_server.load("times.ttf"),
                                font_size: 30.0, 
                                color: Color::BLACK,
                            },
                            Default::default(), 
                        ),
                        ..Default::default()
                    });
                });
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(70.0), Val::Px(70.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ExitButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Exit",
                            TextStyle {
                                font: asset_server.load("times.ttf"),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            });
        });
}

fn despawn_menu(
    menu_query: Query<Entity, With<MainMenu>>,
    mut commands: Commands,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
fn button_system(
    start_query: Query<&Interaction, With<StartButton>>,
    exit_query: Query<&Interaction, With<ExitButton>>,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<State<AppState>>,
) {
    for interaction in &mut start_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                app_state.set(AppState::InGame).unwrap();
            },
            _ => (),
        }
    }

    for interaction in &mut exit_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                exit.send(AppExit);               
            }

            _ => ()
        }
    }
}