use bevy::prelude::*;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use std::fmt::Debug;
use super::position::*;
use super::cursor::*;

const PIECESIZE: f32 = 37.5;
const MOVEOVER: f32 = 40.0; 
const RESET_LIMIT: f32 = -450.0; 
const SCALER: f32 = 0.40; 

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems
                (Startup, 
            (spawn_black_pieces, spawn_white_pieces, spawn_pawns).chain()
                    .after(setup_placement))
            .add_systems(Update, promote_black)
            .add_systems(Update, promote_white)
            .add_systems(Update, grab.after(update_cursor_pos))
            .add_systems(Update, (take_white, take_black));
    }
}

struct Helper; 
impl Helper {
    fn drag<E: Debug + Clone + Reflect>() -> impl Fn(
        Trigger<E>, 
        Commands, 
        Query<Entity, With<Pickable>>
    ) {
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
        Query<Entity, With<Draggable>>, 
        Res<MouseWorldCoords>,
    ) {
        move |ev, mut commands, mut sprites, cursor_pos| {
            let Ok(sprite) = sprites.get_mut(ev.target()) else {
                return;
            };

            commands.entity(sprite).remove::<Draggable>();
            commands.entity(sprite).insert(Dropped); 
        }
    }

}

#[derive(Component)]
struct Movable; 

#[derive(Component)]
struct Draggable; 

#[derive(Component)]
struct Dropped;

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

fn take_white(
   taken_query: Query<(Entity, &mut Transform),  (With<WhitePiece>, Without<Draggable>)>,
   captor_query: Single<&Transform, (With<Draggable>, With<BlackPiece>)>, 
   mut commands: Commands,
   mouse: Res<ButtonInput<KeyCode>>,
   mut removal: ResMut<CaptureZones>,
) {
    let capture = captor_query.into_inner(); 
    for (piece, mut taken) in taken_query.into_iter() {
        let collision = check_for_collisions(
            Aabb2d::new(
                taken.translation.truncate(),
                Vec2::new(PIECESIZE, PIECESIZE),
            ),
            Aabb2d::new(
                capture.translation.truncate(),
                Vec2::new(PIECESIZE, PIECESIZE),
            )
        ); 

        if collision {
            if mouse.just_pressed(KeyCode::KeyX) {
                if removal.white_pos.x > RESET_LIMIT {
                    removal.white_pos.y -= MOVEOVER; 
                    removal.white_pos.x = CAPTURE_START;
                }
                taken.translation.x = removal.white_pos.x; 
                taken.translation.y = removal.white_pos.y; 
                taken.scale.x = SCALER; 
                taken.scale.y = SCALER; 
                commands.entity(piece).remove::<Pickable>();
                
                removal.white_pos.x += MOVEOVER;
            }
        }
    }
}

fn take_black(
    taken_query: Query<(Entity, &mut Transform), (With<BlackPiece>, Without<Draggable>)>,
    captor_query: Single<&Transform, (With<Draggable>, With<WhitePiece>)>, 
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>, 
    mut removal: ResMut<CaptureZones>,
) {
    let capture = captor_query.into_inner(); 
    for (piece, mut taken) in taken_query.into_iter() {
        let collision = check_for_collisions(
            Aabb2d::new(    
                taken.translation.truncate(),
                Vec2::new(PIECESIZE, PIECESIZE),
            ),
            Aabb2d::new(
                capture.translation.truncate(),
                Vec2::new(PIECESIZE, PIECESIZE),
            )
        ); 

        if collision {
            if key.just_pressed(KeyCode::KeyX) {
                if removal.black_pos.x > RESET_LIMIT {
                    removal.black_pos.y -= MOVEOVER; 
                    removal.black_pos.x = CAPTURE_START;
                }
                // Moving pieces off board
                taken.translation.x = removal.black_pos.x; 
                taken.translation.y = removal.black_pos.y; 
                taken.scale.x = SCALER; 
                taken.scale.y = SCALER; 
                commands.entity(piece).remove::<Pickable>();
                
                removal.black_pos.x += MOVEOVER;
            }
        }
    }
}

fn check_for_collisions(
    taken: Aabb2d,
    captor: Aabb2d,
) -> bool {
    if captor.intersects(&taken) {
        true
    }
    else {
        false
    }
}