// [[file:../bevy.note::a83ae206][a83ae206]]
#![deny(warnings)]
#![deny(clippy::all)]

use bevy::prelude::*;

use bevy_mod_picking::prelude::*;
// a83ae206 ends here

// [[file:../bevy.note::031857dd][031857dd]]
use crate::base::FrameIndex;
// 031857dd ends here

// [[file:../bevy.note::711fbcb5][711fbcb5]]
use bevy_panorbit_camera::PanOrbitCamera;

fn update_light_with_camera(
    mut param_set: ParamSet<(
        Query<(&mut Transform, With<DirectionalLight>)>,
        Query<&Transform, With<PanOrbitCamera>>,
    )>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok(camera) = param_set.p1().get_single() {
        let camera_position = camera.translation;
        for (mut transform, _mesh) in param_set.p0().iter_mut() {
            transform.translation = camera_position;
        }
    }

    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        let _viewport = camera.world_to_viewport(camera_transform, Vec3::new(2.144404, 2.2027268, 2.6483808));
    }
}
// 711fbcb5 ends here

// [[file:../bevy.note::b93672bb][b93672bb]]
fn setup_lights(commands: &mut Commands) {
    // light
    // ambient light
    let illuminance = 5500.0;
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.20,
    });
    let trans = Transform::from_xyz(5., 5., 5.);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance,
            ..default()
        },
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    let trans = Transform::from_xyz(-5., 5., -5.);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance,
            ..default()
        },
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    let trans = Transform::from_xyz(5., 5., -5.);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance,
            ..default()
        },
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    let trans = Transform::from_xyz(5., -5., -5.);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance,
            ..default()
        },
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
// b93672bb ends here

// [[file:../bevy.note::c068ff9c][c068ff9c]]
#[derive(Resource, Clone, Debug, Default)]
pub struct MoleculeTrajectory {
    pub mols: Vec<gchemol::Molecule>,
}

/// Visilization state
#[derive(Resource, Clone, Debug, Default)]
pub struct VisilizationState {
    pub display_label: bool,
}

impl MoleculeTrajectory {
    pub fn save_as(&self, path: &std::path::Path) {
        if let Err(err) = gchemol::io::write(path, &self.mols) {
            error!("Write molecules error: {err:?}");
        }
    }
}
// c068ff9c ends here

// [[file:../bevy.note::20198b2d][20198b2d]]
fn keyboard_animation_control(keyboard_input: Res<Input<KeyCode>>, mut current_frame: ResMut<CurrentFrame>) {
    if keyboard_input.just_pressed(KeyCode::Right) {
        current_frame.next();
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        current_frame.prev();
    }
}

