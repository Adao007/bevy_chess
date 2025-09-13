use bevy::prelude::*;
use super::position::*;

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems
                (Startup, 
            (spawn_black_pieces, spawn_white_pieces, spawn_pawns).chain()
                    .after(setup_placement))
            .add_systems(Update, promote_black)
            .add_systems(Update, promote_white);
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King, 
    Queen,
    Bishop, 
    Knight,
    Rook,
}

#[derive(Component)]
pub struct BlackPiece; 

#[derive(Component)]
pub struct WhitePiece; 

#[derive(Component)]
pub struct Pawn; 

#[derive(Component)]
pub struct Movable; 

fn spawn_black_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Placement>,
) {
    let names = vec!["black_rook.png", "black_knight.png", "black_bishop.png",
                                        "nature_queen.png", "black_king.png", "black_bishop.png",
                                        "black_knight.png", "black_rook.png"];
    let types = vec![PieceType::Rook, PieceType::Knight, PieceType::Bishop,
                                    PieceType::King, PieceType::Queen, PieceType::Bishop,
                                    PieceType::Knight, PieceType::Rook];

    for i in 1..=8 {
        if let Some(&(x, y)) = board.positions.get(&format!("H{}", i)) {
            commands.spawn((
                Sprite::from_image(asset_server.load(names[i - 1])),
                Transform::from_xyz(x, y, 1.0),
                BlackPiece,
                Pickable::default(),
                Movable,
            ));
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
            WhitePiece,
            Pickable::default(),
            Movable,
        ));
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
                WhitePiece,
                Pawn,
                Pickable::default(),
                Movable,
            ));
        } 
    }
    
    // Spawning the Black Pawns 
    for i in 1..=8 {
        if let Some(&(x, y)) = board.positions.get(&format!("G{}", i)) {
            commands.spawn((
                Sprite::from_image(asset_server.load("black_pawn.png")),
                Transform::from_xyz(x, y, 1.0),
                BlackPiece,
                Pawn,
                Pickable::default(),
                Movable,
            ));
        }
    }
}

fn promote_black(
    mut commands: Commands,
    mut pawn_query: Query<(Entity, &mut Sprite, &Transform), (With<Pawn>, With<BlackPiece>)>, 
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>, 
) {
    for (entity, mut sprite, piece) in pawn_query.iter_mut() {
        if (piece.translation.y <= -300.0 && piece.translation.y >= -400.0) &&
           (piece.translation.x <= 400.0 && piece.translation.x >= -400.0) {
            //*sprite = Sprite::from_image(asset_server.load("black_queen.png")); 
            if keys.just_released(KeyCode::KeyQ) {
                *sprite = Sprite::from_image(asset_server.load("black_queen.png")); 
                commands.entity(entity).remove::<Pawn>();
            }
            if keys.just_released(KeyCode::KeyR) {
                *sprite = Sprite::from_image(asset_server.load("black_rook.png")); 
                commands.entity(entity).remove::<Pawn>();
            }
            if keys.just_released(KeyCode::KeyK) {
                *sprite = Sprite::from_image(asset_server.load("black_knight.png")); 
                commands.entity(entity).remove::<Pawn>();
            }
            if keys.just_released(KeyCode::KeyB) {
                *sprite = Sprite::from_image(asset_server.load("black_bishop.png"));
                commands.entity(entity).remove::<Pawn>();
            }
            
        }
    }
} 

fn promote_white(
    mut commands: Commands,
    mut pawn_query: Query<(Entity, &mut Sprite, &Transform), (With<Pawn>, With<WhitePiece>)>, 
    asset_server: Res<AssetServer>, 
    keys: Res<ButtonInput<KeyCode>>, 
) {
    for (entity, mut sprite, piece) in pawn_query.iter_mut() {
        if (piece.translation.y >= 300.0 && piece.translation.y <= 400.0) &&
           (piece.translation.x <= 400.0 && piece.translation.x >= -400.0){
            if keys.just_released(KeyCode::KeyQ) {
                *sprite = Sprite::from_image(asset_server.load("white_queen.png")); 
                commands.entity(entity).remove::<Pawn>(); 
            }
            if keys.just_released(KeyCode::KeyB) {
                *sprite = Sprite::from_image(asset_server.load("white_bishop.png")); 
                commands.entity(entity).remove::<Pawn>(); 
            }
            if keys.just_released(KeyCode::KeyK) {
                *sprite = Sprite::from_image(asset_server.load("white_knight.png")); 
                commands.entity(entity).remove::<Pawn>(); 
            }
            if keys.just_released(KeyCode::KeyR) {
                *sprite = Sprite::from_image(asset_server.load("white_rook.png")); 
                commands.entity(entity).remove::<Pawn>(); 
            }
        }
    }
}

