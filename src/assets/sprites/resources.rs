use std::collections::HashMap;
use bevy::prelude::Resource;
use serde::{Serialize, Deserialize};

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct Sprites {
    pub sprites: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSheet {
    pub path: String,
    pub sprite_size: SpriteSize,
    pub sprites: HashMap<String, SpritePosition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpritePosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct SpriteSheets {
    pub sprite_sheets: HashMap<String, SpriteSheet>,
}