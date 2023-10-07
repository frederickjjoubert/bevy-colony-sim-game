use std::path::Path;

pub use bevy::prelude::*;

use crate::{assets::sprites::Aske4TileSet, GameState};

use super::grid::{GridLocation, GridPlugin, GRID_SIZE};

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
    for i in 0..GRID_SIZE {
        spawn_test_wall(&mut commands, i, 0, &sprites);
        spawn_test_wall(&mut commands, i, GRID_SIZE - 1, &sprites);
    }
    for j in 1..GRID_SIZE - 1 {
        spawn_test_wall(&mut commands, 0, j, &sprites);
        spawn_test_wall(&mut commands, GRID_SIZE - 1, j, &sprites);
    }
}

fn spawn_test_wall(commands: &mut Commands, x: usize, y: usize, sprites: &Res<Aske4TileSet>) {
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(x as f32, y as f32, 10.0),
            sprite: TextureAtlasSprite {
                index: 11,
                custom_size: Some(Vec2::splat(1.0)),
                ..default()
            },
            texture_atlas: sprites.tiles.clone(),
            ..default()
        },
        Wall,
        PathfindingBlocker,
        GridLocation::new(x as u32, y as u32),
    ));
}
