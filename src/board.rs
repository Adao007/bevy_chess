use bevy::prelude::*; 
use bevy_ecs_tilemap::prelude::*;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn Camera
    commands.spawn(Camera2d);

    let texture_handle: Handle<Image> = asset_server.load("images.png");

    let board_size = TilemapSize { x: 8, y: 8 };

    // Create a tilemap entity a little early, this entity will tell each tile which tilemap entity
    // it is associated with. Using TilemapId component on each tile, then we will insert the TilemapBundle 
    // bundle on the entity which will have necessary components such as TileStorage... 
    let tilemap_entity = commands.spawn_empty().id(); 
    
    // To create a map, we will need the 'TileStorage' Component.
    // This component is a grid of tile entities and will track each of the tiles in the world. 
    // If multiple layers of tiles exist you would have 
    let mut tile_storage = TileStorage::empty(board_size);

    // Spawn the elements of the tilemap!
    for x in 0..board_size.x {
        for y in 0..board_size.y {
            let tile_pos = TilePos {x, y};
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize {x: 64.0, y: 64.0};
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: board_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });
}