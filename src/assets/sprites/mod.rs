pub mod resources;

use bevy::app::{PreStartup, Startup};
use bevy::prelude::{App, Commands, Plugin, Resource};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use resources::{Sprites, SpriteSheets};

// === Plugin ===
pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, (create_sprites_resource, create_sprite_sheets_resource));
    }
}


pub fn create_sprites_resource(mut commands: Commands) {
    // Read JSON data from a file
    let path = Path::new("assets/sprites/").join("sprites.json");
    let mut file = File::open(path).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Failed to read file");

    let sprites: Sprites = serde_json::from_str(&json_data).unwrap();

    commands.insert_resource(sprites);
}

pub fn create_sprite_sheets_resource(mut commands: Commands) {
    // Read JSON data from a file
    let path = Path::new("assets/sprites/").join("sprite_sheets.json");
    let mut file = File::open(path).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Failed to read file");

    let sprite_sheets: SpriteSheets = serde_json::from_str(&json_data).unwrap();

    commands.insert_resource(sprite_sheets);
}
