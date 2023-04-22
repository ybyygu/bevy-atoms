// [[file:../bevy.note::a83ae206][a83ae206]]
use crate::player::*;

use bevy::prelude::*;

use bevy_mod_picking::{PickableBundle, PickingCameraBundle};
// for lattice
use bevy_prototype_debug_lines::*;
// a83ae206 ends here

// [[file:../bevy.note::711fbcb5][711fbcb5]]
use crate::camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn update_light_with_camera(
    mut param_set: ParamSet<(
        Query<(&mut Transform, With<DirectionalLight>)>,
        Query<&Transform, With<PanOrbitCamera>>,
    )>,
) {
    let camera_position = param_set.p1().single().translation;
    for (mut transform, _mesh) in param_set.p0().iter_mut() {
        let distance = transform.translation.distance(camera_position);
        transform.translation = camera_position;
    }
}
// 711fbcb5 ends here

// [[file:../bevy.note::031857dd][031857dd]]
// #[derive(Clone, Copy, Debug, Component)]
// pub struct Bond;

// #[derive(Clone, Copy, Debug, Component)]
// pub struct Atom;

#[derive(Clone, Copy, Debug, Component)]
pub struct FrameIndex(usize);

#[derive(Clone, Copy, Debug, Component)]
pub struct AtomIndex(usize);

#[derive(Clone, Copy, Debug, Component)]
pub struct BondIndex(usize);
// 031857dd ends here

// [[file:../bevy.note::c068ff9c][c068ff9c]]
#[derive(Resource, Clone, Debug)]
pub struct Molecule {
    inner: gchemol_core::Molecule,
}

impl Default for Molecule {
    fn default() -> Self {
        let mut inner = gchemol_core::Molecule::from_database("CH4");
        inner.rebond();
        Self { inner }
    }
}

#[derive(Resource, Clone, Debug, Default)]
struct CurrentFrame(isize);

#[derive(Resource, Clone, Debug, Default)]
pub struct MoleculeTrajectory {
    mols: Vec<gchemol_core::Molecule>,
}
// c068ff9c ends here

// [[file:../bevy.note::bb92e200][bb92e200]]
fn as_vec3(p: impl Into<[f64; 3]>) -> Vec3 {
    let p = p.into();
    Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32)
}

fn show_lattice(lat: &gchemol_core::Lattice, lines: &mut DebugLines, duration: f32) {
    let p0 = lat.to_cart([0.0, 0.0, 0.0]);
    let p1 = lat.to_cart([1.0, 0.0, 0.0]);
    let p2 = lat.to_cart([0.0, 1.0, 0.0]);
    let p3 = lat.to_cart([0.0, 0.0, 1.0]);
    let p4 = lat.to_cart([1.0, 1.0, 0.0]);
    let p5 = lat.to_cart([1.0, 0.0, 1.0]);
    let p6 = lat.to_cart([0.0, 1.0, 1.0]);
    let p7 = lat.to_cart([1.0, 1.0, 1.0]);
    let p0 = as_vec3(p0);
    let p1 = as_vec3(p1);
    let p2 = as_vec3(p2);
    let p3 = as_vec3(p3);
    let p4 = as_vec3(p4);
    let p5 = as_vec3(p5);
    let p6 = as_vec3(p6);
    let p7 = as_vec3(p7);
    lines.line_colored(p0, p1, duration, Color::RED);
    lines.line_colored(p0, p2, duration, Color::YELLOW);
    lines.line_colored(p0, p3, duration, Color::BLUE);
    lines.line_colored(p1, p4, duration, Color::WHITE);
    lines.line_colored(p1, p5, duration, Color::WHITE);
    lines.line_colored(p2, p4, duration, Color::WHITE);
    lines.line_colored(p2, p6, duration, Color::WHITE);
    lines.line_colored(p3, p5, duration, Color::WHITE);
    lines.line_colored(p3, p6, duration, Color::WHITE);
    lines.line_colored(p7, p4, duration, Color::WHITE);
    lines.line_colored(p7, p5, duration, Color::WHITE);
    lines.line_colored(p7, p6, duration, Color::WHITE);
}
// bb92e200 ends here

