mod game;
mod main_menu;
mod mouse;
mod button;
mod texture_helper;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use texture_helper::set_texture_filters_to_nearest;

const WINDOW_PADDING: (f32, f32) = (200.0, 100.0);
const WINDOW_WIDTH: f32 = ((game::MAP_SIZE.0 * game::CHUNK_SIZE.0) as f32 * game::TILE_SIZE.0) + (game::TILE_SIZE.0 * 2f32) + WINDOW_PADDING.0; 
const WINDOW_HEIGHT: f32 = ((game::MAP_SIZE.1 * game::CHUNK_SIZE.1) as f32 * game::TILE_SIZE.1) + (game::TILE_SIZE.1 * 2f32) + WINDOW_PADDING.1; 

const WINDOW_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[derive(Component)]
struct MainCamera;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: String::from("Game of Life"),
            ..Default::default()
        })
        .insert_resource(ClearColor(WINDOW_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(mouse::MousePlugin)
        .add_plugin(button::ButtonPlugin)
        .add_system(set_texture_filters_to_nearest)
        .add_state(AppState::MainMenu)
        .run();
}
