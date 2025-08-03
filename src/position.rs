use bevy::prelude::*;
use std::collections::HashMap; 

type BoardPosition = String; 
type Coordinates = (f32, f32); 
const START_POS: f32 = -350.0;

#[derive(Resource)]
pub struct Placement {
    pub positions: HashMap<BoardPosition, Coordinates>,
}

pub fn setup_placement(mut commands: Commands) {
    let mut map: HashMap<String, (f32, f32)> = HashMap::new(); 
    let mut y_pos: f32 = START_POS;
    
    for row in 0..8 {
        let mut x_pos: f32 = START_POS;
         
        for col in 1..=8 {
            let row_letter = (b'A' + row) as char;
            map.insert(format!("{}{}", row_letter, col), (x_pos, y_pos)); 
            x_pos += 100.0; 
        }
        y_pos += 100.0;
    }

    commands.insert_resource(Placement {positions: map});
}

pub struct PositionPlugin;
impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_placement);
    }
}