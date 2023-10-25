use crate::game::constants::TILE_SIZE;

pub fn grid_to_world(x: i32, y: i32) -> (f32, f32) {
    let world_x = x as f32 * TILE_SIZE as f32;
    let world_y = y as f32 * TILE_SIZE as f32;
    (world_x, world_y)
}

// Be sure to check the output is within bounds.
pub fn world_to_grid(world_x: f32, world_y: f32) -> (i32, i32) {
    let x = (world_x / TILE_SIZE as f32).floor() as i32;
    let y = (world_y / TILE_SIZE as f32).floor() as i32;
    (x, y)
}
