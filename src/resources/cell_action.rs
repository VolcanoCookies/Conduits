use super::{controls::ControlMode, area_action_event::AreaAction};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CellAction {
    Place,
    Energize,
    Drain,
    Delete,
}

impl From<ControlMode> for CellAction {
    fn from(c: ControlMode) -> Self {
        match c {
            ControlMode::Select => panic!("Cannot turn ControlMode::Select into a CellAction"),
            ControlMode::Move => panic!("Cannot turn ControlMode::Move into a CellAction"),
            ControlMode::Place => CellAction::Place,
            ControlMode::Energize => CellAction::Energize,
            ControlMode::Drain => CellAction::Drain,
            ControlMode::Delete => CellAction::Delete,
        }
    }
}

impl From<CellAction> for ControlMode {
    fn from(c: CellAction) -> Self {
        match c {
            CellAction::Place => ControlMode::Place,
            CellAction::Energize => ControlMode::Energize,
            CellAction::Drain => ControlMode::Drain,
            CellAction::Delete => ControlMode::Delete,
        }
    }
}

impl From<AreaAction> for CellAction {
    fn from(a: AreaAction) -> Self {
        match a {
            AreaAction::Place => CellAction::Place,
            AreaAction::Energize => CellAction::Energize,
            AreaAction::Drain => CellAction::Drain,
            AreaAction::Delete => CellAction::Delete,
        }
    }
}