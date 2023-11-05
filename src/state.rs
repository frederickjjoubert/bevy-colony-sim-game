use bevy::prelude::{Reflect, States};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum AppState {
    Loading,
    MainMenu,
    #[default]
    Game,
}
