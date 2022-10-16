use bevy::prelude::*;

use crate::components::position::Position;

#[derive(Component)]
pub struct CursorMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct Cursor {
    pub mode: CursorMode,
    pub selection_origin: Position,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            mode: CursorMode::Place,
            selection_origin: Position::default(),
        }
    }
}

#[derive(Component, Debug, Eq, PartialEq, Clone, Copy)]
pub enum SelectMode {
    Area,
    Axis,
}

#[derive(Component, Debug, Eq, PartialEq, Clone, Copy)]
pub enum CursorMode {
    Place,
    Energize,
    Delete,
    // Turn heads and tails back into conductors
    Drain,
}

impl Default for CursorMode {
    fn default() -> Self {
        Self::Place
    }
}
