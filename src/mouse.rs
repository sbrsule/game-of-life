use bevy::prelude::*;
use bevy_ecs_tilemap::{MapQuery, TilePos, Tile};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(cursor_system);
    }
}

fn cursor_system(
    window: Res<Windows>,
    mut map_query: MapQuery,
    mut tile_query: Query<&mut Tile>,
    buttons: Res<Input<MouseButton>>,
) {
    // Get window entity
    let window = window.get_primary().unwrap();

    // If the cursor is pressed in the window, the specific tile is accessed
    if let Some(_) = window.cursor_position() {
        if buttons.pressed(MouseButton::Left) {
            let cursor_pos = window.cursor_position().unwrap();
            let tile_pos = get_tile(cursor_pos, window); 
            println!("({:?}", tile_pos);
            if tile_pos.0 >= 0 && tile_pos.1 >= 0 {
                let tile_pos = TilePos(tile_pos.0 as u32, tile_pos.1 as u32);
                match map_query.get_tile_entity(tile_pos, 0u16, crate::game::CELL_ID) {
                    
                    // If the cursor is clicked on a tile, changes the tile's visibility
                    Ok(tile_entity) => {
                        let mut tile = tile_query.get_mut(tile_entity).unwrap();
                        println!("valid tile");
                        tile.visible = true;
    
                        // Update tile 
                        map_query.notify_chunk_for_tile(tile_pos, 0u16, crate::game::CELL_ID);
                    }
                    Err(_) => ()
                };
            }
        }
        else if buttons.pressed(MouseButton::Right) {
            let cursor_pos = window.cursor_position().unwrap();
            let tile_pos = get_tile(cursor_pos, window); 
            println!("({:?}", tile_pos);
            if tile_pos.0 >= 0 && tile_pos.1 >= 0 {
                let tile_pos = TilePos(tile_pos.0 as u32, tile_pos.1 as u32);
                match map_query.get_tile_entity(tile_pos, 0u16, crate::game::CELL_ID) {
                    
                    // If the cursor is clicked on a tile, changes the tile's visibility
                    Ok(tile_entity) => {
                        let mut tile = tile_query.get_mut(tile_entity).unwrap();
                        println!("valid tile");
                        tile.visible = false;
    
                        // Update tile 
                        map_query.notify_chunk_for_tile(tile_pos, 0u16, crate::game::CELL_ID);
                    }
                    Err(_) => ()
                };
            }
        }
    }
}

// Gets tile's coordinates within the map
fn get_tile(
    cursor_pos: Vec2,
    window: &Window,
) -> (i32, i32) {
    // Grabs the windows dimensions
    let window_width = window.width(); 
    let window_height = window.height();


    // Gets the maps origin point's coordinates relative to the window
    let map_pos = (((window_width as u32) - crate::game::TRUE_MAP_SIZE.0) / 2, ((window_height as u32) - crate::game::TRUE_MAP_SIZE.1) / 2);

    // Converts the cursors position from the windows coordinates into the map coordinates
    let tile_x = ((cursor_pos[0] - map_pos.0 as f32) / crate::game::TILE_SIZE.0) as i32;
    let tile_y = ((cursor_pos[1] - map_pos.1 as f32) / crate::game::TILE_SIZE.1) as i32;

    (tile_x, tile_y)
}