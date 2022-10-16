use bevy::prelude::{Component, KeyCode, MouseButton};

#[derive(Component)]
pub struct Controls {
    pub mode: ControlMode,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            mode: ControlMode::Move,
        }
    }
}

impl Controls {
    pub const PlaceMode: KeyCode = KeyCode::W;
    pub const EnergizeMode: KeyCode = KeyCode::E;

    pub const DeleteMode: KeyCode = KeyCode::D;
    pub const DrainMode: KeyCode = KeyCode::S;
    pub const SelectMode: KeyCode = KeyCode::T;

    pub const MoveMode: KeyCode = KeyCode::M;

    pub const PauseKey: KeyCode = KeyCode::Space;
    pub const SelectArea: KeyCode = KeyCode::LControl;
    pub const DrawLine: KeyCode = KeyCode::LShift;
    pub const SpeedKey: KeyCode = KeyCode::LShift;
    pub const MoveUp: KeyCode = KeyCode::PageUp;
    pub const MoveRight: KeyCode = KeyCode::Right;
    pub const MoveDown: KeyCode = KeyCode::Down;
    pub const MoveLeft: KeyCode = KeyCode::Left;
    pub const Unselect: KeyCode = KeyCode::Escape;
    pub const FillSelection: KeyCode = KeyCode::F;
    pub const Cancel: KeyCode = KeyCode::Escape;

    pub const MouseMove: MouseButton = MouseButton::Middle;
    pub const MousePrimary: MouseButton = MouseButton::Left;
    pub const MouseSecondary: MouseButton = MouseButton::Right;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ControlMode {
    Select,
    Move,
    Place,
    Energize,
    Drain,
    Delete,
}

impl ControlMode {
    pub fn any(&self, i: impl IntoIterator<Item = ControlMode>) -> bool {
        i.into_iter().any(|c| c == *self)
    }

    pub fn actionable(&self) -> bool {
        match self {
            ControlMode::Place => true,
            ControlMode::Energize => true,
            ControlMode::Drain => true,
            ControlMode::Delete => true,
            _ => false,
        }
    }
}
