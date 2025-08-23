use bevy::prelude::*;
use super::board::*;

#[derive(Resource, Default)]
pub struct MouseCoords(Option<Vec2>);

#[derive(Resource, Debug, Default, Deref)]
pub struct MouseWorldCoords(pub Option<Vec2>);

// Cursor position will be updated on any 'CursorMoved' events. 
pub fn update_cursor_pos(
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Single<&Window>,
    mut mouse_coords: ResMut<MouseCoords>,
    mut mouse_world_coords: ResMut<MouseWorldCoords>,
) {
    mouse_coords.0 = window.cursor_position();
    mouse_world_coords.0 = window.cursor_position().map(|pos| {
    let (camera, camera_transform) = camera.into_inner();
        camera
            .viewport_to_world_2d(camera_transform, pos)
            .unwrap_or(vec2(0.0, 0.0))
    });
}

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build (&self, app: &mut App) {
        app
            .init_resource::<MouseCoords>()
            .init_resource::<MouseWorldCoords>()
            .add_systems(Update, update_cursor_pos);
    }
}