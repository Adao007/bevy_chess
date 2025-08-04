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

fn spawn_black_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Placement>,
) {
    let names = vec!["black_rook.png", "black_knight.png", "black_bishop.png",
                                        "black_queen.png", "black_king.png", "black_bishop.png",
                                        "black_knight.png", "black_rook.png"];
    let types = vec![PieceType::Rook, PieceType::Knight, PieceType::Bishop,
                                    PieceType::King, PieceType::Queen, PieceType::Bishop,
                                    PieceType::Knight, PieceType::Rook];

    for i in 1..=8 {
        if let Some(&(x, y)) = board.positions.get(&format!("H{}", i)) {
            commands.spawn((
                Sprite::from_image(asset_server.load(names[i - 1])),
                Transform::from_xyz(x, y, 1.0),
            ))
                .insert(
                    Piece { 
                        color: PieceColor::Black, 
                        piece_type: types[i - 1] 
                        // TODO: position variable later
                });
        }
    }                                
}

fn spawn_white_pieces(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    board: Res<Placement>,
) {
    let names = vec!["white_rook.png", "white_knight.png", "white_bishop.png", 
                                        "white_queen.png", "white_king.png", "white_bishop.png",
                                        "white_knight.png", "white_rook.png"];
    let types = vec![PieceType::Rook, PieceType::Knight, PieceType::Bishop, 
                                        PieceType::Queen, PieceType::King, PieceType::Bishop, 
                                        PieceType::Knight, PieceType::Rook];

    for i in 1..=8 {
        if let Some(&(x, y)) = board.positions.get(&format!("A{}", i)) {
        commands.spawn((
            Sprite::from_image(asset_server.load(names[i - 1])),
            Transform::from_xyz(x, y, 1.0),
        ))
            .insert(
                Piece{
                    color: PieceColor::White,
                    piece_type: types[i - 1],
                    // TODO: position variable later
                }); 
        }
    }
}

fn spawn_pawns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Placement>,
) {
    // Spawning the White Pawns
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
    
    // Spawning the Black Pawns 
    for i in 1..=8 {
        if let Some(&(x, y)) = board.positions.get(&format!("G{}", i)) {
            commands.spawn((
                Sprite::from_image(asset_server.load("black_pawn.png")),
                Transform::from_xyz(x, y, 1.0),
            ))
                .insert(
                    Piece {
                        color: PieceColor::Black,
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
            .add_systems(Startup, spawn_black_pieces.after(setup_placement))
            .add_systems(Startup, spawn_white_pieces.after(setup_placement))
            .add_systems(Startup, spawn_pawns.after(setup_placement));
    }
}