use bevy::prelude::*;
use bevy_ecs_tilemap::{MapQuery, Tile, TilePos};

const CLICKED_BUTTON: Color = Color::rgb(0.9, 0.9, 0.9);
const HOVERED_BUTTON: Color = Color::rgb(0.7, 0.7, 0.7);
const NORMAL_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5);

const FONT_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_buttons)
            .add_system(button_system);
    } 
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &mut Style),
        (Changed<Interaction>, With<Button>),
    >,
    mut map_query: MapQuery,
    mut tile_query: Query<&mut Tile>
) {
    for (interaction, mut color, mut style) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = CLICKED_BUTTON.into();
                style.size = Size::new(Val::Px(143.0), Val::Px(52.0));
                for x in 0..(crate::game::MAP_SIZE.0*crate::game::CHUNK_SIZE.0) {
                    for y in 0..(crate::game::MAP_SIZE.1*crate::game::CHUNK_SIZE.1){
                        let tile_entity = map_query.get_tile_entity(TilePos(x,y), 0u16, crate::game::CELL_ID).unwrap();
                        let mut tile = tile_query.get_mut(tile_entity).unwrap();
                        tile.visible = false;

                        // Update tile
                        map_query.notify_chunk_for_tile(TilePos(x,y), 0u16, crate::game::CELL_ID);

                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                style.size = Size::new(Val::Px(150.0), Val::Px(55.0));
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                style.size = Size::new(Val::Px(150.0), Val::Px(55.0));
            }
        }
    }
}

fn spawn_buttons(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(55.0)),
            justify_content: JustifyContent::Center,
            margin: Rect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Px(35.0),
            },
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: NORMAL_BUTTON.into(),
        ..Default::default()
    }) 
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Clear",
                TextStyle {
                    font: asset_server.load("times.ttf"),
                    font_size: 30.0,
                    color: FONT_COLOR,
                },
                Default::default(),
            ),
            ..Default::default()
        });
    });
}