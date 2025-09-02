use bevy::prelude::*;
use std::fmt::Debug;
use super::position::*;
use super::cursor::*;

struct Helper; 
impl Helper {
    fn drag<E: Debug + Clone + Reflect>() -> impl Fn(Trigger<E>, Commands, Query<Entity, With<Pickable>>) {
        move |ev, mut commands, mut sprites| {
            let Ok(sprite) = sprites.get_mut(ev.target()) else {
                return; 
            }; 

            commands.entity(sprite).insert(Draggable);
        }
    }

    fn drop<E: Debug + Clone + Reflect>() -> impl Fn(
        Trigger<E>, 
        Commands, 
        Query<(Entity, &mut Piece), With<Draggable>>, 
        Res<MouseWorldCoords>,
    ) {
        move |ev, mut commands, mut sprites, cursor_pos| {
            let Ok(sprite) = sprites.get_mut(ev.target()) else {
                return;
            };

            let (sprite, mut piece) = sprite; 
            if let Some(pos) = cursor_pos.0 {
                piece.position.x = pos.x;
                piece.position.y = pos.y; 
            }
            
            commands.entity(sprite).remove::<Draggable>();
        }
    }
}

#[derive(Component)]
struct Movable; 

#[derive(Component)]
struct Draggable; 

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
    pub position: Vec2,
}

#[derive(Component)]
pub struct BlackPiece; 

#[derive(Component)]
pub struct WhitePiece; 

#[derive(Component)]
pub struct Pawn; 

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
            ))
            .insert(
        Piece { 
                    position: Vec2::new(x, y), 
            })
            .observe(Helper::drag::<Pointer<Pressed>>())
            .observe(Helper::drop::<Pointer<Released>>());
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
        ))
            .insert(
                Piece{
                    position: Vec2::new(x, y), 
            })
            .observe(Helper::drag::<Pointer<Pressed>>())
            .observe(Helper::drop::<Pointer<Released>>());
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
            ))
                .insert(
                    Piece{
                        position: Vec2::new(x, y), 
                })
                .observe(Helper::drag::<Pointer<Pressed>>())
                .observe(Helper::drop::<Pointer<Released>>());
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
            ))
                .insert(
                    Piece {
                        position: Vec2::new(x, y), 
                })
                .observe(Helper::drag::<Pointer<Pressed>>())
                .observe(Helper::drop::<Pointer<Released>>());
        }
    }
}

fn grab(
    cursor_pos: Res<MouseWorldCoords>, 
    piece: Single<&mut Transform, With<Draggable>>
) {
    let mut transform = piece.into_inner(); 
    if let Some(pos) = cursor_pos.0 {
        transform.translation.x = pos.x;
        transform.translation.y = pos.y; 
    }
}

fn promote_black(
    mut commands: Commands,
    mut pawn_query: Query<(Entity, &mut Sprite, &Piece), (With<Pawn>, With<BlackPiece>)>, 
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>, 
) {
    for (entity, mut sprite, piece) in pawn_query.iter_mut() {
        if piece.position.y <= -300.0 && piece.position.y >= -400.0 {
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
    mut pawn_query: Query<(Entity, &mut Sprite, &Piece), (With<Pawn>, With<WhitePiece>)>, 
    asset_server: Res<AssetServer>, 
    keys: Res<ButtonInput<KeyCode>>, 
) {
    for (entity, mut sprite, piece) in pawn_query.iter_mut() {
        if piece.position.y >= 300.0 && piece.position.y <= 400.0 {
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

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_black_pieces.after(setup_placement))
            .add_systems(Startup, spawn_white_pieces.after(setup_placement))
            .add_systems(Startup, spawn_pawns.after(setup_placement))
            .add_systems(Update, grab.after(update_cursor_pos))
            .add_systems(Update, promote_black)
            .add_systems(Update, promote_white); 
    }
}