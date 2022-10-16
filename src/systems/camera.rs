use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{
    components::position::Position,
    resources::controls::{ControlMode, Controls},
    CELL_SIZE,
};

#[derive(Component, Default)]
pub struct MousePosition {
    pub world: Vec2,
    pub cell: Position,
    pub screen: Vec2,
}

#[derive(Component)]
pub struct CameraZoom(pub f32);

pub fn window_to_world_position(
    mouse_position: Vec2,
    windows: Res<Windows>,
    camera_transform: &Transform,
) -> Vec2 {
    let win = windows.primary();
    let win_dim = Vec2::new(win.width(), win.height());

    let pos = (mouse_position - (win_dim / 2.)) * camera_transform.scale.truncate()
        + camera_transform.translation.truncate();

    return pos.round();
}

// Slightly simpler constant rate zooming!
//zoom = zoom*pow(targetZoom/zoom, deltaTime/duration)
//
//// ..or as a function:
//float logerp(float a, float b, float t){
//    return a*pow(b/a, t);
//}

pub fn camera_movement(
    mut move_events: EventReader<CursorMoved>,
    mut scroll_events: EventReader<MouseWheel>,
    mut keys: Res<Input<KeyCode>>,
    mut mouse_button: Res<Input<MouseButton>>,
    controls: Res<Controls>,
    mut mouse_position: ResMut<MousePosition>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut windows: Res<Windows>,
    mut zoom: ResMut<CameraZoom>,
) {
    // Calculate camera zoom
    let mut zoom_delta = 0.;
    for event in scroll_events.iter() {
        zoom_delta += event.y as f32;
    }
    zoom.0 = zoom.0 * (0.125 / zoom.0).powf((zoom_delta as f32) / 20.);

    let mut delta = Vec2::ZERO;
    let dragging = (mouse_button.pressed(Controls::MousePrimary)
        && controls.mode == ControlMode::Move)
        || mouse_button.pressed(Controls::MouseMove);

    // Calculate new mouse positions
    for event in move_events.iter() {
        if dragging {
            delta = event.position - mouse_position.screen;
        }
        mouse_position.screen = event.position;
    }

    let (mut camera_transform, mut camera_projection) = camera_query.single_mut();

    mouse_position.world =
        window_to_world_position(mouse_position.screen, windows, &camera_transform);
    mouse_position.cell = (mouse_position.world / CELL_SIZE).round().into();

    // Move camera with keyboard
    if !dragging {
        if keys.pressed(Controls::MoveRight) {
            delta.x -= 4.;
        }
        if keys.pressed(Controls::MoveLeft) {
            delta.x += 4.;
        }
        if keys.pressed(Controls::MoveUp) {
            delta.y -= 4.;
        }
        if keys.pressed(Controls::MoveDown) {
            delta.y += 4.;
        }

        if keys.pressed(KeyCode::LShift) || keys.pressed(KeyCode::RShift) {
            delta *= 4.;
        }
    }

    // Apply changes to camera
    delta *= zoom.0;
    camera_transform.translation -= Vec3::new(delta.x, delta.y, 0.0);
    camera_transform.scale = Vec3::splat(zoom.0);
    camera_projection.far = 1000. / zoom.0;
}
