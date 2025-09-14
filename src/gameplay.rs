use bevy::prelude::*; 
use bevy::math::bounding::{Aabb2d, IntersectsVolume}; 
use super::cursor::*;
use super::pieces::*;
use super::position::*;

const PIECESIZE: f32 = 37.5;
const MOVEOVER: f32 = 40.0; 
const RESET_LIMIT: f32 = -450.0; 
const SCALER: f32 = 0.40; 

pub struct GameplayPlugin; 
impl Plugin for GameplayPlugin {
    fn build (&self, app: &mut App) {
        app
            .insert_resource( Previous {position: Vec2::new(0.0, 0.0)} )
            .add_systems(Update, (grab.after(update_cursor_pos), drag).chain())
            .add_systems(Update, (drop, (illegal_black_mv, illegal_white_mv)).chain())
            .add_systems(Update, (take_white, take_black));
    }
}

#[derive(Resource)]
struct Previous{
    position: Vec2,
}

#[derive(Component)]
struct Draggable; 

#[derive(Component)]
struct Dropped;

fn drag(
    cursor_pos: Res<MouseWorldCoords>, 
    piece: Single<&mut Transform, With<Draggable>>
) {
    let mut transform = piece.into_inner(); 
    if let Some(pos) = cursor_pos.0 {
        transform.translation.x = pos.x;
        transform.translation.y = pos.y; 
        transform.translation.z = 2.0; 
    }
}

fn grab(
    mut commands: Commands,
    cursor_pos: Res<MouseWorldCoords>, 
    piece_query: Query<(Entity, &Transform), With<Movable>>, 
    dropped_query: Query<Entity, With<Dropped>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut previous: ResMut<Previous>,
) {
    for (piece, transform) in piece_query.iter() {
        let x_max = transform.translation.x + PIECESIZE; let y_max = transform.translation.y + PIECESIZE;
        let x_min = transform.translation.x - PIECESIZE; let y_min = transform.translation.y - PIECESIZE;
        
        if let Some(pos) = cursor_pos.0 {
            if (pos.x >= x_min && pos.x <= x_max) && (pos.y >= y_min && pos.y <= y_max) {
                if mouse.just_pressed(MouseButton::Left) {
                    for previous in dropped_query.iter() {
                         commands.entity(previous).remove::<Dropped>();
                    }
                    previous.position.x = transform.translation.x; 
                    previous.position.y = transform.translation.y; 
                    commands.entity(piece).insert(Draggable); 
                }
            }
        }
    }
}

fn drop(
    mut commands: Commands, 
    dragging_query: Single<(Entity, &mut Transform), With<Draggable>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let (piece, mut transform) = dragging_query.into_inner(); 
    if mouse.just_released(MouseButton::Left) {
        transform.translation.z = 1.0; 
        commands.entity(piece).remove::<Draggable>();
        commands.entity(piece).insert(Dropped);
    }
}

fn take_white(
    mut commands: Commands,
    mut removal: ResMut<CaptureZones>,
    taken_query: Query<(Entity, &mut Transform),  (With<WhitePiece>, Without<Dropped>)>,
    captor_query: Single<&Transform, (With<Dropped>, With<BlackPiece>)>,  
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
            if removal.white_pos.x > RESET_LIMIT {
                removal.white_pos.y -= MOVEOVER; 
                removal.white_pos.x = CAPTURE_START;
            }
            taken.translation.x = removal.white_pos.x; taken.translation.y = removal.white_pos.y; 
            taken.scale.x = SCALER; taken.scale.y = SCALER; 
            removal.white_pos.x += MOVEOVER;
            commands.entity(piece).remove::<Movable>(); 
        }
    }
}

fn take_black(
    mut commands: Commands,
    mut removal: ResMut<CaptureZones>,
    taken_query: Query<(Entity, &mut Transform), (With<BlackPiece>, Without<Dropped>)>,
    captor_query: Single<&Transform, (With<Dropped>, With<WhitePiece>)>, 
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
            if removal.black_pos.x > RESET_LIMIT {
                removal.black_pos.y -= MOVEOVER; 
                removal.black_pos.x = CAPTURE_START;
            }
            taken.translation.x = removal.black_pos.x; taken.translation.y = removal.black_pos.y; 
            taken.scale.x = SCALER; taken.scale.y = SCALER; 
            removal.black_pos.x += MOVEOVER;
            commands.entity(piece).remove::<Movable>(); 
        }
    }
}

fn illegal_black_mv( 
    previous: Res<Previous>, 
    friendly_query: Query<&Transform, (With<BlackPiece>, Without<Dropped>)>, 
    captor_query: Single<&mut Transform, (With<BlackPiece>, With<Dropped>)>, 
) {
    let mut captor_transform = captor_query.into_inner(); 
    for friendly_transfrom in friendly_query.into_iter() {
        let friendly_collision = check_for_collisions(
            Aabb2d::new(friendly_transfrom.translation.truncate(),
             Vec2::new(PIECESIZE, PIECESIZE)), 
            Aabb2d::new(captor_transform.translation.truncate(),
            Vec2::new(PIECESIZE, PIECESIZE)), 
        ); 

        if friendly_collision {
            captor_transform.translation.x = previous.position.x; 
            captor_transform.translation.y = previous.position.y; 
        }
    }
}

fn illegal_white_mv(
    previous: Res<Previous>, 
    friendly_query: Query<&Transform, (With<WhitePiece>, Without<Dropped>)>, 
    captor_query: Single<&mut Transform, (With<WhitePiece>, With<Dropped>)>, 
) {
    let mut captor_transform = captor_query.into_inner(); 
    for friendly_transfrom in friendly_query.into_iter() {
        let friendly_collision = check_for_collisions(
            Aabb2d::new(friendly_transfrom.translation.truncate(),
             Vec2::new(PIECESIZE, PIECESIZE)), 
            Aabb2d::new(captor_transform.translation.truncate(),
            Vec2::new(PIECESIZE, PIECESIZE)), 
        ); 

        if friendly_collision {
            captor_transform.translation.x = previous.position.x; 
            captor_transform.translation.y = previous.position.y; 
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