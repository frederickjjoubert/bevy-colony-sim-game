use bevy::prelude::*;

use crate::GameState;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            camera_movement_system.run_if(in_state(GameState::Gameplay)),
        );
    }
}

// TODO leafwing input
fn camera_movement_system(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut camera_transform = camera.single_mut();
    if keyboard.pressed(KeyCode::W) {
        camera_transform.translation.y += time.delta_seconds() * 20.0;
    }
    if keyboard.pressed(KeyCode::S) {
        camera_transform.translation.y -= time.delta_seconds() * 20.0;
    }
    if keyboard.pressed(KeyCode::D) {
        camera_transform.translation.x += time.delta_seconds() * 20.0;
    }
    if keyboard.pressed(KeyCode::A) {
        camera_transform.translation.x -= time.delta_seconds() * 20.0;
    }
}
