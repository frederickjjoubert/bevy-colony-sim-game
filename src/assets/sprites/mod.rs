pub mod resources;

use bevy::app::{PreStartup, Startup};
use bevy::prelude::{App, AssetServer, Assets, Commands, Plugin, Res, ResMut, Resource, Vec2};
use bevy::sprite::TextureAtlas;
use resources::{SpriteSheets, Sprites};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

// === Plugin ===
pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (create_sprites_resource, create_sprite_sheets_resource),
        )
        .add_systems(
            Startup,
            (test_sprites_loaded, create_character_sprite_sheet),
        );
    }
}

fn create_sprites_resource(mut commands: Commands) {
    // Read JSON data from a file
    let path = Path::new("assets/sprites/").join("sprites.json");
    let mut file = File::open(path).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read file");

    let sprites: Sprites = serde_json::from_str(&json_data).unwrap();

    commands.insert_resource(sprites);
}

fn create_sprite_sheets_resource(mut commands: Commands) {
    // Read JSON data from a file
    let path = Path::new("assets/sprites/").join("sprite_sheets.json");
    let mut file = File::open(path).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read file");

    let sprite_sheets: SpriteSheets = serde_json::from_str(&json_data).unwrap();

    commands.insert_resource(sprite_sheets);
}

fn test_sprites_loaded(sprites: Res<Sprites>, sprite_sheets: Res<SpriteSheets>) {
    println!("test_sprites_loaded!");
    println!("{:?}", sprites.sprites);
    println!("{:?}", sprite_sheets.sprite_sheets);
}

fn create_character_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_sheets: Res<SpriteSheets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_sheet = sprite_sheets.sprite_sheets.get("character").unwrap();
    let texture_handle = asset_server.load(&sprite_sheet.path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(
            sprite_sheet.sprite_size.width as f32,
            sprite_sheet.sprite_size.height as f32,
        ),
        1,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
}
