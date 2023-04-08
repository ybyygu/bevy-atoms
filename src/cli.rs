// [[file:../bevy.note::*imports][imports:1]]
use gut::cli::*;
use gut::fs::*;
use gut::prelude::*;
// imports:1 ends here

// [[file:../bevy.note::49c1ea76][49c1ea76]]
use gchemol::prelude::*;
use gchemol::Molecule;

use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
/// A simple molecule viewer
pub struct ViewerCli {
    #[clap(flatten)]
    verbose: Verbosity,

    /// path to molecule to compute
    molfile: PathBuf,
}

impl ViewerCli {
    pub fn enter_main() -> Result<()> {
        let args = Self::parse();
        args.verbose.setup_logger();
        let mut mol = Molecule::from_file(&args.molfile)?;
        mol.unbuild_crystal();
        mol.recenter();
        mol.rebond();

        let mol_plugin = crate::molecule::MoleculePlugin::from_mol(mol);

        App::new()
            // .add_plugins(DefaultPlugins)
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                ..default()
            }))
            .add_plugin(EguiPlugin)
            .add_plugins(DefaultPickingPlugins)
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::desktop_app())
            .add_plugin(mol_plugin)
            .run();

        Ok(())
    }
}
// 49c1ea76 ends here
