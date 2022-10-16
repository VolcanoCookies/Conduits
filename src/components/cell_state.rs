use bevy::prelude::Component;

#[derive(Component, Eq, PartialEq, Debug, Clone, Copy)]
pub enum CellState {
    Conductor,
    Tail,
    Head,
    Empty,
}

#[derive(Component)]
pub struct NextState(pub CellState);

#[derive(Component)]
pub struct CurrentState(pub CellState);