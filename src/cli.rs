// [[file:../bevy.note::ff77face][ff77face]]
use gut::cli::*;
use gut::fs::*;
use gut::prelude::Result;
// ff77face ends here

// [[file:../bevy.note::101c2ae1][101c2ae1]]
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
use bevy_console::{ConsoleConfiguration, ConsolePlugin};

/// Label atoms with serial number or element symbols
#[derive(Parser, ConsoleCommand)]
#[command(name = "label")]
struct LabelCommand {
    /// Atoms to be selected. If not set, all atoms will be selected.
    selection: Option<String>,

    /// Delete atom labels
    #[arg(short, long)]
    delete: bool,
}

fn label_command(
    mut cmd: ConsoleCommand<LabelCommand>,
    mut state: ResMut<crate::molecule::VisilizationState>,
    mut visibility_query: Query<&mut Visibility, With<crate::molecule::AtomLabel>>,
) {
    if let Some(Ok(LabelCommand { selection, delete })) = cmd.take() {
        reply!(cmd, "{selection:?}");
        state.display_label = !delete;
        for mut visibility in &mut visibility_query {
            if delete {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
            }
        }

        cmd.ok();
    }
}
// 101c2ae1 ends here

// [[file:../bevy.note::49c1ea76][49c1ea76]]
use gchemol::prelude::*;
use gchemol::Molecule;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
/// A simple molecule viewer
pub struct ViewerCli {
    /// path to molecule to compute
    molfile: PathBuf,
}

impl ViewerCli {
    pub fn enter_main() -> Result<()> {
        let args = Self::parse();

        let mut mols: Vec<_> = gchemol::io::read(&args.molfile)?.collect();
        // FIXME: should be refactored when UI is ready
        for mol in mols.iter_mut() {
            let lat = mol.unbuild_crystal();
            mol.recenter();
            mol.rebond();
            mol.lattice = lat;
        }
        let mol_plugin = crate::molecule::MoleculePlugin::from_mols(mols);

        let log_plugin = LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "wgpu=error,gchemol_view=debug".to_string(),
        };
        let window_plugin = WindowPlugin {
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            ..default()
        };
        let default_plugin = DefaultPlugins.set(log_plugin).set(window_plugin);

        App::new()
            .add_plugins(
                default_plugin
                    .build()
                    .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
            )
            // .add_plugin(crate::GamePlugin)
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
            .add_console_command::<LabelCommand, _>(label_command)
            .run();

        Ok(())
    }
}
// 49c1ea76 ends here
