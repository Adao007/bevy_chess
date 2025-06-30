use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod board;
use board::BoardPlugin;
mod cursor;
use cursor::CursorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BoardPlugin,
            CursorPlugin,
        ))
        .add_plugins(TilemapPlugin)
        .run();
}

