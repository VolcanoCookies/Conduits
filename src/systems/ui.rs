use bevy::prelude::*;

use crate::{
    components::{colors::Colors, line_assist::LineAssist, selection::Selection},
    resources::{
        controls::{ControlMode, Controls},
        cursor::{Cursor, CursorMarker},
        gui::Gui,
    },
    AppState, CELL_SIZE,
};

use super::{
    camera::MousePosition,
    setup::{
        DragTextHorizontalMarker, DragTextVerticalMarker, LineAssistOverlayMarker, PauseIconMarker,
        SelectionOverlayMarker,
    },
};

pub fn icon_toggles(
    app_state: Res<AppState>,
    cursor: Res<Cursor>,
    mouse_position: Res<MousePosition>,
    selection: Res<Selection>,
    line_assist: Res<LineAssist>,
    controls: Res<Controls>,
    mut gui: ResMut<Gui>,
    mut pause_icon_query: Query<
        &mut Visibility,
        (
            With<PauseIconMarker>,
            Without<SelectionOverlayMarker>,
            Without<LineAssistOverlayMarker>,
        ),
    >,
    mut line_assist_overlay_query: Query<
        (&mut Transform, &mut Sprite, &mut Visibility),
        (
            With<LineAssistOverlayMarker>,
            Without<SelectionOverlayMarker>,
            Without<PauseIconMarker>,
        ),
    >,
    mut selection_area_overlay_query: Query<
        (&mut Transform, &mut Sprite, &mut Visibility),
        (
            With<SelectionOverlayMarker>,
            Without<LineAssistOverlayMarker>,
            Without<PauseIconMarker>,
        ),
    >,
    mut drag_overlay_text_query: Query<
        (&mut Text, Option<&DragTextHorizontalMarker>),
        Or<(With<DragTextHorizontalMarker>, With<DragTextVerticalMarker>)>,
    >,
    mut ui_color_query: Query<(&mut UiColor), With<UiImage>>,
) {
    let mut visibility = pause_icon_query.single_mut();
    visibility.is_visible = *app_state == AppState::Paused;

    let (mut line_assist_transform, mut line_assist_sprite, mut line_assist_visibility) =
        line_assist_overlay_query.single_mut();
    if line_assist.drawing {
        line_assist_sprite.color = Colors::SelectionOverlay;
        line_assist_visibility.is_visible = true;

        line_assist_transform.translation =
            (Vec2::from(line_assist.area.a) - 0.5).extend(16.) * CELL_SIZE;
        line_assist_sprite.custom_size = Some(Vec2::from(line_assist.area.size()) * CELL_SIZE);
    } else {
        line_assist_visibility.is_visible = false;
    }

    let (mut selection_transform, mut selection_sprite, mut selection_visibility) =
        selection_area_overlay_query.single_mut();
    if selection.active {
        selection_sprite.color = Colors::SelectionOverlay;
        selection_visibility.is_visible = true;

        selection_transform.translation =
            (Vec2::from(selection.area.a) - 0.5).extend(16.) * CELL_SIZE;
        selection_sprite.custom_size = Some(Vec2::from(selection.area.size()) * CELL_SIZE);
    } else {
        selection_visibility.is_visible = false;
    }

    //        for (mut text, horizontal) in drag_overlay_text_query.iter_mut() {
    //            if *horizontal == None {
    //                // Vertical text
    //            } else {
    //                // Horizontal text
    //                text
    //            }
    //        }
}

pub fn update_toolbar_icons(
    controls: Res<Controls>,
    mut gui: ResMut<Gui>,
    mut ui_color_query: Query<(&mut UiColor), With<UiImage>>,
) {
    if controls.is_changed() {
        let (entity, color) = match controls.mode {
            ControlMode::Select => (gui.select_icon, Color::GREEN),
            ControlMode::Move => (gui.move_icon, Color::GREEN),
            ControlMode::Place => (gui.place_icon, Color::GREEN),
            ControlMode::Energize => (gui.power_icon, Color::GREEN),
            ControlMode::Drain => (gui.power_icon, Color::RED),
            ControlMode::Delete => (gui.place_icon, Color::RED),
        };
        let mut ui_old_color = ui_color_query.get_mut(gui.select_icon).unwrap();
        ui_old_color.0 = Color::default();
        let mut ui_color = ui_color_query.get_mut(entity).unwrap();
        ui_color.0 = color;
        gui.select_icon = entity;
    }
}

pub fn update_cursor(
    mouse_position: Res<MousePosition>,
    gui: Res<Gui>,
    mut cursor_query: Query<(&mut Transform, &mut Sprite)>,
) {
    if mouse_position.is_changed() {
        let (mut transform, mut sprite) = cursor_query.get_mut(gui.cursor).unwrap();
        transform.translation = (Vec2::from(mouse_position.cell) * CELL_SIZE).extend(2.);
    }

    //    sprite.color = match controls.mode {
    //        ControlMode::Place => Colors::ConductorSelector,
    //        ControlMode::Energize => Colors::EnergizeSelector,
    //        ControlMode::Delete => Colors::RemoveSelector,
    //        ControlMode::Drain => Colors::DrainSelector,
    //        ControlMode::Select => Colors::ConductorSelector,
    //        ControlMode::Move => Colors::Conductor,
    //    }
}
