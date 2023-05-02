// [[file:../bevy.note::6c039888][6c039888]]
// #![deny(warnings)]
#![deny(clippy::all)]

use bevy::prelude::*;
use gut::cli::*;
// 6c039888 ends here

// [[file:../bevy.note::d66e839e][d66e839e]]
pub struct RemoteConsolePlugin;
// d66e839e ends here

// [[file:../bevy.note::*pub][pub:1]]
impl Plugin for RemoteConsolePlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugin(ConsolePlugin)
        //     .add_console_command::<DeleteCommand, _>(delete_command)
        //     .add_system(disable_arcball_camera_in_console.after(ConsoleSet::ConsoleUI));
        //
    }
}
// pub:1 ends here
