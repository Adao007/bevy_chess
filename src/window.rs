use bevy::prelude::*; 
use bevy::window::{PrimaryWindow, Window, WindowMode}; 

pub struct WindowsPlugin; 
impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ResolutionSettings {
                large: Vec2::new(1920.0, 1080.0),
                medium: Vec2::new(1600.0, 1080.0),
                small: Vec2::new(1040.0, 1080.0),
            })
            .add_systems(Startup, set_fullscreen)
            .add_systems(Update, toggle_resolution); 
    }
}

#[derive(Resource)]
pub struct ResolutionSettings{
    large: Vec2,
    medium: Vec2,
    small: Vec2, 
}

fn set_fullscreen(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>
) {
    for mut window in window_query.iter_mut() {
        window.mode = 
            WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    }
}

fn toggle_resolution(
    keys: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        let res = resolution.small; 
        window.resolution.set(res.x, res.y); 
    }
    if keys.just_pressed(KeyCode::Digit2) {
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        let res = resolution.large; 
        window.resolution.set(res.x, res.y); 
    }
}
