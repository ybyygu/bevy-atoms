// [[file:../bevy.note::6c039888][6c039888]]
// #![deny(warnings)]
#![deny(clippy::all)]

use crate::net::ServerPlugin;

use bevy::prelude::*;
use gut::cli::*;
// 6c039888 ends here

// [[file:../bevy.note::d66e839e][d66e839e]]
pub struct RemoteConsolePlugin;

/// Command that can be evoked from the remote client side
#[derive(Clone, Debug)]
pub enum RemoteCommand {
    /// Label atom
    Label,
    /// Delete molecule
    Delete,
    /// Load molecule
    Load,
}
// d66e839e ends here

// [[file:../bevy.note::3d0c7156][3d0c7156]]
impl Plugin for RemoteConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ServerPlugin);
        //     .add_console_command::<DeleteCommand, _>(delete_command)
        //     .add_system(disable_arcball_camera_in_console.after(ConsoleSet::ConsoleUI));
        //
    }
}
// 3d0c7156 ends here
