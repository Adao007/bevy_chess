use bevy::prelude::*; 
use bevy::color::palettes;
use bevy_ecs_tilemap::prelude::*;
use crate::cursor::CursorPos;

const MAP_LENGTH: u32 = 8;

// Set that collects systems 
#[derive(SystemSet, Clone, Copy, Hash, PartialEq, Eq, Debug)] 
pub struct SpawnMapSet;

// // Loads images for the squares on the board
// #[derive(Deref, Resource)]
// pub struct TileHandleSquare(Handle<Image>);
// impl FromWorld for TileHandleSquare {
//     fn from_world(world: &mut World) -> Self {
//         let asset_server = world.resource::<AssetServer>();
//         Self(asset_server.load("small_ns.png"))
//     }
// }

// #[derive(Deref, Resource)]
// pub struct SecondTileHandleSquare(Handle<Image>);
// impl FromWorld for SecondTileHandleSquare {
//     fn from_world(world: &mut World) -> Self {
//         let asset_server = world.resource::<AssetServer>();
//         Self(asset_server.load("small_gs.png"))
//     }
// }

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Spawn Camera
    commands.spawn(Camera2d);

    let image_handles = vec![
        asset_server.load("small_ns.png"),
        asset_server.load("small_gs.png"),
    ];
    let texture_vec = TilemapTexture::Vector(image_handles);

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

#[derive(Component)]
struct TileLabel(Entity);

fn spawn_tile_labels(
    mut commands: Commands,
    tilemap_q: Query<(
        &Transform,
        &TilemapType,
        &TilemapGridSize,
        &TilemapTileSize,
        &TileStorage,
        &TilemapSize,
        &TilemapAnchor,
    )>,
    tile_q: Query<&mut TilePos>, 
) {
    for (map_transform, map_type, grid_size, 
        tile_size, tilemap_storage, map_size, anchor) in tilemap_q.iter() {
            for tile_entity in tilemap_storage.iter().flatten() {
                let tile_pos = tile_q.get(*tile_entity).unwrap();
                let tile_center = tile_pos
                    .center_in_world(map_size, grid_size, tile_size, map_type, anchor)
                    .extend(1.0);
                let transform = *map_transform * Transform::from_translation(tile_center);

                let label_entity = commands
                    .spawn((
                        Text2d::new(format!("{}, {}", tile_pos.x, tile_pos.y)),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        TextLayout::new_with_justify(JustifyText::Center),
                        transform,
                    ))
                    .id();
                commands
                    .entity(*tile_entity)
                    .insert(TileLabel(label_entity));
            }
    }
}

#[derive(Component)]
struct HighlightedLabel;

fn highlight_tile_labels(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TileStorage,
        &Transform,
        &TilemapAnchor,
    )>,
    highlighted_tiles_q: Query<Entity, With<HighlightedLabel>>,
    tile_label_q: Query<&TileLabel>,
    mut text_q: Query<&mut TextColor>,
) {
    // Un-highlight any previously highlighted tile labels.
    for highlighted_tile_entity in highlighted_tiles_q.iter() {
        if let Ok(label) = tile_label_q.get(highlighted_tile_entity) {
            if let Ok(mut text_color) = text_q.get_mut(label.0) {
                text_color.0 = Color::BLACK;
                commands
                    .entity(highlighted_tile_entity)
                    .remove::<HighlightedLabel>();
            }
        }
    }

    for (map_size, grid_size, tile_size, map_type, tile_storage, map_transform, anchor) in
        tilemap_q.iter()
    {
        // Grab the cursor position from the `Res<CursorPos>`
        let cursor_pos: Vec2 = cursor_pos.0;
        // We need to make sure that the cursor's world position is correct relative to the map
        // due to any map transformation.
        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 0.0 and 1.0
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        // Once we have a world position we can transform it into a possible tile position.
        if let Some(tile_pos) = TilePos::from_world_pos(
            &cursor_in_map_pos,
            map_size,
            grid_size,
            tile_size,
            map_type,
            anchor,
        ) {
            // Highlight the relevant tile's label
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                if let Ok(label) = tile_label_q.get(tile_entity) {
                    if let Ok(mut text_color) = text_q.get_mut(label.0) {
                        text_color.0 = palettes::tailwind::RED_600.into();
                        commands.entity(tile_entity).insert(HighlightedLabel);
                    }
                }
            }
        }
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
        //    .init_resource::<TileHandleSquare>()
            .add_systems(Startup, startup.in_set(SpawnMapSet))
            .add_systems(Startup, spawn_tile_labels.after(SpawnMapSet))
            .add_systems(Update, highlight_tile_labels);
    }
}