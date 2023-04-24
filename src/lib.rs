// [[file:../bevy.note::*src/lib.rs][src/lib.rs:1]]
// #![deny(warnings)]
// #![deny(clippy::all)]
// src/lib.rs:1 ends here

// [[file:../bevy.note::3edfd207][3edfd207]]
// mod actions;
mod camera;
// mod loading;
// mod menu;
mod molecule;
mod player;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;
// 3edfd207 ends here

// [[file:../bevy.note::043e6795][043e6795]]
// use crate::actions::ActionsPlugin;
// use crate::loading::LoadingPlugin;
// use crate::menu::MenuPlugin;

use bevy::app::App;
use bevy::prelude::*;
// 043e6795 ends here
