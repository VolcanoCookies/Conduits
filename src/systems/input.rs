use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::TilemapId,
    tiles::{TileBundle, TilePos, TileStorage},
};

use crate::{
    components::{
        area::Area,
        cell_state::{CellState, CurrentState, NextState},
        line_assist::LineAssist,
        selection::Selection,
    },
    resources::{
        area_action_event::{AreaAction, AreaActionEvent},
        cell_action::CellAction,
        controls::{ControlMode, Controls},
        cursor::Cursor,
    },
    AppState, SIZE,
};

use super::camera::MousePosition;

pub fn handle_input(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    mut cursor: ResMut<Cursor>,
    mut selection: ResMut<Selection>,
    mut line_assist: ResMut<LineAssist>,
    mut controls: ResMut<Controls>,
    app_state: Res<AppState>,
    mouse_button: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut area_action_events: EventWriter<AreaActionEvent>,
) {
    if !line_assist.drawing
        && keys.pressed(Controls::SelectArea)
        && mouse_button.just_pressed(MouseButton::Left)
    {
        selection.selecting = true;
        selection.active = true;
        selection.origin = mouse_position.cell.into();
    } else if !selection.selecting
        && keys.pressed(Controls::DrawLine)
        && mouse_button.just_pressed(MouseButton::Left)
    {
        line_assist.drawing = true;
        line_assist.origin = mouse_position.cell.into();
    }

    if selection.selecting
        && (keys.just_released(Controls::SelectArea)
            || mouse_button.just_released(MouseButton::Left))
    {
        selection.selecting = false;
    } else if line_assist.drawing
        && (keys.just_released(Controls::DrawLine) || mouse_button.just_released(MouseButton::Left))
    {
        line_assist.drawing = false;
        if controls.mode != ControlMode::Select {
            area_action_events.send(AreaActionEvent {
                area: line_assist.area,
                action: controls.mode.into(),
            });
        }
    }

    if selection.selecting {
        selection.area = Area::new(selection.origin, mouse_position.cell).grow((1, 1).into());
    } else if line_assist.drawing {
        let a = line_assist.origin;
        let mut b = mouse_position.cell;
        let diff = (a - b).abs();
        if diff.x > diff.y {
            b.y = a.y;
        } else {
            b.x = a.x;
        }
        line_assist.area = Area::new(a, b).grow((1, 1).into());
    }

    for key in keys.get_just_pressed() {
        match *key {
            Controls::PlaceMode => controls.mode = ControlMode::Place,
            Controls::EnergizeMode => controls.mode = ControlMode::Energize,
            Controls::DrainMode => {
                if selection.active {
                    // Fire area action event
                    area_action_events.send(AreaActionEvent {
                        area: selection.area,
                        action: AreaAction::Drain,
                    });
                } else {
                    controls.mode = ControlMode::Drain;
                }
            },
            Controls::DeleteMode => {
                if selection.active {
                    // Fire area action event
                    area_action_events.send(AreaActionEvent {
                        area: selection.area,
                        action: AreaAction::Delete,
                    });
                } else {
                    controls.mode = ControlMode::Delete;
                }
            }
            Controls::SelectMode => controls.mode = ControlMode::Select,
            Controls::MoveMode => controls.mode = ControlMode::Move,
            Controls::Unselect => {
                selection.selecting = false;
                selection.active = false;
                line_assist.drawing = false;
            }
            Controls::PauseKey => {
                if *app_state == AppState::Running {
                    commands.insert_resource(AppState::Paused);
                } else {
                    commands.insert_resource(AppState::Running);
                }
            }
            _ => (),
        }
    }
}

pub fn edit_wire(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    mouse_button: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    selection: Res<Selection>,
    line_assist: Res<LineAssist>,
    controls: Res<Controls>,
    cursor: Res<Cursor>,
    mut area_action_events: EventReader<AreaActionEvent>,
    mut tile_query: Query<(&CurrentState, &mut NextState)>,
    mut tilemap_query: Query<(Entity, &mut TileStorage)>,
) {
    if selection.selecting || line_assist.drawing {
        return;
    }

    let (tilemap_entity, mut tile_storage) = tilemap_query.single_mut();

    for event in area_action_events.iter() {
        turn_cells(
            event.area,
            &mut commands,
            tilemap_entity,
            &mut tile_storage,
            &mut tile_query,
            event.action.into(),
        )
    }

    if controls.mode.actionable() {
        if mouse_button.pressed(Controls::MousePrimary) {
            turn_cell(
                TilePos {
                    x: mouse_position.cell.x as u32,
                    y: mouse_position.cell.y as u32,
                },
                &mut commands,
                tilemap_entity,
                &mut tile_storage,
                &mut tile_query,
                controls.mode.into(),
            );
        } else if mouse_button.pressed(Controls::MouseSecondary) {
            turn_cell(
                TilePos {
                    x: mouse_position.cell.x as u32,
                    y: mouse_position.cell.y as u32,
                },
                &mut commands,
                tilemap_entity,
                &mut tile_storage,
                &mut tile_query,
                CellAction::Delete,
            );
        }
    }
}

fn turn_cells(
    area: Area,
    commands: &mut Commands,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
    tile_query: &mut Query<(&CurrentState, &mut NextState)>,
    action: CellAction,
) {
    let map_area = Area::new((0, 0), (SIZE, SIZE));
    let clamped = area.clamp(map_area);
    for x in clamped.a.x..clamped.b.x {
        for y in clamped.a.y..clamped.b.y {
            turn_cell(
                TilePos::new(x as u32, y as u32),
                commands,
                tilemap_entity,
                tile_storage,
                tile_query,
                action,
            );
        }
    }
}

fn turn_cell(
    tile_pos: TilePos,
    commands: &mut Commands,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
    tile_query: &mut Query<(&CurrentState, &mut NextState)>,
    action: CellAction,
) {
    if tile_pos.x >= SIZE as u32 || tile_pos.y >= SIZE as u32 {
        return;
    }

    let tile_entity = tile_storage.get(&tile_pos);
    if tile_entity == None {
        if action == CellAction::Place {
            spawn_cell(
                commands,
                tilemap_entity,
                tile_storage,
                tile_pos,
                CellState::Conductor,
            );
        }
    } else {
        let tile_entity = tile_entity.unwrap();
        let (current_state, mut next_state) = tile_query.get_mut(tile_entity).unwrap();

        if action == CellAction::Place && current_state.0 != CellState::Conductor {
            next_state.0 = CellState::Conductor;
        } else if action == CellAction::Energize && current_state.0 == CellState::Conductor {
            next_state.0 = CellState::Head;
        } else if action == CellAction::Delete {
            commands.entity(tile_entity).despawn();
            tile_storage.remove(&tile_pos);
        } else if action == CellAction::Drain
            && (current_state.0 == CellState::Head || current_state.0 == CellState::Tail)
        {
            next_state.0 = CellState::Conductor;
        }
    }
}

pub fn spawn_cell(
    commands: &mut Commands,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
    tile_pos: TilePos,
    state: CellState,
) {
    //info!("Spawning {:?} at {}:{}", state, tile_pos.x, tile_pos.y);

    let entity = commands
        .spawn()
        .insert_bundle(TileBundle {
            position: tile_pos.clone(),
            tilemap_id: TilemapId(tilemap_entity),
            ..default()
        })
        .insert(NextState(state))
        .insert(CurrentState(state))
        .id();
    tile_storage.set(&tile_pos, entity);
}
