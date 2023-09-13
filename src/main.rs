use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct DebugData {
    grid_sizing: GridSizing,
    player_trans: Vec2,
    camera_trans: Vec2,
    world_size: Vec2,
    viewport_size: Vec2,
}

use crate::tilemap::{GridSizing, TileData};
use bevy::window::{PresentMode, WindowTheme};
use leafwing_input_manager::prelude::*;
use player::{Action, PlayerStartBundle};
use std::time::Duration;

mod camera;
mod player;
mod tilemap;

#[derive(Component)]
struct GameCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default()).insert(GameCamera);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/initial.ldtk"),
        ..default()
    });
    commands.insert_resource(TileData::default());
}

fn main() {
    let window_cfg = Window {
        title: "Weird Woods".into(),
        resolution: (1024., 768.).into(),
        present_mode: PresentMode::AutoVsync,
        fit_canvas_to_parent: true,
        prevent_default_event_handling: false,
        window_theme: Some(WindowTheme::Dark),
        ..default()
    };

    App::new()
        .insert_resource(FixedTime::new(Duration::from_millis(16)))
        .init_resource::<DebugData>()
        .register_type::<DebugData>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(window_cfg),
                ..default()
            }),
            // bevy::diagnostic::LogDiagnosticsPlugin::default(),
            // bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            LdtkPlugin,
            InputManagerPlugin::<Action>::default(),
            ResourceInspectorPlugin::<DebugData>::default(),
        ))
        .register_ldtk_entity::<PlayerStartBundle>("PlayerStart")
        .add_systems(Startup, (setup,))
        .insert_resource(LevelSelection::Index(0))
        .add_systems(
            Update,
            (
                camera::zoom_in,
                player::player_movement,
                bevy::window::close_on_esc,
                tilemap::setup_tileset_enums,
                player::spawn_player,
                camera::follow_player,
            ),
        )
        .run();
}
