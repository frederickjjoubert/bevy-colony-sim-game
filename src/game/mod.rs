mod brain;
mod camera;
mod constants;
mod grid;
mod starfield;
pub mod state;
pub mod ui;
mod utils;
pub mod wall;

use crate::assets::sprites::Robots;
use crate::assets::AssetsPlugin;
use crate::game::camera::GameCameraPlugin;
use crate::game::state::GameState;
use crate::game::ui::UIPlugin;
use crate::game::utils::grid_to_world;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use brain::{Brain, BrainPlugin};
use starfield::spawn_star_field;
use wall::WallPlugin;

// === Plugin ===
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // State
            .add_state::<GameState>()
            //.add_loading_state(
            //LoadingState::new(GameState::Loading).continue_to_state(GameState::Gameplay),
            //)
            // Plugins
            .add_plugins(AssetsPlugin)
            .add_plugins(BrainPlugin)
            .add_plugins(WallPlugin)
            .add_plugins(GameCameraPlugin)
            .add_plugins(UIPlugin)
            // Systems
            .add_systems(Startup, spawn_star_field)
            .add_systems(OnEnter(GameState::Gameplay), spawn_test_pawn);
    }
}

pub fn spawn_test_pawn(mut commands: Commands, sprites: Res<Robots>) {
    info!("spawn_test_pawn");
    let (world_x, world_y) = grid_to_world((2 / 2) as i32, (2 / 2) as i32);
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(world_x, world_y, 1.0),
            sprite: TextureAtlasSprite {
                anchor: Anchor::BottomLeft,
                custom_size: Some(Vec2::splat(32.0)),
                index: 1,
                ..default()
            },
            texture_atlas: sprites.frames.clone(),
            ..default()
        },
        Brain::new(),
    ));
}
