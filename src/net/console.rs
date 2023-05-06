// [[file:../../bevy.note::6c039888][6c039888]]
// #![deny(warnings)]
#![deny(clippy::all)]

use super::{RemoteCommand, StreamEvent};
use crate::net::ServerPlugin;
use gchemol_core::Molecule;

use bevy::prelude::*;
// 6c039888 ends here

// [[file:../../bevy.note::d66e839e][d66e839e]]
pub struct RemoteConsolePlugin;
// d66e839e ends here

// [[file:../../bevy.note::22cddf8a][22cddf8a]]
fn delete_command(
    mut commands: Commands,
    mut molecule_query: Query<Entity, With<crate::player::Molecule>>,
    mut reader: EventReader<StreamEvent>,
) {
    for (_per_frame, StreamEvent(cmd)) in reader.iter().enumerate() {
        if let RemoteCommand::Delete = cmd {
            if let Ok(molecule_entity) = molecule_query.get_single() {
                info!("remove molecule");
                commands.entity(molecule_entity).despawn_recursive();
            }
        }
    }
}
// 22cddf8a ends here

// [[file:../../bevy.note::09fa2046][09fa2046]]
use bevy_panorbit_camera::PanOrbitCamera;

fn load_command(
    mut commands: Commands,
    mut reader: EventReader<StreamEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    molecule_query: Query<Entity, With<crate::player::Molecule>>,
    mut arcball_camera: Query<&mut PanOrbitCamera>,
) {
    for (_per_frame, StreamEvent(cmd)) in reader.iter().enumerate() {
        match cmd {
            RemoteCommand::Load(mols) => {
                // FIXME: rewrite
                let mol = &mols[0];
                info!("handle received mol: {}", mol.title());
                // remove existing molecule
                if let Ok(molecule_entity) = molecule_query.get_single() {
                    info!("molecule removed");
                    commands.entity(molecule_entity).despawn_recursive();
                }
                // show molecule on received
                crate::player::spawn_molecule(mol, true, 0, &mut commands, &mut meshes, &mut materials, &mut lines);
                // recenter view
                if let Ok(mut pan_orbit) = arcball_camera.get_single_mut() {
                    let center = mol.center_of_geometry().map(|x| x as f32);
                    pan_orbit.focus = center.into();
                }
                break;
            }
            _ => {
                //
            }
        }
    }
}
// 09fa2046 ends here

// [[file:../../bevy.note::3d0c7156][3d0c7156]]
impl Plugin for RemoteConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(delete_command).add_system(load_command);
    }
}
// 3d0c7156 ends here
