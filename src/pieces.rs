use super::position::*;
use bevy::prelude::*;

// Edit piece png(s) here

// Black Pieces PNG
const BLACK_PAWN_IMAGE: &str = "dandelion.png";
const BLACK_ROOK_IMAGE: &str = "black_rook.png";
const BLACK_KNIGHT_IMAGE: &str = "clove.png";
const BLACK_BISHOP_IMAGE: &str = "black_bishop.png";
const BLACK_QUEEN_IMAGE: &str = "black_queen.png";
const BLACK_KING_IMAGE: &str = "black_king.png";
// White Pieces PNG
const WHITE_PAWN_IMAGE: &str = "caterpillar.png";
const WHITE_ROOK_IMAGE: &str = "white_rook.png";
const WHITE_KNIGHT_IMAGE: &str = "white_knight.png";
const WHITE_BISHOP_IMAGE: &str = "ladybug.png";
const WHITE_QUEEN_IMAGE: &str = "white_queen.png";
const WHITE_KING_IMAGE: &str = "white_king.png";

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pieces.after(setup_placement))
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

fn spawn_pieces(mut commands: Commands, asset_server: Res<AssetServer>, board: Res<Placement>) {
    spawn_white_pieces(&mut commands, &asset_server, &board);
    spawn_pawns(&mut commands, &asset_server, &board);
    spawn_black_pieces(&mut commands, &asset_server, &board);
}

pub fn spawn_black_pieces(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    board: &Res<Placement>,
) {
    let names = vec![
        BLACK_ROOK_IMAGE,
        BLACK_KNIGHT_IMAGE,
        BLACK_BISHOP_IMAGE,
        BLACK_QUEEN_IMAGE,
        BLACK_KING_IMAGE,
        BLACK_BISHOP_IMAGE,
        BLACK_KNIGHT_IMAGE,
        BLACK_ROOK_IMAGE,
    ];
    let types = vec![
        PieceType::Rook,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::King,
        PieceType::Queen,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Rook,
    ];

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

pub fn spawn_white_pieces(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    board: &Res<Placement>,
) {
    let names = vec![
        WHITE_ROOK_IMAGE,
        WHITE_KNIGHT_IMAGE,
        WHITE_BISHOP_IMAGE,
        WHITE_QUEEN_IMAGE,
        WHITE_KING_IMAGE,
        WHITE_BISHOP_IMAGE,
        WHITE_KNIGHT_IMAGE,
        WHITE_ROOK_IMAGE,
    ];
    let types = vec![
        PieceType::Rook,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Queen,
        PieceType::King,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Rook,
    ];

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

pub fn spawn_pawns(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    board: &Res<Placement>,
) {
    // Spawning the White Pawns
    for i in 1..=8 {
        if let Some(&(x, y)) = board.positions.get(&format!("B{}", i)) {
            commands.spawn((
                Sprite::from_image(asset_server.load(WHITE_PAWN_IMAGE)),
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
                Sprite::from_image(asset_server.load(BLACK_PAWN_IMAGE)),
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
        if (piece.translation.y <= -300.0 && piece.translation.y >= -400.0)
            && (piece.translation.x <= 400.0 && piece.translation.x >= -400.0)
        {
            //*sprite = Sprite::from_image(asset_server.load("black_queen.png"));
            if keys.just_released(KeyCode::KeyQ) {
                *sprite = Sprite::from_image(asset_server.load(BLACK_QUEEN_IMAGE));
                commands.entity(entity).remove::<Pawn>();
            }
            if keys.just_released(KeyCode::KeyR) {
                *sprite = Sprite::from_image(asset_server.load(BLACK_ROOK_IMAGE));
                commands.entity(entity).remove::<Pawn>();
            }
            if keys.just_released(KeyCode::KeyK) {
                *sprite = Sprite::from_image(asset_server.load(BLACK_KNIGHT_IMAGE));
                commands.entity(entity).remove::<Pawn>();
            }
            if keys.just_released(KeyCode::KeyB) {
                *sprite = Sprite::from_image(asset_server.load(BLACK_BISHOP_IMAGE));
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
        if (piece.translation.y >= 300.0 && piece.translation.y <= 400.0)
            && (piece.translation.x <= 400.0 && piece.translation.x >= -400.0)
        {
            if keys.just_released(KeyCode::KeyQ) {
                *sprite = Sprite::from_image(asset_server.load(WHITE_QUEEN_IMAGE));
                commands.entity(entity).remove::<Pawn>();
            }
            if keys.just_released(KeyCode::KeyB) {
                *sprite = Sprite::from_image(asset_server.load(WHITE_BISHOP_IMAGE));
                commands.entity(entity).remove::<Pawn>();
            }
            if keys.just_released(KeyCode::KeyK) {
                *sprite = Sprite::from_image(asset_server.load(WHITE_KNIGHT_IMAGE));
                commands.entity(entity).remove::<Pawn>();
            }
            if keys.just_released(KeyCode::KeyR) {
                *sprite = Sprite::from_image(asset_server.load(WHITE_ROOK_IMAGE));
                commands.entity(entity).remove::<Pawn>();
            }
        }
    }
}
