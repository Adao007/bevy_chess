use bevy::prelude::*; 
use bevy_ecs_tilemap::prelude::*;

const MAP_LENGTH: u32 = 8;

// Set that collects systems 
#[derive(SystemSet, Clone, Copy, Hash, PartialEq, Eq, Debug)] 
pub struct SpawnMapSet;

#[derive(Component)]
pub struct MainCamera;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Spawn Camera
    commands.spawn((Camera2d, MainCamera));

    let image_handles = vec![
        asset_server.load("small_ns.png"),
        asset_server.load("small_gs.png"),
    ];
    let texture_vec = TilemapTexture::Vector(image_handles);

    let board_size = TilemapSize { 
        x: MAP_LENGTH,
        y: MAP_LENGTH,
    };

    let tilemap_entity = commands.spawn_empty().id(); 
    let tilemap_id = TilemapId(tilemap_entity);
    let mut tile_storage = TileStorage::empty(board_size);

    fill_board_nature(
        TileTextureIndex(0),
        board_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_board_geo(
        TileTextureIndex(1),
        board_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize {x: 100.0, y: 100.0};
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: board_size,
        storage: tile_storage,
        texture: texture_vec,
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });
}

fn fill_board_nature(
    texture_index: TileTextureIndex,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..size.x {
            for y in 0..size.y {
                if x == 0 || x % 2 == 0 {
                    if y == 0 || y % 2 == 0 {
                        let tile_pos = TilePos{ x, y };
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
                else {
                    if y == 1 || y % 2 != 0 {
                        let tile_pos = TilePos{ x, y };
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
            }
        }
    });
}
// END OF BOARD SETUP

fn fill_board_geo(
    texture_index: TileTextureIndex,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..size.x {
            for y in 0..size.y {
                if x == 0 || x % 2 == 0 {
                    if y == 1 || y % 2 != 0 {
                        let tile_pos = TilePos{ x, y };
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
                else {
                    if y == 0 || y % 2 == 0 {
                        let tile_pos = TilePos{ x, y };
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
            }
        }
    });
}
// END OF BOARD SETUP

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
        //    .init_resource::<TileHandleSquare>()
            .add_systems(Startup, startup.in_set(SpawnMapSet));
    }
}