// [[file:../bevy.note::*src/lib.rs][src/lib.rs:1]]
// #![deny(warnings)]
// #![deny(clippy::all)]
// src/lib.rs:1 ends here

// [[file:../bevy.note::3edfd207][3edfd207]]
mod actions;
// mod camera;
mod loading;
mod menu;
mod molecule;
mod player;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;
// 3edfd207 ends here

// [[file:../bevy.note::043e6795][043e6795]]
use crate::actions::ActionsPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
// 043e6795 ends here

// [[file:../bevy.note::398b4a02][398b4a02]]
// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            // .add_plugin(LoadingPlugin)
            // .add_plugin(MenuPlugin)
            // .add_plugin(ActionsPlugin)
            // .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
// 398b4a02 ends here
