use bevy::asset::{Assets, Handle};
use bevy::prelude::{Commands, Entity, Query, Res, Resource, Transform};
use bevy::reflect::Reflect;
use bevy_ecs_ldtk::ldtk::{LayerInstance, Type};
use bevy_ecs_ldtk::utils::grid_coords_to_translation_relative_to_tile_layer;
use bevy_ecs_ldtk::{GridCoords, LdtkAsset, LdtkLevel};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Reflect, Default, Clone)]
pub struct GridSizing {
    pub cell_size_px: f32,
    pub grid_height: usize,
    pub grid_width: usize,
}

/// The number of cells that make up the level.
pub fn get_grid_size(level: &LdtkLevel) -> GridSizing {
    // Assumes all layers will have the same size.
    let LayerInstance {
        c_wid,
        c_hei,
        grid_size,
        ..
    } = &level.level.layer_instances.as_ref().expect("layer")[0];

    GridSizing {
        cell_size_px: *grid_size as f32,
        grid_height: *c_hei as usize,
        grid_width: *c_wid as usize,
    }
}

#[derive(Resource, Default)]
pub struct TileData {
    pub sizing: GridSizing,
    pub tile_ids: Vec<i32>,
    pub membership: HashMap<TileType, HashSet<i32>>,
}

// FIXME: make sure this runs only once on level change
pub fn setup_tileset_enums(
    mut commands: Commands,
    query: Query<(Entity, &Handle<LdtkAsset>)>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    // tile_types: Res<TileData>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    let level_handle = match level_query.get_single() {
        Ok((_, level_handle)) => level_handle,
        // Can't do anything if there's no level handle to work with
        _ => return,
    };
    let level = levels.get(level_handle).expect("level");
    let tile_ids = level
        .level
        .layer_instances
        .as_ref()
        .expect("layer")
        .iter()
        .find(|l| l.layer_instance_type == Type::Tiles)
        .expect("tile layer")
        .grid_tiles
        .iter()
        .map(|t| t.t)
        .collect();

    let (_, handle) = query.single();
    let membership = ldtk_assets
        .get(handle)
        .expect("asset")
        .project
        .defs
        .tilesets[0]
        .enum_tags
        .iter()
        .map(|x| {
            (
                x.enum_value_id
                    .parse::<TileType>()
                    .expect("invalid tile type"),
                x.tile_ids.iter().copied().collect(),
            )
        })
        .collect();
    let sizing = get_grid_size(level);
    commands.insert_resource(TileData {
        sizing,
        tile_ids,
        membership,
    });
}

pub fn grid_coords_to_transform(coords: GridCoords, sizing: GridSizing) -> Transform {
    let translate = grid_coords_to_translation_relative_to_tile_layer(
        coords,
        [sizing.cell_size_px as i32, sizing.cell_size_px as i32].into(),
    );
    Transform::from_xyz(translate.x, translate.y, 0.)
}

// FIXME: can this be simplified?
pub fn transform_to_tile_offset(xform: &Transform, sizing: &GridSizing) -> TileOffset {
    let x = (xform.translation.x.max(0.0) / sizing.cell_size_px).ceil();
    let x = (x as usize).min(sizing.grid_width);

    let y = (xform.translation.y.max(0.0) / sizing.cell_size_px).floor();
    // The grid cells start at the top-left so we need to compute the difference between y and the
    // total grid height.
    let y = (sizing.grid_height.saturating_sub(y as usize))
        .min(sizing.grid_height)
        .saturating_sub(1)
        * sizing.grid_width;

    x.saturating_sub(1) + y
}

#[derive(Debug, Reflect, Hash, Eq, PartialEq, Clone)]
pub enum TileType {
    Walkable,
    Swimmable,
    Ledge,
    Wall,
}

impl FromStr for TileType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Walkable" => Ok(Self::Walkable),
            "Swimmable" => Ok(Self::Swimmable),
            "Ledge" => Ok(Self::Ledge),
            "Wall" => Ok(Self::Wall),
            _ => Err(()),
        }
    }
}

pub type TileId = i32;
pub type TileOffset = usize;

#[cfg(test)]
mod tests {
    use super::{transform_to_tile_offset, GridSizing};
    use bevy::prelude::*;

    #[test]
    fn test_x_offset() {
        let sizing = GridSizing {
            cell_size_px: 16.,
            grid_height: 1,
            grid_width: 2,
        };

        assert_eq!(
            0,
            transform_to_tile_offset(&Transform::from_xyz(0., 0., 0.), &sizing),
        );
        assert_eq!(
            0,
            transform_to_tile_offset(&Transform::from_xyz(16.0, 0., 0.), &sizing),
        );
        // > 16 should register as the 1th cell, not the 0th.
        assert_eq!(
            1,
            transform_to_tile_offset(&Transform::from_xyz(16.1, 0., 0.), &sizing),
        );
        assert_eq!(
            1,
            transform_to_tile_offset(&Transform::from_xyz(32.0, 0., 0.), &sizing),
        );
        // > the bound of the grid should stay at whatever the max cell is.
        assert_eq!(
            1,
            transform_to_tile_offset(&Transform::from_xyz(40.0, 0., 0.), &sizing),
        );
    }
    #[test]
    fn test_y_offset() {
        let sizing = GridSizing {
            cell_size_px: 16.,
            // two rows this time means all the 0.0 y values place us in the 1th row instead of
            // the 0th.
            grid_height: 2,
            grid_width: 2,
        };

        assert_eq!(
            2,
            transform_to_tile_offset(&Transform::from_xyz(0., 0., 0.), &sizing),
        );
        assert_eq!(
            2,
            transform_to_tile_offset(&Transform::from_xyz(16.0, 0., 0.), &sizing),
        );
        // > 16 should register as the 1th cell, not the 0th.
        assert_eq!(
            3,
            transform_to_tile_offset(&Transform::from_xyz(16.1, 0., 0.), &sizing),
        );
        assert_eq!(
            3,
            transform_to_tile_offset(&Transform::from_xyz(32.0, 0., 0.), &sizing),
        );
        // > the bound of the grid should stay at whatever the max cell is.
        assert_eq!(
            3,
            transform_to_tile_offset(&Transform::from_xyz(40.0, 0., 0.), &sizing),
        );
    }
}
