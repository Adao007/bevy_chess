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
    commands.insert_resource(Placement {positions: map});
}

pub struct PositionPlugin;
impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_placement);
    }
}