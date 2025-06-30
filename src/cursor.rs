use bevy::prelude::*;

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build (&self, app: &mut App) {

    }
}

#[derive(Resource)]
pub struct CursorPos(Vec2); 
impl Default for CursorPos {
    fn default() -> Self {
        // The cursor pos will be initialized to a far away place. It will be updated when the mouse moves. 
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

// Cursor position will be updated on any 'CursorMoved' events. 
pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.read() {
        for (cam_t, cam) in camera_q.iter() {
            if let Ok(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}