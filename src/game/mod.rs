mod brain;
mod grid;
mod map;
mod starfield;
mod wall;

use bevy::render::camera::ScalingMode;
use starfield::spawn_star_field;

use crate::assets::sprites::Robots;
use crate::assets::AssetsPlugin;
use crate::camera::GameCameraPlugin;
use crate::GameState;
use bevy::prelude::*;

use self::brain::{Brain, BrainPlugin};
use self::wall::WallPlugin;

// === Plugin ===
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(AssetsPlugin)
            .add_plugins(BrainPlugin)
            .add_plugins(WallPlugin)
            .add_plugins(GameCameraPlugin)
            // Systems
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_star_field)
            .add_systems(PostStartup, start_game_state)
            .add_systems(OnEnter(GameState::Gameplay), spawn_test_pawn);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: 40.0,
                    min_height: 22.5,
                },
                ..default()
            },
            ..default()
        },
        Name::new("Camera"),
    ));
}

pub fn start_game_state(mut next_game_state: ResMut<NextState<GameState>>) {
    // next_game_state.set(GameState::Gameplay);
}

pub fn spawn_test_pawn(mut commands: Commands, sprites: Res<Robots>) {
    print!("spawn_test_pawn");
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(2.0, 2.0, 100.0),
            sprite: TextureAtlasSprite {
                index: 1,
                custom_size: Some(Vec2::splat(1.0)),
                ..default()
            },
            texture_atlas: sprites.frames.clone(),
            ..default()
        },
        Brain::new(),
    ));
}
