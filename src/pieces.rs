use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use super::position::*;
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
    
}

fn spawn_pieces(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    board: Res<Placement>,
) {
    if let Some(&(x, y)) = board.positions.get(&"A1".to_string()) {
        commands.spawn((
            Sprite::from_image(asset_server.load("white_rook.png")),
            Transform::from_xyz(x, y, 1.0),
        ))
            .insert(
                Piece{
                    color: PieceColor::White,
                    piece_type: PieceType::Rook,
                    // TODO: position variable later
                }); 
    }

    if let Some(&(x, y)) = board.positions.get(&"A2".to_string()) {
        commands.spawn((
            Sprite::from_image(asset_server.load("white_knight.png")),
            Transform::from_xyz(x, y, 1.0),
        ))
            .insert(
                Piece{
                    color: PieceColor::White,
                    piece_type: PieceType::Knight,
                    // TODO: position variable later
                }); 
    }

    if let Some(&(x, y)) = board.positions.get(&"A3".to_string()) {
        commands.spawn((
            Sprite::from_image(asset_server.load("white_bishop.png")),
            Transform::from_xyz(x, y, 1.0),
        ))
            .insert(
                Piece{
                    color: PieceColor::White,
                    piece_type: PieceType::Bishop,
                    // TODO: position variable later
                }); 
    }

    if let Some(&(x, y)) = board.positions.get(&"A4".to_string()) {
        commands.spawn((
            Sprite::from_image(asset_server.load("white_queen.png")),
            Transform::from_xyz(x, y, 1.0),
        ))
            .insert(
                Piece{
                    color: PieceColor::White,
                    piece_type: PieceType::Queen,
                    // TODO: position variable later
                }); 
    }

    if let Some(&(x, y)) = board.positions.get(&"A5".to_string()) {
        commands.spawn((
            Sprite::from_image(asset_server.load("white_king.png")),
            Transform::from_xyz(x, y, 1.0),
        ))
            .insert(
                Piece{
                    color: PieceColor::White,
                    piece_type: PieceType::King,
                    // TODO: position variable later
                }); 
    }

    if let Some(&(x, y)) = board.positions.get(&"A6".to_string()) {
        commands.spawn((
            Sprite::from_image(asset_server.load("white_bishop.png")),
            Transform::from_xyz(x, y, 1.0),
        ))
            .insert(
                Piece{
                    color: PieceColor::White,
                    piece_type: PieceType::Bishop,
                    // TODO: position variable later
                }); 
    }

    if let Some(&(x, y)) = board.positions.get(&"A7".to_string()) {
        commands.spawn((
            Sprite::from_image(asset_server.load("white_knight.png")),
            Transform::from_xyz(x, y, 1.0),
        ))
            .insert(
                Piece{
                    color: PieceColor::White,
                    piece_type: PieceType::Knight,
                    // TODO: position variable later
                }); 
    }

    if let Some(&(x, y)) = board.positions.get(&"A8".to_string()) {
        commands.spawn((
            Sprite::from_image(asset_server.load("white_rook.png")),
            Transform::from_xyz(x, y, 1.0),
        ))
            .insert(
                Piece{
                    color: PieceColor::White,
                    piece_type: PieceType::Rook,
                    // TODO: position variable later
                }); 
    }
}

fn spawn_pawns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Placement>,
) {
    for i in 1..=8 {
        if let Some(&(x, y)) = board.positions.get(&format!("B{}", i)) {
            commands.spawn((
                Sprite::from_image(asset_server.load("white_pawn.png")),
                Transform::from_xyz(x, y, 1.0),
            ))
                .insert(
                    Piece{
                        color:PieceColor::White,
                        piece_type: PieceType::Pawn,
                        // TODO: position variable later
                });
        } 
    }
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_pieces.after(setup_placement))
            .add_systems(Startup, spawn_pawns.after(setup_placement));
    }
}