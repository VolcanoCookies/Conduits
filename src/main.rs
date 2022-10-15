pub mod components;
mod resources;
pub mod systems;
pub mod utils;

use std::time::Duration;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::close_on_esc;
use components::cell_map::CellMap;
use components::cursor::{Cursor, DragEvent};
use components::state_map::StateMap;
use iyes_loopless::prelude::*;
use systems::camera::{camera_movement, CameraZoom, MousePosition};
use systems::input::{edit_wire, handle_input};
use systems::setup::setup;
use systems::state::{do_state, update_state};
use systems::ui::{icon_toggles, update_cursor};

pub const SIZE: i32 = 1024;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AppState {
    Running,
    Paused,
}

pub fn paused(app_state: Res<AppState>) -> bool {
    *app_state == AppState::Paused
}

fn main() {
    let mut update_stage = SystemStage::parallel();
    update_stage.add_system(update_state.label("update_state"));
    update_stage.add_system(do_state.run_if_not(paused).after("update_state"));

    App::new()
        .add_event::<DragEvent>()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .insert_resource(AppState::Running)
        .insert_resource(CellMap::new(SIZE, SIZE))
        .insert_resource(StateMap::new(SIZE, SIZE))
        .insert_resource(MousePosition::default())
        .insert_resource(CameraZoom(1.))
        .insert_resource(Cursor::default())
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_system(camera_movement)
                .with_system(handle_input.after(camera_movement).label("handle_input"))
                .with_system(update_cursor.after(camera_movement))
                .with_system(icon_toggles),
        )
        .add_stage(
            "fixed_update",
            FixedTimestepStage::new(Duration::from_millis(125)).with_stage(update_stage),
        )
        .add_stage_after(
            "fixed_update",
            "edit_wire",
            SystemStage::parallel().with_system(edit_wire),
        )
        .add_system(close_on_esc)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

pub fn unknown_entity() -> Entity {
    Entity::from_raw(0)
}
