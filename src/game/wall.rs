pub use bevy::prelude::*;

use super::grid::{GridLocation, GridPlugin, GRID_SIZE};

#[derive(Component, Default)]
pub struct Wall;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GridPlugin::<Wall>::default())
            .add_systems(Startup, spawn_test_walls);
    }
}

fn spawn_test_walls(mut commands: Commands) {
    for i in 0..GRID_SIZE {
        spawn_test_wall(&mut commands, i, 0);
        spawn_test_wall(&mut commands, i, GRID_SIZE - 1);
    }
    for j in 1..GRID_SIZE - 1 {
        spawn_test_wall(&mut commands, 0, j);
        spawn_test_wall(&mut commands, GRID_SIZE - 1, j);
    }
}

fn spawn_test_wall(commands: &mut Commands, x: usize, y: usize) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x as f32, y as f32, 0.0),
            ..default()
        },
        Wall,
        GridLocation::new(x as u32, y as u32),
    ));
}
