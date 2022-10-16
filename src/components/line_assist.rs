use bevy::prelude::Component;

use super::{position::Position, area::Area};

#[derive(Component)]
pub struct LineAssist {
    pub area: Area,
    pub drawing: bool,
    pub origin: Position,
}

impl Default for LineAssist {
    fn default() -> Self {
        LineAssist {
            area: Default::default(),
            drawing: false,
            origin: Default::default(),
        }
    }
}


