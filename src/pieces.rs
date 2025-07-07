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

#[derive(Clone, Copy, Component)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType, 
    // Current Position
    tile_pos: TilePos,
}

fn spawn_pieces(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("white_rook.png")),
        Transform::from_xyz(0., 0., 1.0),
    ))
        .insert(
            Piece{
                color: PieceColor::White,
                piece_type: PieceType::Rook,
                tile_pos: TilePos{ x: 0, y: 0},
            }); 
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_pieces);
    }
}