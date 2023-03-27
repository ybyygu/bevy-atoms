// [[file:../bevy.note::a83ae206][a83ae206]]
use bevy::prelude::*;

use bevy_mod_picking::{PickableBundle, PickingCameraBundle};
// a83ae206 ends here

// [[file:../bevy.note::92de9269][92de9269]]
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

pub struct MoleculePlugin;

impl Plugin for MoleculePlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<Molecule>()
            .add_startup_system(spawn_molecule)
            .add_plugin(LookTransformPlugin)
            .add_plugin(OrbitCameraPlugin::default());
    }
}
// 92de9269 ends here

// [[file:../bevy.note::031857dd][031857dd]]
#[derive(Clone, Debug, Component)]
pub struct Atom {
    element: usize,
    symbol: String,
    label: String,
    color: Color,
    radius: f32,
}

#[derive(Clone, Copy, Debug, Component)]
pub struct Bond;

/// 定义原子或化学键的位置
#[derive(Component, Clone, Copy, Debug, Deref, DerefMut)]
pub struct Position(Vec3);

/// 定义原子或化学键的编号
#[derive(Clone, Copy, Debug, Component)]
pub struct Index(pub u64);
// 031857dd ends here

// [[file:../bevy.note::c068ff9c][c068ff9c]]
#[derive(Resource)]
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
// c068ff9c ends here

// [[file:../bevy.note::deffe145][deffe145]]
pub fn spawn_molecule(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mol_query: Res<Molecule>,
) {
    let mut mol = gchemol_core::Molecule::from_database("CH4");
    mol.rebond();
    for (i, a) in mol.atoms() {
        let [x, y, z] = a.position();
        let radius = ((a.get_cov_radius().unwrap_or(0.5) + 0.5) / 3.0) as f32;
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius,
                    ..Default::default()
                })),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
                ..default()
            })
            .insert(PickableBundle::default());
    }
    // add chemical bonds
    for (i, j, b) in mol.bonds() {
        let ai = mol.get_atom_unchecked(i);
        let aj = mol.get_atom_unchecked(j);
        let pi: Vec3 = ai.position().map(|v| v as f32).into();
        let pj: Vec3 = aj.position().map(|v| v as f32).into();
        let center = (pi + pj) / 2.0;
        let dij = pj - pi;
        let lij = dij.length();
        let rot = Quat::from_rotation_arc(Vec3::Y, dij.normalize());
        let transform = Transform::from_translation(center).with_rotation(rot);
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.07,
                height: lij,
                ..default()
            })),
            material: materials.add(Color::BLUE.into()),
            transform,
            ..default()
        });
    }

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });

    // 缩放平移旋转控制 (中键: 缩放, Ctrl-Left: 旋转, Right: 平移)
    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_wheel_zoom_sensitivity: 0.005,
                smoothing_weight: 0.02,
                ..Default::default()
            },
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ))
        .insert(PickingCameraBundle::default());
}
// deffe145 ends here
