use bevy::prelude::*;

use crate::{
    components::{
        cell::CELL_SIZE,
        colors::Colors,
        cursor::{Cursor, CursorMarker},
    },
    utils::closest_multiple,
    AppState,
};

use super::{
    camera::MousePosition,
    setup::{DragOverlayMarker, PauseIconMarker},
};

pub fn icon_toggles(
    app_state: Res<AppState>,
    cursor: Res<Cursor>,
    mouse_position: Res<MousePosition>,
    mut pause_icon_query: Query<
        &mut Visibility,
        (With<PauseIconMarker>, Without<DragOverlayMarker>),
    >,
    mut drag_overlay_query: Query<
        (&mut Transform, &mut Sprite, &mut Visibility),
        (With<DragOverlayMarker>, Without<PauseIconMarker>),
    >,
) {
    let mut visibility = pause_icon_query.single_mut();
    visibility.is_visible = *app_state == AppState::Paused;

    let (mut transform, mut sprite, mut visibility) = drag_overlay_query.single_mut();
    if cursor.dragging {
        visibility.is_visible = true;
        let mouse_translation = mouse_position.cell.extend(16.);
        let min_pos = mouse_translation.min(cursor.dragging_origin.extend(16.)) - 0.5;
        let max_pos = mouse_translation.max(cursor.dragging_origin.extend(16.)) + 0.5;
        transform.translation = min_pos * CELL_SIZE;
        sprite.custom_size = Some((max_pos - min_pos).truncate() * CELL_SIZE);
        sprite.color = match cursor.mode {
            crate::components::cursor::CursorMode::Place => Colors::DragOverlayPlace,
            crate::components::cursor::CursorMode::Energize => Colors::DragOverlayEnergize,
            crate::components::cursor::CursorMode::Remove => Colors::DragOverlayRemove,
            crate::components::cursor::CursorMode::Drain => Colors::DragOverlayDrain,
        }
    } else {
        visibility.is_visible = false;
    }
}

pub fn update_cursor(
    mouse_position: Res<MousePosition>,
    cursor: Res<Cursor>,
    mut cursor_query: Query<(&mut Transform, &mut Sprite), With<CursorMarker>>,
) {
    let (mut transform, mut sprite) = cursor_query.single_mut();
    transform.translation = (mouse_position.cell * CELL_SIZE).extend(2.);
    sprite.color = match cursor.mode {
        crate::components::cursor::CursorMode::Place => Colors::ConductorSelector,
        crate::components::cursor::CursorMode::Energize => Colors::EnergizeSelector,
        crate::components::cursor::CursorMode::Remove => Colors::RemoveSelector,
        crate::components::cursor::CursorMode::Drain => Colors::DrainSelector,
    }
}
