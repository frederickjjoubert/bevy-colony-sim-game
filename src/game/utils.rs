use crate::game::constants::TILE_SIZE;

pub fn grid_to_world(x: i32, y: i32) -> (f32, f32) {
    let world_x = x as f32 * TILE_SIZE as f32;
    let world_y = y as f32 * TILE_SIZE as f32;
    (world_x, world_y)
}
