use crate::key::{Key, KeyType};
use crate::tilemap::{transform_to_tile_offset, TileData, TileType};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::prelude::{
    default, Added, Bundle, Commands, Component, Entity, KeyCode, Query, Reflect, Res, ResMut,
    SpriteBundle, Transform, With, Without,
};
use bevy_ecs_ldtk::{GridCoords, LdtkEntity, LdtkLevel};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use std::collections::HashSet;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Confirm,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Component)]
pub struct PlayerStart;

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component, Debug)]
pub struct Inventory {
    keys: HashSet<KeyType>,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
    query: Query<&GridCoords, Added<PlayerStart>>,
) {
    if let Ok(&coords) = query.get_single() {
        let (_, level_handle) = level_query.single();
        let level = levels.get(level_handle).expect("level");
        let sizing = crate::tilemap::get_grid_size(level);

        let mut transform = crate::tilemap::grid_coords_to_transform(coords, sizing);
        transform.translation.z = 10.;

        commands.spawn((
            Player,
            Inventory::default(),
            SpriteBundle {
                texture: asset_server.load("player.png"),
                transform,
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
pub fn player_movement(
    mut query: Query<(&mut Transform, &ActionState<Action>), With<Player>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
    tile_types: Res<TileData>,
) {
    // if we don't know which tiles are walkable, we can't move
    if tile_types.membership.is_empty() {
        return;
    }

    for (mut initial_xform, action) in &mut query {
        let (_, level_handle) = level_query.single();
        let level = levels.get(level_handle).expect("level");
        let size = crate::tilemap::get_grid_size(level);

        let tile_offset = transform_to_tile_offset(&initial_xform, &size);
        let tile_id = &tile_types.tile_ids[tile_offset];

        let current_tile: Vec<_> = tile_types
            .membership
            .iter()
            .filter_map(|(k, v)| if v.contains(tile_id) { Some(k) } else { None })
            .cloned()
            .collect();

        let move_speed = if current_tile.contains(&TileType::Swimmable) {
            0.75
        } else {
            2.5
        };

        let mut proposed_xform = *initial_xform;

        if action.pressed(Action::Up) {
            proposed_xform.translation.y += move_speed;
        }
        if action.pressed(Action::Down) {
            proposed_xform.translation.y -= move_speed;
        }
        if action.pressed(Action::Left) {
            proposed_xform.translation.x -= move_speed;
        }
        if action.pressed(Action::Right) {
            proposed_xform.translation.x += move_speed;
        }

        // Using the updated translation, see if the move is permitted.

        let tile_offset = transform_to_tile_offset(&proposed_xform, &size);
        let tile_id = &tile_types.tile_ids[tile_offset];
        let proposed_tile: Vec<_> = tile_types
            .membership
            .iter()
            .filter_map(|(k, v)| if v.contains(tile_id) { Some(k) } else { None })
            .cloned()
            .collect();

        if proposed_tile
            .iter()
            .any(|tt| matches!(tt, TileType::Swimmable | TileType::Walkable))
        {
            initial_xform.translation = proposed_xform.translation;
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerStartBundle {
    player_start: PlayerStart,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[allow(clippy::type_complexity)]
pub fn collect_key(
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    query2: Query<(Entity, &Transform, &KeyType), (With<Key>, Without<Player>)>,
    mut query3: Query<&mut Inventory>,
    mut monitor: ResMut<crate::DebugData>,
) {
    for player_xform in query.iter() {
        for (entity, key_xform, key_type) in query2.iter() {
            if player_xform.translation.distance(key_xform.translation) < 16.0 {
                let mut inventory = query3.single_mut();
                inventory.keys.insert(*key_type);
                monitor.has_yellow_key = matches!(key_type, KeyType::Yellow);
                commands.entity(entity).despawn();
            }
        }
    }
}
