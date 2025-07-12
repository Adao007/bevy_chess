#![allow(unused_variables)]

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

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
            PositionPlugin,
            BoardPlugin,
            CursorPlugin,
            PiecesPlugin,
        ))
        .add_plugins(TilemapPlugin)
        .run();
}

