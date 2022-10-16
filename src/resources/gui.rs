use bevy::prelude::{Component, Entity};

#[derive(Component, Copy, Clone)]
pub struct Gui {
    pub cursor: Entity,
    pub pause_icon: Entity,
    pub place_icon: Entity,
    pub power_icon: Entity,
    pub select_icon: Entity,
    pub move_icon: Entity,
    pub highlighted_toolbar_icon: Entity,
}

fn invalid() -> Entity {
    Entity::from_raw(0)
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            cursor: invalid(),
            pause_icon: invalid(),
            place_icon: invalid(),
            power_icon: invalid(),
            select_icon: invalid(),
            move_icon: invalid(),
            highlighted_toolbar_icon: invalid(),
        }
    }
}
