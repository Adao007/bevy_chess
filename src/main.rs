#![allow(unused_variables)]

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod window; use window::WindowsPlugin;
mod board; use board::BoardPlugin;
mod cursor; use cursor::CursorPlugin;
mod pieces; use pieces::PiecesPlugin;
mod position; use position::PositionPlugin;
mod gameplay; use gameplay::GameplayPlugin; 
mod reset; use reset::ResetPlugin; 

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WindowsPlugin,
            BoardPlugin,
            PositionPlugin,
            CursorPlugin,
            PiecesPlugin,
            GameplayPlugin,
            ResetPlugin, 
        ))
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, spawn_text)
        .run();
}

fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
        .with_child(Text::new(concat!(
            "Press 4 to which to windowed, and 5 to switch to fullscreen\n",
            "Press 1, 2, 3 to change resolution sizes\n",
            "Press esc to reset the board\n",
            "Press Q(ueen), R(ook), B(ishop), K(night) to promote pawns."
        )));
}