fn traj_animation_player(
    traj: Res<MoleculeTrajectory>,
    current_frame: Res<CurrentFrame>,
    mut visibility_query: Query<(&mut Visibility, &FrameIndex)>,
) {
    let nframes = traj.mols.len();
    if let Some(ci) = current_frame.index(nframes) {
        for (mut visibility, FrameIndex(fi)) in visibility_query.iter_mut() {
            if *fi == ci as usize {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
// 20198b2d ends here

// [[file:../bevy.note::31795e08][31795e08]]
use crate::base::AtomIndex;

pub fn get_selected_atoms(selection_query: &Query<(&AtomIndex, &mut PickSelection)>) -> Vec<usize> {
    let mut selected = vec![];
    for (AtomIndex(i), selection) in selection_query.iter() {
        if selection.is_selected {
            selected.push(*i);
        }
    }
    selected
}
// 31795e08 ends here

// [[file:../bevy.note::1c6c0570][1c6c0570]]
use crate::base::CurrentFrame;

pub fn spawn_molecules(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    traj: Res<MoleculeTrajectory>,
) {
    // light
    // ambient light
    setup_lights(&mut commands);

    let center = traj
        .mols
        .iter()
        .next()
        .map(|mol| mol.center_of_geometry())
        .unwrap_or_default()
        .map(|x| x as f32);
    let arcball_camera = PanOrbitCamera {
        focus: center.into(),
        allow_upside_down: true,
        enabled: true,
        ..default()
    };

    // mouse: zoom, rotate and translate
    commands
        .spawn(Camera3dBundle {
            // projection: Projection::Orthographic(OrthographicProjection {
            //     near: -500.0,
            //     far: 500.0,
            //     ..default()
            // }),
            ..default()
        })
        .insert(arcball_camera)
        .insert(RaycastPickCamera::default());

    // create atoms and bonds
    for (fi, mol) in traj.mols.iter().enumerate() {
        // only show the first frame
        let visible = fi == 0;
        crate::base::spawn_molecule(mol, visible, fi, &mut commands, &mut meshes, &mut materials);
    }
}
// 1c6c0570 ends here

// [[file:../bevy.note::92f358a8][92f358a8]]
use bevy::window::PrimaryWindow;
use gchemol::Molecule;
use std::path::Path;

/// update molecule title and bonds for better view
fn update_mol_from_path(mol: &mut Molecule, f: &Path) {
    // take file name and its parent directory as the molecule title
    let mut s: Vec<_> = f.iter().rev().take(2).filter_map(|x| x.to_str()).collect();
    if !s.is_empty() {
        s.reverse();
        let s = s.join("/");
        mol.set_title(s);
    }
    if mol.nbonds() == 0 {
        let lat = mol.unbuild_crystal();
        mol.rebond();
        mol.lattice = lat;
    }
}

/// Load molecule if files were dropped into main molecule window
fn drag_and_drop_files(
    mut drag_drop_events: EventReader<FileDragAndDrop>,
    mut mol_event_writer: EventWriter<crate::net::StreamEvent>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    use gchemol::prelude::FromFile;

    let mut mols = vec![];
    let primary_entity = primary_window.single();
    for d in drag_drop_events.iter() {
        if let FileDragAndDrop::DroppedFile { path_buf, window } = d {
            // drop into main window
            if *window == primary_entity {
                if path_buf.is_file() {
                    info!("Dropped a file: {:?}", path_buf);
                    if let Ok(mut mol) = Molecule::from_file(&path_buf) {
                        update_mol_from_path(&mut mol, &path_buf);
                        mols.push(mol);
                    }
                } else if path_buf.is_dir() {
                    info!("Dropped a directory: {:?}", path_buf);
                    let files = gchemol::io::find_files("", &path_buf, true);
                    for f in files {
                        if let Ok(mut mol) = Molecule::from_file(&f) {
                            update_mol_from_path(&mut mol, &f);
                            mols.push(mol);
                        }
                    }
                }
            }
        }
    }

    if !mols.is_empty() {
        info!("Dropped {} Molecules.", mols.len());
        let command = crate::net::RemoteCommand::Load(mols);
        mol_event_writer.send(crate::net::StreamEvent(command));
    }
}
// 92f358a8 ends here

// [[file:../bevy.note::8ec82258][8ec82258]]
#[derive(Debug, Clone)]
pub struct MoleculePlugin {
    traj: MoleculeTrajectory,
}

impl MoleculePlugin {
    /// Create animation from a vec of molecules
    pub fn from_mols(mols: Vec<gchemol::Molecule>) -> Self {
        Self {
            traj: MoleculeTrajectory { mols },
        }
    }
}

impl Plugin for MoleculePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.traj.clone())
            .insert_resource(CurrentFrame::default())
            .insert_resource(VisilizationState::default())
            .add_startup_system(spawn_molecules)
            .add_system(update_light_with_camera)
            .add_system(keyboard_animation_control)
            .add_system(drag_and_drop_files)
            // .add_system(atom_selections)
            .add_system(traj_animation_player);
    }
}
// 8ec82258 ends here
