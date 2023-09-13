use crate::player::Player;
use crate::tilemap::TileData;
use crate::DebugData;
use bevy::prelude::{
    Camera, OrthographicProjection, Query, Res, ResMut, Transform, Vec2, With, Without,
};

const ZOOM: f32 = 0.25;

pub fn zoom_in(mut query: Query<&mut OrthographicProjection, With<Camera>>) {
    for mut projection in query.iter_mut() {
        projection.scale = ZOOM;
    }
}

pub fn follow_player(
    mut cam_query: Query<(&mut Transform, &Camera), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    tile_data: Res<TileData>,
    mut monitor: ResMut<DebugData>,
) {
    let sizing = &tile_data.sizing;
    let world_width = sizing.grid_width as f32 * sizing.cell_size_px;
    let world_height = sizing.grid_height as f32 * sizing.cell_size_px;
    if let (Ok((mut cam_xform, cam)), Ok(player)) =
        (cam_query.get_single_mut(), player_query.get_single())
    {
        let viewport = cam.logical_viewport_size().expect("cam viewport");
        let viewport_size = Vec2::new(viewport.x * ZOOM, viewport.y * ZOOM);

        let min_x = viewport_size.x / 2.0;
        let min_y = viewport_size.y / 2.0;
        let max_x = world_width - viewport_size.x / 2.0;
        let max_y = world_height - viewport_size.y / 2.0;

        // TODO: if the player is moving within a some portion of the middle of the viewport,
        //   return early.
        //   Having a "dead zone" in the center of the screen might feel better.

        let next_cam_pos = player.translation.truncate();
        // leave the z where it is, only mess with the x,y.
        cam_xform.translation.x = next_cam_pos.x.max(min_x).min(max_x);
        cam_xform.translation.y = next_cam_pos.y.max(min_y).min(max_y);

        monitor.player_trans = player.translation.truncate();
        monitor.camera_trans = cam_xform.translation.truncate();
        monitor.grid_sizing = sizing.clone();
        monitor.world_size = Vec2::new(world_width, world_height);
        monitor.viewport_size = viewport_size;
    }
}
