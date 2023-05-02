// [[file:../bevy.note::6c039888][6c039888]]
// #![deny(warnings)]
#![deny(clippy::all)]

use bevy::prelude::*;

use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
use bevy_console::{ConsoleConfiguration, ConsolePlugin, ConsoleSet};
use bevy_console::{ConsoleOpen, PrintConsoleLine};
use bevy_panorbit_camera::PanOrbitCamera;

use gut::cli::*;
// 6c039888 ends here

// [[file:../bevy.note::*base][base:1]]
pub struct CmdConsolePlugin;
// base:1 ends here

// [[file:../bevy.note::101c2ae1][101c2ae1]]
fn disable_arcball_camera_in_console(console: Res<ConsoleOpen>, mut arcball_camera_query: Query<&mut PanOrbitCamera>) {
    if let Ok(mut arcball_camera) = arcball_camera_query.get_single_mut() {
        arcball_camera.enabled = !console.open;
    }
}
// 101c2ae1 ends here

// [[file:../bevy.note::22cddf8a][22cddf8a]]
/// Delete molecule
#[derive(Parser, ConsoleCommand)]
#[command(name = "delete")]
struct DeleteCommand {
    //
}

fn delete_command(
    mut commands: Commands,
    mut cmd: ConsoleCommand<DeleteCommand>,
    mut molecule_query: Query<Entity, With<crate::player::Molecule>>,
) {
    if let Some(Ok(DeleteCommand {})) = cmd.take() {
        if let Ok(molecule_entity) = molecule_query.get_single() {
            info!("remove molecule");
            commands.entity(molecule_entity).despawn_recursive();
            cmd.ok();
        }
    }
}
// 22cddf8a ends here

// [[file:../bevy.note::05bb2f53][05bb2f53]]
impl Plugin for CmdConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConsolePlugin)
            .add_console_command::<DeleteCommand, _>(delete_command)
            .add_system(disable_arcball_camera_in_console.after(ConsoleSet::ConsoleUI));
        //
    }
}
// 05bb2f53 ends here
