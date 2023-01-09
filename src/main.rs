use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use std::time::Duration;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Confirm,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct GameCamera;

#[derive(Default, Component)]
struct PlayerStart;

#[derive(Default, Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default()).insert(GameCamera);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/initial.ldtk"),
        ..default()
    });
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&GridCoords, Added<PlayerStart>>,
) {
    if let Ok(_coords) = query.get_single() {
        // FIXME: need a way to use the grid coordinates to position the player sprite
        // Possibly we can use `Assets<LdtkLevel>`? Look at the platform example.
        commands.spawn((
            Player,
            SpriteBundle {
                texture: asset_server.load("player.png"),
                transform: Transform::from_xyz(0., 0., 10.),
                ..default()
            },
            InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::Space, Action::Confirm),
                    (KeyCode::Return, Action::Confirm),
                    (KeyCode::Up, Action::Up),
                    (KeyCode::W, Action::Up),
                    (KeyCode::Down, Action::Down),
                    (KeyCode::S, Action::Down),
                    (KeyCode::Left, Action::Left),
                    (KeyCode::A, Action::Left),
                    (KeyCode::Right, Action::Right),
                    (KeyCode::D, Action::Right),
                ]),
            },
        ));
    }
}

/// Update player positions based on active input state.
fn player_movement(mut query: Query<(&mut Transform, &ActionState<Action>), With<Player>>) {
    // FIXME: fit the movement to be aligned with the tile grid.
    // FIXME: only apply new position if the terrain permits it.

    const MOVE_SPEED: f32 = 4.0;
    for (mut xform, action) in &mut query {
        if action.pressed(Action::Up) {
            xform.translation.y += MOVE_SPEED;
        }
        if action.pressed(Action::Down) {
            xform.translation.y -= MOVE_SPEED;
        }
        if action.pressed(Action::Left) {
            xform.translation.x -= MOVE_SPEED;
        }
        if action.pressed(Action::Right) {
            xform.translation.x += MOVE_SPEED;
        }
    }
}

fn main() {
    App::new()
        .add_fixed_timestep(Duration::from_millis(16), "my_fixed_update")
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .register_ldtk_entity::<PlayerStartBundle>("PlayerStart")
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .add_system(spawn_player)
        .add_system(player_movement)
        .run();
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerStartBundle {
    player_start: PlayerStart,
    #[grid_coords]
    grid_coords: GridCoords,
}
