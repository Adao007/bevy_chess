use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod board;
use board::BoardPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BoardPlugin,
        ))
        .add_plugins(TilemapPlugin)
        .run();
}

