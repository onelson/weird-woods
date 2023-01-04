use bevy::prelude::*;

#[derive(Component)]
struct GameCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(GameCamera);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .run();
}
