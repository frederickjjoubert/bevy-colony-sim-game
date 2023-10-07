mod assets;
mod camera;
mod game;

use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use game::GamePlugin;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_hanabi::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    Loading,
    #[default]
    Gameplay,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        //.add_loading_state(
        //LoadingState::new(GameState::Loading).continue_to_state(GameState::Gameplay),
        //)
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
