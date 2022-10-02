use crate::common::*;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

pub fn cursor_position_system(
    windows: Res<Windows>,
    mut mouse_position: ResMut<MousePosition>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };
    if let Some(screen_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let normalized_device_coordinates = (screen_position / window_size) * 2.0 - Vec2::ONE;
        let normalized_device_coordinates_to_world =
            camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_position = normalized_device_coordinates_to_world
            .project_point3(normalized_device_coordinates.extend(-1.0));
        mouse_position.position = game_position(world_position);
    }
}
