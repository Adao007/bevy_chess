#![allow(unused_variables)]

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod window;
use window::WindowsPlugin;
mod board;
use board::BoardPlugin;
mod cursor;
use cursor::CursorPlugin;
mod pieces;
use pieces::PiecesPlugin;
mod position;
use position::PositionPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WindowsPlugin,
            BoardPlugin,
            PositionPlugin,
            CursorPlugin,
            PiecesPlugin,
        ))
        .add_plugins(TilemapPlugin)
        .run();
}

