use bevy::asset::{AssetServer, Assets, Handle};
use bevy::prelude::{
    default, Added, Bundle, Commands, Component, Entity, Query, Res, SpriteBundle,
};
use bevy::reflect::Reflect;
use bevy_ecs_ldtk::prelude::LdtkFields;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LdtkEntity, LdtkLevel};
use std::str::FromStr;

#[derive(Default, Component, Reflect)]
pub struct Key;

#[derive(Component, Default, Debug, Reflect, Eq, Hash, PartialEq, Copy, Clone)]
pub enum KeyType {
    Yellow,
    #[default]
    Invalid,
}

impl FromStr for KeyType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Yellow" => Ok(KeyType::Yellow),
            _ => Err(()),
        }
    }
}

impl KeyType {
    pub fn from_field(entity_instance: &EntityInstance) -> KeyType {
        let s = entity_instance
            .get_enum_field("KeyType")
            .expect("key type field");

        s.parse().expect("parse key type")
    }
}

pub fn spawn_keys(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
    // Data loaded from LDtK to be replaced by new entities with sprites.
    query: Query<(Entity, &GridCoords, &KeyType), Added<Key>>,
) {
    for (data_entity, &coords, key_type) in query.iter() {
        let (_, level_handle) = level_query.single();
        let level = levels.get(level_handle).expect("level");
        let sizing = crate::tilemap::get_grid_size(level);

        commands.entity(data_entity).despawn();

        let mut transform = crate::tilemap::grid_coords_to_transform(coords, sizing);
        // FIMXE: still needed?
        transform.translation.z = 10.;

        // FIXME: needs to incude the type of key
        commands.spawn((
            Key,
            *key_type,
            SpriteBundle {
                texture: asset_server.load("key.png"),
                transform,
                ..default()
            },
        ));
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct KeyBundle {
    key: Key,
    #[with(KeyType::from_field)]
    key_type: KeyType,
    #[grid_coords]
    grid_coords: GridCoords,
}
