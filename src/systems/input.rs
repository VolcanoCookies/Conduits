use bevy::{prelude::*, sprite::Anchor};

use crate::{
    components::{
        cell::{Tickable, CELL_SIZE},
        cell_map::CellMap,
        cell_state::{CellState, NextState},
        colors::Colors,
        cursor::{Cursor, CursorMarker, CursorMode, DragEvent},
        position::Position,
        state_map::StateMap,
    },
    resources::controls::Controls,
    AppState, SIZE,
};

use super::camera::MousePosition;

pub fn handle_input(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    mut cursor: ResMut<Cursor>,
    app_state: Res<AppState>,
    mouse_button: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut drag_events: EventWriter<DragEvent>,
) {
    if keys.pressed(Controls::DragKey) && mouse_button.just_pressed(MouseButton::Left) {
        cursor.dragging = true;
        cursor.dragging_origin = mouse_position.cell;
    } else if cursor.dragging
        && (mouse_button.just_released(MouseButton::Left)
            || mouse_button.just_pressed(MouseButton::Right))
    {
        // Fire end of dragging event
        drag_events.send(DragEvent {
            start_cell: cursor.dragging_origin,
            end_cell: mouse_position.cell,
            cancelled: mouse_button.just_pressed(MouseButton::Right),
        });
        cursor.dragging = false;
        cursor.just_stopped_dragging = true;
    }

    if cursor.just_stopped_dragging && !mouse_button.pressed(MouseButton::Left) {
        cursor.just_stopped_dragging = false;
    }

    if !cursor.dragging {
        if keys.just_pressed(Controls::PlaceKey) {
            cursor.mode = CursorMode::Place;
            info!("Set cursor mode to {:?}", CursorMode::Place);
        } else if keys.just_pressed(Controls::EnergizeKey) {
            cursor.mode = CursorMode::Energize;
            info!("Set cursor mode to {:?}", CursorMode::Energize);
        } else if keys.just_pressed(Controls::RemoveKey) {
            cursor.mode = CursorMode::Remove;
            info!("Set cursor mode to {:?}", CursorMode::Remove);
        } else if keys.just_pressed(Controls::DrainKey) {
            cursor.mode = CursorMode::Drain;
            info!("Set cursor mode to {:?}", CursorMode::Drain);
        }
    }

    if keys.just_pressed(Controls::PauseKey) {
        if *app_state == AppState::Running {
            commands.insert_resource(AppState::Paused);
        } else {
            commands.insert_resource(AppState::Running);
        }
    }
}

pub fn edit_wire(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    mouse_button: Res<Input<MouseButton>>,
    cursor: Res<Cursor>,
    mut cell_map: ResMut<CellMap>,
    mut state_map: ResMut<StateMap>,
    mut cell_query: Query<&mut NextState>,
    mut drag_events: EventReader<DragEvent>,
) {
    if cursor.dragging {
        return;
    }

    for event in drag_events.iter() {
        let min = event.start_cell.min(event.end_cell).max(Vec2::splat(0.));
        let max = event.start_cell.max(event.end_cell).min(Vec2::splat(SIZE as f32));
        for x in (min.x as i32)..(max.x as i32 + 1) {
            for y in (min.y as i32)..(max.y as i32 + 1) {
                turn_cell(
                    Vec2::new(x as f32, y as f32),
                    &mut commands,
                    &mut *state_map,
                    &mut *cell_map,
                    &mut cell_query,
                    cursor.mode,
                );
            }
        }
    }

    if mouse_button.pressed(MouseButton::Left)
        && cell_map.inside(mouse_position.cell)
        && !cursor.just_stopped_dragging
    {
        turn_cell(
            mouse_position.cell,
            &mut commands,
            &mut *state_map,
            &mut *cell_map,
            &mut cell_query,
            cursor.mode,
        );
    }
}


fn turn_cell(
    pos: Vec2,
    commands: &mut Commands,
    state_map: &mut StateMap,
    cell_map: &mut CellMap,
    cell_query: &mut Query<&mut NextState>,
    mode: CursorMode,
) {
    if !state_map.inside(pos) {
        return;
    }

    let cell_state = state_map.get(pos);
    if cell_state == CellState::Empty && mode == CursorMode::Place {
        spawn_cell(commands, cell_map, state_map, pos, CellState::Conductor);
    } else if cell_state == CellState::Conductor && mode == CursorMode::Energize {
        let entity = cell_map.get(pos);
        let (mut next_state) = cell_query.get_mut(entity).unwrap();
        next_state.0 = CellState::Head;
        state_map.set(pos, CellState::Head);
    } else if cell_state != CellState::Empty && mode == CursorMode::Remove {
        let entity = cell_map.remove(pos);
        commands.entity(entity).despawn();
        state_map.set(pos, CellState::Empty);
    } else if (cell_state == CellState::Head || cell_state == CellState::Tail)
        && mode == CursorMode::Drain
    {
        let entity = cell_map.get(pos);
        let (mut next_state) = cell_query.get_mut(entity).unwrap();
        next_state.0 = CellState::Conductor;
        state_map.set(pos, CellState::Conductor);
    }
}

pub fn spawn_cell(
    commands: &mut Commands,
    cell_map: &mut CellMap,
    state_map: &mut StateMap,
    position: Vec2,
    state: CellState,
) {
    info!("Spawning {:?} at {}:{}", state, position.x, position.y);

    let entity = commands
        .spawn()
        .insert_bundle(SpatialBundle::default())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(position.x * CELL_SIZE, position.y * CELL_SIZE, 1.),
            ..default()
        })
        .insert(Tickable)
        .insert(Position(position))
        .insert(NextState(state))
        .id();

    cell_map.set(position, entity);
    state_map.set(position, state);
}
