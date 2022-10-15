use bevy::prelude::KeyCode;

pub struct Controls;

impl Controls {
    pub const PlaceKey: KeyCode = KeyCode::Key1;
    pub const EnergizeKey: KeyCode = KeyCode::Key2;
    pub const RemoveKey: KeyCode = KeyCode::Key3;
    pub const DrainKey: KeyCode = KeyCode::Key4;
    pub const PauseKey: KeyCode = KeyCode::Space;
    pub const DragKey: KeyCode = KeyCode::LControl;
    pub const SpeedKey: KeyCode = KeyCode::LShift;
    pub const MoveUp: KeyCode = KeyCode::W;
    pub const MoveRight: KeyCode = KeyCode::D;
    pub const MoveDown: KeyCode = KeyCode::S;
    pub const MoveLeft: KeyCode = KeyCode::A;
}
