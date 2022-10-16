use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::{get_tile_neighbors, TilemapType},
    tiles::{TileColor, TilePos, TileStorage},
};

use crate::components::{
    cell_state::{CellState, CurrentState, NextState},
    colors::Colors,
};

pub fn update_state(
    mut tile_query: Query<(&mut CurrentState, &NextState, &mut TileColor), Changed<NextState>>,
) {
    for (mut current_state, next_state, mut tile_color) in tile_query.iter_mut() {
        match next_state.0 {
            CellState::Conductor => tile_color.0 = Colors::Conductor,
            CellState::Tail => tile_color.0 = Colors::Tail,
            CellState::Head => tile_color.0 = Colors::Head,
            CellState::Empty => {}
        }
        current_state.0 = next_state.0;
    }
}

pub fn do_state(
    mut commands: Commands,
    tile_query: Query<(Entity, &TilePos, &CurrentState)>,
    tilemap_query: Query<(&TileStorage, &TilemapType)>,
) {
    let (tile_storage, tilemap_type) = tilemap_query.single();

    for (entity, tile_pos, current_state) in tile_query.iter() {
        match current_state.0 {
            CellState::Conductor => {
                let heads = get_tile_neighbors(tile_pos, tile_storage, tilemap_type)
                    .into_iter()
                    .filter(|neighbor| {
                        let state = tile_query.get_component::<CurrentState>(*neighbor).unwrap();
                        state.0 == CellState::Head
                    })
                    .count();
                if heads == 1 || heads == 2 {
                    // Turn on this cell
                    commands.entity(entity).insert(NextState(CellState::Head));
                }
            }
            CellState::Tail => {
                // Turn of this cell
                commands
                    .entity(entity)
                    .insert(NextState(CellState::Conductor));
            }
            CellState::Head => {
                // De-Energize this cell
                commands.entity(entity).insert(NextState(CellState::Tail));
            }
            CellState::Empty => {}
        }
    }
}
