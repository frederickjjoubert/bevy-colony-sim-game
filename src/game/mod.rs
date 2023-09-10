mod brain;
mod grid;
mod map;
mod starfield;
mod wall;

use bevy::render::camera::ScalingMode;
use starfield::spawn_star_field;

use crate::assets::sprites::resources::{SpriteSheets, Sprites};
use crate::assets::AssetsPlugin;
use bevy::prelude::*;

use self::brain::BrainPlugin;
use self::wall::WallPlugin;

// === Plugin ===
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            //Plugins
            .add_plugins(AssetsPlugin)
            .add_plugins(BrainPlugin)
            .add_plugins(WallPlugin)
            // Systems
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_star_field)
            .add_systems(Startup, test_sprites_loaded);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: 80.0,
                    min_height: 45.0,
                },
                ..default()
            },
            ..default()
        },
        Name::new("Camera"),
    ));
}

pub fn test_sprites_loaded(sprites: Res<Sprites>, sprite_sheets: Res<SpriteSheets>) {
    println!("test_sprites_loaded!");
    println!("{:?}", sprites.sprites);
    println!("{:?}", sprite_sheets.sprite_sheets);
}
