// [[file:../bevy.note::*imports][imports:1]]
use gut::cli::*;
use gut::fs::*;
use gut::prelude::*;
// imports:1 ends here

// [[file:../bevy.note::101c2ae1][101c2ae1]]
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
use bevy_console::{ConsoleConfiguration, ConsolePlugin};

/// Prints given arguments to the console
#[derive(Parser, ConsoleCommand)]
#[command(name = "log")]
struct LogCommand {
    /// Message to print
    msg: String,
    /// Number of times to print message
    num: Option<i64>,
}

fn log_command(mut log: ConsoleCommand<LogCommand>, atoms: Query<(&Transform), With<crate::player::Atom>>) {
    if let Some(Ok(LogCommand { msg, num })) = log.take() {
        let repeat_count = num.unwrap_or(1);

        for _ in 0..repeat_count {
            reply!(log, "{msg}");
        }

        for player_transform in atoms.iter() {
            reply!(
                log,
                "Pos: {:.2}, {:.2}, {:.2}",
                player_transform.translation.x,
                player_transform.translation.y,
                player_transform.translation.z,
            );
        }

        log.ok();
    }
}
// 101c2ae1 ends here

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
        let mut mols: Vec<_> = gchemol::io::read(&args.molfile)?.collect();
        // FIXME: should be refactored when UI is ready
        for mol in mols.iter_mut() {
            let lat = mol.unbuild_crystal();
            mol.recenter();
            mol.rebond();
            mol.lattice = lat;
        }
        let mol_plugin = crate::molecule::MoleculePlugin::from_mols(mols);

        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                ..default()
            }))
            // .add_plugin(EguiPlugin)
            .add_plugins(DefaultPickingPlugins)
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::desktop_app())
            .add_plugin(ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                // override config here
                ..Default::default()
            })
            .add_plugin(mol_plugin)
            .add_console_command::<LogCommand, _>(log_command)
            .run();

        Ok(())
    }
}
// 49c1ea76 ends here
