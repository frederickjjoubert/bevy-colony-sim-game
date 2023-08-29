mod camera;
mod game;
mod assets;

use game::GamePlugin;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_hanabi::prelude::*;



fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                // Set up the window
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        title: "Space Colony Sim Game".to_string(),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(HanabiPlugin)
        .add_plugins(GamePlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .run();
}
