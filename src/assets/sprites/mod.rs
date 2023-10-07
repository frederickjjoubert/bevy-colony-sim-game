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
        //app.init_collection::<Robots>()
        //.init_collection::<Aske4TileSet>();
        app.add_systems(Startup, load_assets);
    }
}

pub fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let tiles = assets.load("sprites/aske4/TileSet v1.0.png");
    let frames = assets.load("sprites/belillart/The-Loop-all-assets/Robot1-all-sprites.png");
    let tiles = texture_atlas.add(TextureAtlas::from_grid(
        tiles,
        Vec2::splat(32.0),
        10,
        10,
        None,
        None,
    ));
    let frames = texture_atlas.add(TextureAtlas::from_grid(
        frames,
        Vec2::splat(16.0),
        7,
        4,
        None,
        None,
    ));

    commands.insert_resource(Aske4TileSet { tiles });
    commands.insert_resource(Robots { frames });
}
