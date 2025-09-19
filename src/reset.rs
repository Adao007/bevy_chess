use bevy::prelude::*; 

pub struct ResetPlugin; 
impl Plugin for ResetPlugin {
    fn build(&self, app: &mut App) {

    }
}

struct Helper; 
impl Helper {
    fn despawn() {
        Self::despawn_black_pieces(); 
        Self::despawn_white_pieces(); 
        Self::despawn_pawns(); 
    }

    fn despawn_black_pieces() {

    }

    fn despawn_white_pieces() {

    }

    fn despawn_pawns() {
        
    }
}

// Despawn black pieces and white pieces. 
fn reset(
    keys: Res<ButtonInput<KeyCode>>, 
) {
    Helper::despawn(); 
}