// [[file:../bevy.note::20198b2d][20198b2d]]
// #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
// enum FrameState {
//     #[default]
//     Pause,
//     Play,
// }

fn play_animation(
    traj: Res<MoleculeTrajectory>,
    current_frame: Res<CurrentFrame>,
    mut visibility_query: Query<(&mut Visibility, &FrameIndex), Or<(With<Atom>, With<Bond>)>>,
) {
    let nframe = traj.mols.len() as isize;
    // % operator not work for negative number. We need Euclidean division.
    // https://users.rust-lang.org/t/why-works-differently-between-rust-and-python/83911
    let ci = current_frame.0.rem_euclid(nframe);
    for (mut visibility, FrameIndex(fi)) in visibility_query.iter_mut() {
        if *fi == ci as usize {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn frame_control(keyboard_input: Res<Input<KeyCode>>, mut current_frame: ResMut<CurrentFrame>) {
    if keyboard_input.just_pressed(KeyCode::Right) {
        current_frame.0 += 1;
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        current_frame.0 -= 1;
    }
}
// 20198b2d ends here

// [[file:../bevy.note::1c6c0570][1c6c0570]]
pub fn spawn_molecules(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lines: ResMut<DebugLines>,
    traj: Res<MoleculeTrajectory>,
) {
    for (fi, mol) in traj.mols.iter().enumerate() {
        // only show the first molecule on startup
        let visible = fi == 0;
        for (i, a) in mol.atoms() {
            let mut atom = Atom::new(a);
            atom.set_visible(visible);
            commands
                .spawn(AtomBundle::new(atom, &mut meshes, &mut materials))
                .insert(AtomIndex(i))
                .insert(FrameIndex(fi));
            // .insert(PickableBundle::default());
        }

        // add chemical bonds
        for (i, j, b) in mol.bonds() {
            let ai = mol.get_atom_unchecked(i);
            let aj = mol.get_atom_unchecked(j);
            let atom1 = Atom::new(ai);
            let atom2 = Atom::new(aj);
            let mut bond = Bond::new(atom1, atom2);
            bond.set_visible(visible);
            commands
                .spawn(BondBundle::new(bond, &mut meshes, &mut materials))
                .insert(FrameIndex(fi));
        }

        // lattice
        if let Some(lat) = mol.get_lattice() {
            show_lattice(lat, &mut lines, f32::MAX);
        }
    }

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

    // mouse: zoom, rotate and translate
    commands
        .spawn(Camera3dBundle::default())
        .insert(PanOrbitCamera::default())
        .insert(PickingCameraBundle::default());
}
// 1c6c0570 ends here

// [[file:../bevy.note::8ec82258][8ec82258]]
#[derive(Debug, Clone)]
pub struct MoleculePlugin {
    traj: MoleculeTrajectory,
}

impl MoleculePlugin {
    /// Create animation from a vec of molecules
    pub fn from_mols(mols: Vec<gchemol_core::Molecule>) -> Self {
        Self {
            traj: MoleculeTrajectory { mols },
        }
    }
}

impl Plugin for MoleculePlugin {
    fn build(&self, app: &mut App) {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.insert_resource(self.traj.clone())
            .insert_resource(CurrentFrame(0))
            .add_plugin(DebugLinesPlugin::default())
            .add_plugin(PanOrbitCameraPlugin)
            // .add_plugin(WorldInspectorPlugin::new())
            .add_system(update_light_with_camera);

        match self.traj.mols.len() {
            0 => {
                eprintln!("No molecule loaded!");
            }
            1 => {
                app.add_startup_system(spawn_molecules);
            }
            _ => {
                use bevy::app::StartupSet::PostStartup;
                app.add_startup_system(spawn_molecules)
                    // .add_system(frame_control.in_base_set(PostStartup));
                    .add_system(frame_control)
                    // .add_system(play_animation.in_base_set(PostStartup));
                    .add_system(play_animation);
            }
        }
    }
}
// 8ec82258 ends here
