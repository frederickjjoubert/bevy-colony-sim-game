pub mod sprites;

use bevy::prelude::{App, Plugin};
use sprites::SpritesPlugin;

// === Plugin ===
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SpritesPlugin);
        // TODO: Add Sound Effects Plugin
        // TODO: ADD Music Plugin
    }
}

