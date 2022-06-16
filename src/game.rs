use bevy::{prelude::*};
use bevy_ecs_tilemap::prelude::*;

use crate::AppState;

const TIME_STEP: f64 = 0.5;

pub const MAP_SIZE: (u32, u32) = (4, 4);
pub const CHUNK_SIZE: (u32, u32) = (12, 12);
pub const TILE_SIZE: (f32, f32) = (16.0, 16.0);
pub const TEXTURE_SIZE: (f32, f32) = (96.0, 16.0);
pub const TRUE_MAP_SIZE: (u32, u32) = (MAP_SIZE.0 * CHUNK_SIZE.0 * TILE_SIZE.0 as u32, MAP_SIZE.1 * CHUNK_SIZE.1 * TILE_SIZE.1 as u32);

pub const GRID_ID: u16 = 0;
pub const CELL_ID: u16 = 1;


#[derive(Component)]
pub struct GameCamera;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::InGame) 
                    .with_system(startup)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_cells)
                    .with_system(pause_game)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Paused)
                    .with_system(pause_game)
            );
    }
}

#[derive(Component)]
pub struct LastUpdate(f64);

fn startup(
    asset_server: Res<AssetServer>, 
    mut commands: Commands,
    mut map_query: MapQuery,
    mut app_state: ResMut<State<AppState>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCamera);
    app_state.push(AppState::Paused).unwrap();

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let layer_settings = LayerSettings::new(
        MapSize(MAP_SIZE.0, MAP_SIZE.1),
        ChunkSize(CHUNK_SIZE.0, CHUNK_SIZE.1),
        TileSize(TILE_SIZE.0, TILE_SIZE.1),
        TextureSize(TEXTURE_SIZE.0, TEXTURE_SIZE.1),
    );
    let (mut layer_builder_grid, layer_0_entity) = 
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings.clone(), 0u16, GRID_ID);
    
    layer_builder_grid.set_all(Tile {
        texture_index: 4,
        ..Default::default()
    }  
    .into(),);

    let (mut layer_builder_cell, layer_1_entity) = 
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings.clone(), 0u16, CELL_ID);


    for x in 0..(MAP_SIZE.0 * CHUNK_SIZE.0) {
        for y in 0..(MAP_SIZE.1 * CHUNK_SIZE.1) {
            let _ = layer_builder_cell.set_tile(
               TilePos(x, y),
               Tile {
                   texture_index: 0,
                   visible: false,
                   ..Default::default()
               }.into(),
            );
        }
    }

    map_query.build_layer(&mut commands, layer_builder_grid, texture_handle.clone());
    map_query.build_layer(&mut commands, layer_builder_cell, texture_handle.clone());
    commands.entity(layer_1_entity).insert(LastUpdate(0.0));
    commands.entity(layer_0_entity);

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    map.add_layer(&mut commands, GRID_ID, layer_0_entity);
    map.add_layer(&mut commands, CELL_ID, layer_1_entity);

    commands
        .entity(map_entity)
        .insert(map)
        // This line centers the map on the screen
        .insert(Transform::from_xyz(-((MAP_SIZE.0 * CHUNK_SIZE.0) as f32 * TILE_SIZE.0)/2.0, -((MAP_SIZE.1 * CHUNK_SIZE.1) as f32 * TILE_SIZE.1)/2.0, 0.0))
        .insert(GlobalTransform::default());
}

fn update_cells(
    mut commands: Commands,
    tile_query: Query<(Entity, &Tile, &TilePos)>, mut map_query: MapQuery,
    time: Res<Time>,
    mut last_update_query: Query<&mut LastUpdate>,
) {
    let current_time = time.seconds_since_startup();
    let mut last_update = last_update_query.single_mut();
    if current_time - last_update.0 > TIME_STEP {
        for (entity, tile, pos) in tile_query.iter() {
            let neighbor_count = map_query
                .get_tile_neighbors(*pos, 0u16, CELL_ID)
                .iter()
                .filter(|&&neighboring_result| {
                    if neighboring_result.is_ok() {
                        let tile_component: &Tile = tile_query
                            .get_component::<Tile>(neighboring_result.unwrap())
                            .unwrap();
                        tile_component.visible
                    } else {
                        false
                    }
                })
                .count();
             
            let was_alive = tile.visible;
            let is_alive = match (was_alive, neighbor_count) {
                (true, x) if x < 2 => false,
                (true, 2) | (true,3) => true,
                (true, x) if x > 3 => false,
                (false, 3) => true,
                (otherwise, _) => otherwise, 
            };
        
            if is_alive && !was_alive {
                commands.entity(entity).insert(Tile {
                    visible: true,
                    ..*tile
                });
                map_query.notify_chunk_for_tile(*pos, 0u16, CELL_ID);
            } else if !is_alive &&was_alive {
                commands.entity(entity).insert(Tile {
                    visible: false,
                    ..*tile
                });
                map_query.notify_chunk_for_tile(*pos, 0u16, CELL_ID);
            }
        }

        last_update.0 = current_time;
    }

}

fn pause_game(
    mut app_state: ResMut<State<AppState>>, 
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::P) {
        match app_state.current() {
            AppState::InGame => {
                app_state.push(AppState::Paused).unwrap();
                println!("PAUSED")
            }
            AppState::Paused => {
                app_state.pop().unwrap();
                println!("RESUMED")
            }
            _ => ()
        }
        keys.reset(KeyCode::P);
    }
}