use bevy::prelude::Component;

use crate::components::area::Area;

use super::controls::ControlMode;

#[derive(Component)]
pub struct AreaActionEvent {
    pub area: Area,
    pub action: AreaAction,
}

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub enum AreaAction {
    Place,
    Energize,
    Drain,
    Delete,
}

impl From<ControlMode> for AreaAction {
    fn from(c: ControlMode) -> Self {
        match c {
            ControlMode::Select => panic!("Cannot turn ControlMode::Select into a AreaAction"),
            ControlMode::Move => panic!("Cannot turn ControlMode::Select into a AreaAction"),
            ControlMode::Place => AreaAction::Place,
            ControlMode::Energize => AreaAction::Energize,
            ControlMode::Drain => AreaAction::Drain,
            ControlMode::Delete => AreaAction::Delete,
        }
    }
}
