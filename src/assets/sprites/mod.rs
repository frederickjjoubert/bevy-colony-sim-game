use bevy::prelude::*;
use bevy_asset_loader::asset_collection::{AssetCollection, AssetCollectionApp};
use bevy_asset_loader::loading_state::LoadingStateAppExt;

use crate::GameState;

// === Plugin ===
pub struct SpritesPlugin;

#[derive(AssetCollection, Resource)]
pub struct Aske4TileSet {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 10, rows = 10))]
    #[asset(path = "sprites/aske4/TileSet v1.0.png")]
    pub tiles: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct Robots {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 7, rows = 4))]
    #[asset(path = "sprites/belillart/The-Loop-all-assets/Robot1-all-sprites.png")]
    pub frames: Handle<TextureAtlas>,
}

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<Robots>()
            .init_collection::<Aske4TileSet>();
    }
}
