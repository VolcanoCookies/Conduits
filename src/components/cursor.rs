use bevy::prelude::*;

#[derive(Component)]
pub struct CursorMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct Cursor {
    pub mode: CursorMode,
    pub dragging: bool,
    pub dragging_origin: Vec2,
    pub just_stopped_dragging: bool,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            mode: CursorMode::Place,
            dragging: false,
            dragging_origin: Vec2::ZERO,
            just_stopped_dragging: false,
        }
    }
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum CursorMode {
    Place,
    Energize,
    Remove,
    // Turn heads and tails back into conductors
    Drain,
}

impl Default for CursorMode {
    fn default() -> Self {
        Self::Place
    }
}

#[derive(Component)]
pub struct DragEvent {
    pub start_cell: Vec2,
    pub end_cell: Vec2,
    pub cancelled: bool,
}
