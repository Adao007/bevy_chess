use bevy::prelude::*; 
use bevy_ecs_tilemap::prelude::*;

const MAP_LENGTH: u32 = 8;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TileHandleSquare>()
            .add_systems(Startup, startup);
    }
}

// Loads images for the squares on the board
#[derive(Deref, Resource)]
pub struct TileHandleSquare(Handle<Image>);

impl FromWorld for TileHandleSquare {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("images.png"))
    }
}

fn startup(
    mut commands: Commands,
    tile_handle_square: Res<TileHandleSquare>
) {
    // Spawn Camera
    commands.spawn(Camera2d);

    let board_size = TilemapSize { 
        x: MAP_LENGTH,
        y: MAP_LENGTH,
    };

    // Create a tilemap entity a little early, this entity will tell each tile which tilemap entity
    // it is associated with. Using TilemapId component on each tile, then we will insert the TilemapBundle 
    // bundle on the entity which will have necessary components such as TileStorage... 
    let tilemap_entity = commands.spawn_empty().id(); 
    
    let tilemap_id = TilemapId(tilemap_entity);
    // To create a map, we will need the 'TileStorage' Component.
    // This component is a grid of tile entities and will track each of the tiles in the world. 
    // If multiple layers of tiles exist you would have 
    let mut tile_storage = TileStorage::empty(board_size);

    fill_board(
        TileTextureIndex(0),
        board_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize {x: 64.0, y: 64.0};
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: board_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_handle_square.clone()),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });
}

fn fill_board(
    texture_index: TileTextureIndex,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..size.x {
            for y in 0..size.y {
                let tile_pos = TilePos{ x, y};
                let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index,
                        ..Default::default()
                    })    
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });
}
// END OF BOARD SETUP

