// [[file:../bevy.note::ff77face][ff77face]]
use gut::cli::*;
use gut::fs::*;
use gut::prelude::Result;
// ff77face ends here

// [[file:../bevy.note::49c1ea76][49c1ea76]]
use gchemol::prelude::*;
use gchemol::Molecule;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_mod_picking::prelude::{DebugPickingPlugin, DefaultPickingPlugins};
use bevy_panorbit_camera::PanOrbitCameraPlugin;

use bevy::app::AppExit;
fn exit_on_q(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: ResMut<Events<AppExit>>) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_events.send(AppExit);
    }
}

fn set_window_title(mut window: Query<&mut Window>) {
    if let Ok(mut window) = window.get_single_mut() {
        let version = env!("CARGO_PKG_VERSION");
        window.title = format!("gchemol view {version}");
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
/// A simple molecule viewer
pub struct ViewerCli {
    /// path to molecule to compute
    molfile: Option<PathBuf>,
}

impl ViewerCli {
    pub fn enter_main() -> Result<()> {
        let args = Self::parse();

        let log_plugin = LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "wgpu=error,gchemol_view=debug".to_string(),
        };
        let window_plugin = WindowPlugin {
            // exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            ..default()
        };
        let default_plugin = DefaultPlugins.set(log_plugin).set(window_plugin);
        let mut app = App::new();
        app.add_plugins(
            default_plugin
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(PanOrbitCameraPlugin);

        let mols = if let Some(molfile) = args.molfile {
            let mut mols: Vec<_> = gchemol::io::read(&molfile)?.collect();
            info!("Loaded {} molecules from {:?}", mols.len(), molfile);
            // FIXME: refactor when UI ready
            for mol in mols.iter_mut() {
                let lat = mol.unbuild_crystal();
                mol.rebond();
                mol.lattice = lat;
            }
            mols
        } else {
            info!("No molecule loaded.");
            vec![]
        };
        let mol_plugin = crate::molecule::MoleculePlugin::from_mols(mols);

        app.add_plugin(EguiPlugin)
            // do not show debug ui
            .add_plugins(DefaultPickingPlugins.build().disable::<DebugPickingPlugin>())
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::desktop_app())
            // Set background color
            .insert_resource(ClearColor(Color::BLACK))
            .add_plugin(mol_plugin)
            .add_plugin(crate::ui::LabelPlugin::default())
            .add_plugin(crate::net::ServerPlugin)
            .add_startup_system(set_window_title)
            .add_system(exit_on_q)
            .add_system(bevy::window::exit_on_primary_closed)
            .run();

        Ok(())
    }
}
// 49c1ea76 ends here
