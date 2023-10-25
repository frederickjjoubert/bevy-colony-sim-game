pub mod components;
mod fonts;
mod interactions;
mod layout;

use crate::game::ui::interactions::wall_button_interactions;
use crate::game::ui::layout::{despawn_game_ui, spawn_game_ui};
use crate::state::AppState;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_game_ui)
            .add_systems(
                Update,
                wall_button_interactions.run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_game_ui);
    }
}
