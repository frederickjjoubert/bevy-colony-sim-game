use bevy::prelude::{Reflect, States};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum GameState {
    Loading,
    #[default]
    Gameplay,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
