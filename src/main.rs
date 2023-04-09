// [[file:../bevy.note::1ba2a38a][1ba2a38a]]
// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

use gchemol_view::GamePlugin;

mod molecule;

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    use bevy_inspector_egui::prelude::*;
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

    // When building for WASM, print panics to the browser console
    console_error_panic_hook::set_once();

    let mut mol = gchemol_core::Molecule::from_database("CH4");
    mol.rebond();
    let mol_plugin = molecule::MoleculePlugin::from_mol(mol);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(mol_plugin)
        .run();
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let _ = gchemol_view::cli::ViewerCli::enter_main();
}
// 1ba2a38a ends here
