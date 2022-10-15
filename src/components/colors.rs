use bevy::prelude::Color;

pub struct Colors;

impl Colors {
    pub const Conductor: Color = Color::WHITE;
    pub const Tail: Color = Color::BLUE;
    pub const Head: Color = Color::RED;

    pub const ConductorSelector: Color =
        Color::rgba(0.42352941176, 0.96078431372, 0.25882352941, 1.0);
    pub const EnergizeSelector: Color =
        Color::rgba(0.96078431372, 0.94901960784, 0.25882352941, 1.0);
    pub const RemoveSelector: Color = Color::rgba(0.98039215686, 0.15686274509, 0.34901960784, 1.0);
    pub const DrainSelector: Color = Color::rgba(0.25882352941, 0.28235294117, 0.96078431372, 1.0);

    pub const DragOverlayPlace: Color =
        Color::rgba(0.42352941176, 0.96078431372, 0.25882352941, 0.4);
    pub const DragOverlayEnergize: Color =
        Color::rgba(0.96078431372, 0.94901960784, 0.25882352941, 0.4);
    pub const DragOverlayRemove: Color =
        Color::rgba(0.98039215686, 0.15686274509, 0.34901960784, 0.4);
    pub const DragOverlayDrain: Color =
        Color::rgba(0.25882352941, 0.28235294117, 0.96078431372, 0.4);
}
