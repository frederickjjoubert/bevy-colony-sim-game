pub mod resources;

use bevy::app::{PreStartup, Startup};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use resources::{SpriteSheets, Sprites};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::GameState;

// === Plugin ===
pub struct SpritesPlugin;

#[derive(AssetCollection, Resource)]
pub struct Aske4TileSet {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 10, rows = 10))]
    #[asset(path = "sprites/aske4/TileSet v1.0.png")]
    pub tiles: Handle<TextureAtlas>,
}

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (create_sprites_resource, create_sprite_sheets_resource),
        )
        .add_collection_to_loading_state::<_, Aske4TileSet>(GameState::Loading);
    }
}

pub fn create_sprites_resource(mut commands: Commands) {
    // Read JSON data from a file
    let path = Path::new("assets/sprites/").join("sprites.json");
    let mut file = File::open(path).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read file");

    let sprites: Sprites = serde_json::from_str(&json_data).unwrap();

    commands.insert_resource(sprites);
}

pub fn create_sprite_sheets_resource(mut commands: Commands) {
    // Read JSON data from a file
    let path = Path::new("assets/sprites/").join("sprite_sheets.json");
    let mut file = File::open(path).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read file");

    let sprite_sheets: SpriteSheets = serde_json::from_str(&json_data).unwrap();

    commands.insert_resource(sprite_sheets);
}
