use bevy::prelude::*; 
use super::pieces::*;
use super::position::*;

pub struct ResetPlugin; 
impl Plugin for ResetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, reset);
    }
}

struct Helper; 
impl Helper {
    fn despawn_pieces<T: Component> (
        commands: &mut Commands,
        query: Query<Entity, With<T>>,
    ) {
        for piece in query.iter() {
            commands.entity(piece).despawn();
        }
    }
}

// Despawn black pieces and white pieces. 
fn reset(
    mut commands: Commands, 
    black_query: Query<Entity, With<BlackPiece>>, 
    white_query: Query<Entity, With<WhitePiece>>, 
    keys: Res<ButtonInput<KeyCode>>, 
    asset_server: Res<AssetServer>,
    board: Res<Placement>,
) {
        if keys.pressed(KeyCode::Escape) {
            Helper::despawn_pieces::<BlackPiece>(&mut commands, black_query); 
            Helper::despawn_pieces::<WhitePiece>(&mut commands, white_query); 
            reset_pieces(commands, asset_server, board);
        }
}

fn reset_pieces(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    board: Res<Placement>,
) {
    spawn_white_pieces(&mut commands, &asset_server, &board);
    spawn_pawns(&mut commands, &asset_server, &board);
    spawn_black_pieces(&mut commands, &asset_server, &board);
}
