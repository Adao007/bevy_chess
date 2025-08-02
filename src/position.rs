use bevy::prelude::*;
use std::collections::HashMap; 

type BoardPosition = String; 
type Coordinates = (f32, f32); 

#[derive(Resource)]
pub struct Placement {
    pub positions: HashMap<BoardPosition, Coordinates>,
}

pub fn setup_placement(mut commands: Commands) {
    let mut map: HashMap<String, (f32, f32)> = HashMap::new(); 
    map.insert("A1".to_string(), (-350.0, -350.0));
    map.insert("A2".to_string(), (-250.0, -350.0));
    map.insert("A3".to_string(), (-150.0, -350.0));
    map.insert("A4".to_string(), (-50.0, -350.0));
    map.insert("A5".to_string(), (50.0, -350.0));
    map.insert("A6".to_string(), (150.0, -350.0));
    map.insert("A7".to_string(), (250.0, -350.0));
    map.insert("A8".to_string(), (350.0, -350.0));

    let mut x_pos: f32 = -350.0;
    let mut y_pos: f32 = -250.0;

    for i in 1..=8 {
        map.insert(format!("B{}", i), (x_pos, y_pos));
        x_pos += 100.0; 
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