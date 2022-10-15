use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Anchor}, window::WindowId, winit::WinitWindows};

use crate::{
    components::{
        cell::CELL_SIZE,
        colors::Colors,
        cursor::{Cursor, CursorMarker},
    },
    SIZE,
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: NonSend<WinitWindows>,
) {
    // Set window title
    let primary = windows.get_window(WindowId::primary()).unwrap();
    primary.set_title("Electric Rust");

    // Spawn main camera
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform {
            translation: Vec3 {
                x: (SIZE as f32 * CELL_SIZE - CELL_SIZE) / 2.,
                y: (SIZE as f32 * CELL_SIZE - CELL_SIZE) / 2.,
                z: 999.9,
            },
            ..default()
        },
        ..default()
    });

    // Create background mesh
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            translation: Vec3 {
                x: (SIZE as f32 * CELL_SIZE - CELL_SIZE) / 2.,
                y: (SIZE as f32 * CELL_SIZE - CELL_SIZE) / 2.,
                z: 0.,
            },
            scale: Vec3::splat(SIZE as f32 * CELL_SIZE),
            ..default()
        },
        material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
        ..default()
    });

    // Spawn cursor
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(CELL_SIZE)),
                ..default()
            },
            texture: asset_server.load("cursor.png"),
            ..default()
        })
        .insert(CursorMarker);

    // Spawn UI
    // Pause Icon Handle
    let pause_icon: Handle<Image> = asset_server.load("pause.png");

    // Root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // Pause Icon
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            right: Val::Px(8.0),
                            top: Val::Px(8.0),
                            ..default()
                        },
                        ..default()
                    },
                    image: pause_icon.into(),
                    ..default()
                })
                .insert(PauseIconMarker);
        });

    // Spawn drag overlay entity
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Colors::DragOverlayPlace,
                anchor: Anchor::BottomLeft,
                ..default()
            },
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(DragOverlayMarker);
//
//    for i in 0..3 {
//        commands.spawn()
//        .inset_bundle(Text2dBundle)
//    }
}

#[derive(Component)]
pub struct DragOverlayMarker;

#[derive(Component)]
pub struct PauseIconMarker;
