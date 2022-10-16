use bevy::prelude::Component;

use super::{area::Area, position::Position};

#[derive(Component)]
pub struct Selection {
    pub area: Area,
    pub selecting: bool,
    pub active: bool,
    pub origin: Position,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            area: Default::default(),
            selecting: false,
            active: false,
            origin: Default::default(),
        }
    }
}
