use bevy::{prelude::*, sprite::Anchor, window::WindowId, winit::WinitWindows};
use bevy_ecs_tilemap::{
    prelude::{TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::TileStorage,
    TilemapBundle,
};

use crate::{
    components::colors::Colors,
    resources::{cursor::CursorMarker, gui::Gui},
    CELL_SIZE, SIZE,
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

    let tilemap_entity = commands.spawn().id();

    let tilemap_size = TilemapSize {
        x: SIZE as u32,
        y: SIZE as u32,
    };
    let tile_storage = TileStorage::empty(tilemap_size);
    let tile_size = TilemapTileSize { x: 4., y: 4. };
    let grid_size = tile_size.into();

    let texture_handle = asset_server.load("tile.png");

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            map_type: TilemapType::Square {
                diagonal_neighbors: true,
            },
            ..default()
        });

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
    //    commands.spawn_bundle(MaterialMesh2dBundle {
    //        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    //        transform: Transform {
    //            translation: Vec3 {
    //                x: (SIZE as f32 * CELL_SIZE - CELL_SIZE) / 2.,
    //                y: (SIZE as f32 * CELL_SIZE - CELL_SIZE) / 2.,
    //                z: 0.,
    //            },
    //            scale: Vec3::splat(SIZE as f32 * CELL_SIZE),
    //            ..default()
    //        },
    //        material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
    //        ..default()
    //    });

    let mut gui: Gui = Gui::default();

    // Spawn cursor
    gui.cursor = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(CELL_SIZE * 7.)),
                ..default()
            },
            texture: asset_server.load("cursor.png"),
            ..default()
        })
        .insert(CursorMarker)
        .id();

    // Spawn UI
    // Pause Icon Handle
    let pause_icon: Handle<Image> = asset_server.load("pause.png");

    let roboto_font_handle: Handle<Font> = asset_server.load("Roboto-Regular.ttf");

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
            gui.pause_icon = parent
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
                .insert(PauseIconMarker)
                .id();

            // Toolbar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(100.), Val::Auto),
                        position: UiRect {
                            bottom: Val::Px(0.),
                            left: Val::Px(0.),
                            ..default()
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|toolbar| {
                    let button_style = Style {
                        size: Size::new(Val::Px(64.), Val::Px(64.)),
                        margin: UiRect::all(Val::Px(8.)),
                        ..default()
                    };
                    // Power Icon
                    gui.place_icon = toolbar
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            image: asset_server.load("place.png").into(),
                            ..default()
                        })
                        .id();
                    // Power Icon
                    gui.power_icon = toolbar
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            image: asset_server.load("power.png").into(),
                            ..default()
                        })
                        .id();
                    // Move Icon
                    gui.select_icon = toolbar
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            image: asset_server.load("select.png").into(),
                            ..default()
                        })
                        .id();
                    // Select Icon
                    gui.move_icon = toolbar
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            image: asset_server.load("move.png").into(),
                            ..default()
                        })
                        .id();
                });
        });

    let font_handle: Handle<Font> = asset_server.load("font.ttf");

    // Spawn line assist overlay
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
        .insert(LineAssistOverlayMarker)
        .with_children(|parent| {
            parent
                .spawn()
                .insert_bundle(Text2dBundle::default())
                .insert(DragTextHorizontalMarker);

            parent
                .spawn()
                .insert_bundle(Text2dBundle::default())
                .insert(DragTextHorizontalMarker);

            parent
                .spawn()
                .insert_bundle(Text2dBundle::default())
                .insert(DragTextVerticalMarker);

            parent
                .spawn()
                .insert_bundle(Text2dBundle::default())
                .insert(DragTextVerticalMarker);
        });

    // Spawn area selection overlay
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
        .insert(SelectionOverlayMarker)
        .with_children(|parent| {
            parent
                .spawn()
                .insert_bundle(Text2dBundle::default())
                .insert(DragTextHorizontalMarker);

            parent
                .spawn()
                .insert_bundle(Text2dBundle::default())
                .insert(DragTextHorizontalMarker);

            parent
                .spawn()
                .insert_bundle(Text2dBundle::default())
                .insert(DragTextVerticalMarker);

            parent
                .spawn()
                .insert_bundle(Text2dBundle::default())
                .insert(DragTextVerticalMarker);
        });

    gui.highlighted_toolbar_icon = gui.move_icon;
    commands.insert_resource(gui);
}

#[derive(Component)]
pub struct DragTextHorizontalMarker;

#[derive(Component)]
pub struct DragTextVerticalMarker;

#[derive(Component)]
pub struct LineAssistOverlayMarker;

#[derive(Component)]
pub struct SelectionOverlayMarker;

#[derive(Component)]
pub struct PauseIconMarker;
