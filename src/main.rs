mod assets;
mod colors;
mod game;
mod state;

use crate::game::state::GameState;
use crate::game::ui::components::UI;
use crate::game::wall::Wall;
use crate::state::AppState;
use bevy::prelude::*;
use bevy::window::{close_on_esc, PresentMode, WindowResolution};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;

fn main() {
    App::new()
        // Window Background Color
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                // Prevents blurry sprites
                .set(ImagePlugin::default_nearest())
                // Set up the window
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        title: "Space Colony Sim Game".to_string(),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        ..default()
                    }),
                    ..default()
                })
                .build()
                // bevy_embedded_assets
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugins(HanabiPlugin)
        .add_plugins(GamePlugin)
        // Bevy Inspector egui
        .add_plugins(FilterQueryInspectorPlugin::<(Without<UI>, Without<Wall>)>::default())
        .add_plugins(FilterQueryInspectorPlugin::<With<UI>>::default())
        // .add_plugins(ResourceInspectorPlugin::<Score>::default())
        .add_plugins(StateInspectorPlugin::<AppState>::default())
        .add_plugins(StateInspectorPlugin::<GameState>::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, close_on_esc)
        .run();
}
