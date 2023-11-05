use super::grid::{GridLocation, GridPlugin, GRID_SIZE};
use crate::assets::sprites::Aske4TileSet;
use crate::game::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::game::state::GameState;
use crate::game::utils::grid_to_world;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component, Default)]
pub struct Wall;

#[derive(Component, Default)]
pub struct PathfindingBlocker;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GridPlugin::<Wall>::default())
            .add_plugins(GridPlugin::<PathfindingBlocker>::default())
            .add_systems(OnEnter(GameState::Gameplay), spawn_test_walls);
    }
}

fn spawn_test_walls(mut commands: Commands, sprites: Res<Aske4TileSet>) {
    for i in 0..MAP_WIDTH as usize {
        spawn_test_wall(&mut commands, i, 0, &sprites);
        spawn_test_wall(&mut commands, i, MAP_HEIGHT as usize - 1, &sprites);
    }
    for j in 1..MAP_HEIGHT as usize - 1 {
        spawn_test_wall(&mut commands, 0, j, &sprites);
        spawn_test_wall(&mut commands, MAP_WIDTH as usize - 1, j, &sprites);
    }
}

fn spawn_test_wall(commands: &mut Commands, x: usize, y: usize, sprites: &Res<Aske4TileSet>) {
    let (world_x, world_y) = grid_to_world(x as i32, y as i32);
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(world_x, world_y, 1.0),
            sprite: TextureAtlasSprite {
                anchor: Anchor::BottomLeft,
                index: 11,
                ..default()
            },
            texture_atlas: sprites.tiles.clone(),
            ..default()
        },
        Wall,
        PathfindingBlocker,
        GridLocation::new(x as u32, y as u32),
        Name::new(format!("Wall_Tile_{}_{}", x, y)),
    ));
}
