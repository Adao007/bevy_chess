use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy::color::palettes::basic::RED;
use super::cursor::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White, 
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King, 
    Queen,
    Bishop, 
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType, 
    // Current Position
    pub x: u8,
    pub y: u8,
}

fn spawn_pieces(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
    //commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(0.0, 20.0, 1.0),
    ));

}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_pieces);
    }
}