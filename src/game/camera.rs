use crate::game::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::game::utils::grid_to_world;
use bevy::prelude::*;

use crate::GameState;

const CAMERA_DEFAULT_ZOOM: f32 = 0.75;
const CAMERA_MOVEMENT_SPEED: f32 = 200.0;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            (camera_movement_system, reset_camera_position).run_if(in_state(GameState::Gameplay)),
        );
    }
}

pub fn spawn_camera(mut commands: Commands) {
    info!("spawn_camera");
    let (world_x, world_y) = grid_to_world((MAP_WIDTH / 2) as i32, (MAP_HEIGHT / 2) as i32);
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(world_x, world_y, 1000.0),
            projection: OrthographicProjection {
                scale: CAMERA_DEFAULT_ZOOM,
                ..default()
            },
            ..default()
        },
        Name::new("GameCamera"),
    ));
}

// TODO leafwing input
fn camera_movement_system(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut camera_transform = camera.single_mut();
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
        camera_transform.translation.y += time.delta_seconds() * CAMERA_MOVEMENT_SPEED;
    }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
        camera_transform.translation.y -= time.delta_seconds() * CAMERA_MOVEMENT_SPEED;
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        camera_transform.translation.x += time.delta_seconds() * CAMERA_MOVEMENT_SPEED;
    }
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        camera_transform.translation.x -= time.delta_seconds() * CAMERA_MOVEMENT_SPEED;
    }
}

pub fn reset_camera_position(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if keyboard.just_pressed(KeyCode::C) {
        if let Ok(mut transform) = camera_query.get_single_mut() {
            let (world_x, world_y) = grid_to_world((MAP_WIDTH / 2) as i32, (MAP_HEIGHT / 2) as i32);
            transform.translation.x = world_x;
            transform.translation.y = world_y;
        }
    }
}
