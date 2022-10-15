use bevy::{prelude::*, sprite::Sprite};

use crate::components::{
    cell::Tickable,
    cell_state::{CellState, NextState},
    colors::Colors,
    direction::Direction,
    position::Position,
    state_map::StateMap,
};

pub fn update_state(
    mut commands: Commands,
    mut state_map: ResMut<StateMap>,
    mut cell_query: Query<
        (&Position, &NextState, &mut Sprite),
        (With<Tickable>, Changed<NextState>),
    >,
) {
    for (position, next_state, mut sprite) in cell_query.iter_mut() {
        state_map.set(position.0, next_state.0);
        match next_state.0 {
            CellState::Conductor => sprite.color = Colors::Conductor,
            CellState::Tail => sprite.color = Colors::Tail,
            CellState::Head => sprite.color = Colors::Head,
            CellState::Empty => {}
        }
    }
}

pub fn do_state(
    state_map: Res<StateMap>,
    mut cell_query: Query<(&Position, &mut NextState), With<Tickable>>,
) {
    for (position, mut next_state) in cell_query.iter_mut() {
        let state = state_map.get(position.0);

        match state {
            CellState::Conductor => {
                let mut heads: i32 = 0;
                for dir in Direction::All {
                    if state_map.get(position.0 + dir) == CellState::Head {
                        heads += 1;
                    }
                }
                if heads == 1 || heads == 2 {
                    // Turn on this cell
                    next_state.0 = CellState::Head;
                }
            }
            CellState::Tail => {
                // Turn of this cell
                next_state.0 = CellState::Conductor
            }
            CellState::Head => {
                // De-Energize this cell
                next_state.0 = CellState::Tail
            }
            CellState::Empty => {}
        }
    }
}
