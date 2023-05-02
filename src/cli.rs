// [[file:../bevy.note::ff77face][ff77face]]
use gut::cli::*;
use gut::fs::*;
use gut::prelude::Result;
// ff77face ends here

// [[file:../bevy.note::101c2ae1][101c2ae1]]
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
use bevy_console::{ConsoleConfiguration, ConsolePlugin, ConsoleSet};

use bevy_console::{ConsoleOpen, PrintConsoleLine};
use bevy_panorbit_camera::PanOrbitCamera;
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

    #[arg(long)]
    /// Client side only (ad-hoc)
    client: bool,
}

impl ViewerCli {
    pub fn enter_main() -> Result<()> {
        let args = Self::parse();

        // FIXME: remove when net ready
        if args.client {
            let mol = gchemol::Molecule::from_file(&args.molfile)?;
            let client = reqwest::blocking::Client::builder().build().expect("reqwest client");
            let uri = format!("http://{}/view-molecule", "127.0.0.1:3039");
            let resp = client.post(&uri).json(&mol).send()?.text()?;
            dbg!(resp);
            return Ok(());
        }
        let mut mols: Vec<_> = gchemol::io::read(&args.molfile)?.collect();

        // FIXME: should be refactored when UI is ready
        for mol in mols.iter_mut() {
            let lat = mol.unbuild_crystal();
            // mol.recenter();
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
            .add_plugin(crate::net::ServerPlugin)
            .add_plugins(DefaultPickingPlugins)
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::desktop_app())
            .add_plugin(ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                // override config here
                ..Default::default()
            })
            .add_plugin(mol_plugin)
            .add_plugin(crate::ui::LabelPlugin::default())
            .add_console_command::<DeleteCommand, _>(delete_command)
            .add_system(disable_arcball_camera_in_console.after(ConsoleSet::ConsoleUI))
            .run();

        Ok(())
    }
}
// 49c1ea76 ends here
