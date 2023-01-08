use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
struct GameCamera;

#[derive(Default, Component)]
struct PlayerStart;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default()).insert(GameCamera);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/initial.ldtk"),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .run();
}

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    player_start: PlayerStart,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
