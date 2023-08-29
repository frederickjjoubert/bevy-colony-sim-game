mod brain;
mod grid;
mod starfield;
mod wall;
mod map;

use starfield::spawn_star_field;

use bevy::prelude::*;
use crate::assets::AssetsPlugin;
use crate::assets::sprites::resources::{Sprites, SpriteSheets};

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
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            projection: OrthographicProjection::default(),
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
