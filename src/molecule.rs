// [[file:../bevy.note::a83ae206][a83ae206]]
use bevy::prelude::*;

use bevy_mod_picking::{PickableBundle, PickingCameraBundle};
// for lattice
use bevy_prototype_debug_lines::*;
// a83ae206 ends here

// [[file:../bevy.note::92de9269][92de9269]]
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};
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
// c068ff9c ends here

// [[file:../bevy.note::bb92e200][bb92e200]]
fn as_vec3(p: impl Into<[f64; 3]>) -> Vec3 {
    let p = p.into();
    Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32)
}

fn show_lattice(lat: &gchemol_core::Lattice, lines: &mut DebugLines) {
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
    lines.line_colored(p0, p1, f32::MAX, Color::RED);
    lines.line_colored(p0, p2, f32::MAX, Color::YELLOW);
    lines.line_colored(p0, p3, f32::MAX, Color::BLUE);
    lines.line_colored(p1, p4, f32::MAX, Color::WHITE);
    lines.line_colored(p1, p5, f32::MAX, Color::WHITE);
    lines.line_colored(p2, p4, f32::MAX, Color::WHITE);
    lines.line_colored(p2, p6, f32::MAX, Color::WHITE);
    lines.line_colored(p3, p5, f32::MAX, Color::WHITE);
    lines.line_colored(p3, p6, f32::MAX, Color::WHITE);
    lines.line_colored(p7, p4, f32::MAX, Color::WHITE);
    lines.line_colored(p7, p5, f32::MAX, Color::WHITE);
    lines.line_colored(p7, p6, f32::MAX, Color::WHITE);
}
// bb92e200 ends here

// [[file:../bevy.note::deffe145][deffe145]]
fn get_color(atom: &gchemol_core::Atom) -> Color {
    match atom.symbol() {
        "H" => Color::rgb_u8(255, 255, 255),
        "He" => Color::rgb_u8(217, 255, 255),
        "Li" => Color::rgb_u8(204, 128, 255),
        "Be" => Color::rgb_u8(194, 255, 0),
        "B" => Color::rgb_u8(255, 181, 181),
        "C" => Color::rgb_u8(144, 144, 144),
        "N" => Color::rgb_u8(48, 80, 248),
        "O" => Color::rgb_u8(255, 13, 13),
        "F" => Color::rgb_u8(144, 224, 80),
        "Ne" => Color::rgb_u8(179, 227, 245),
        "Na" => Color::rgb_u8(171, 92, 242),
        "Mg" => Color::rgb_u8(138, 255, 0),
        "Al" => Color::rgb_u8(191, 166, 166),
        "Si" => Color::rgb_u8(240, 200, 160),
        "P" => Color::rgb_u8(255, 128, 0),
        "S" => Color::rgb_u8(255, 255, 48),
        "Cl" => Color::rgb_u8(31, 240, 31),
        "Ar" => Color::rgb_u8(128, 209, 227),
        "K" => Color::rgb_u8(143, 64, 212),
        "Ca" => Color::rgb_u8(61, 255, 0),
        "Sc" => Color::rgb_u8(230, 230, 230),
        "Ti" => Color::rgb_u8(191, 194, 199),
        "V" => Color::rgb_u8(166, 166, 171),
        "Cr" => Color::rgb_u8(138, 153, 199),
        "Mn" => Color::rgb_u8(156, 122, 199),
        "Fe" => Color::rgb_u8(224, 102, 51),
        "Co" => Color::rgb_u8(240, 144, 160),
        "Ni" => Color::rgb_u8(80, 208, 80),
        "Cu" => Color::rgb_u8(200, 128, 51),
        "Zn" => Color::rgb_u8(125, 128, 176),
        "Ga" => Color::rgb_u8(194, 143, 143),
        "Ge" => Color::rgb_u8(102, 143, 143),
        "As" => Color::rgb_u8(189, 128, 227),
        "Se" => Color::rgb_u8(255, 161, 0),
        "Br" => Color::rgb_u8(166, 41, 41),
        "Kr" => Color::rgb_u8(92, 184, 209),
        "Rb" => Color::rgb_u8(112, 46, 176),
        "Sr" => Color::rgb_u8(0, 255, 0),
        "Y" => Color::rgb_u8(148, 255, 255),
        "Zr" => Color::rgb_u8(148, 224, 224),
        "Nb" => Color::rgb_u8(115, 194, 201),
        "Mo" => Color::rgb_u8(84, 181, 181),
        "Tc" => Color::rgb_u8(59, 158, 158),
        "Ru" => Color::rgb_u8(36, 143, 143),
        "Rh" => Color::rgb_u8(10, 125, 140),
        "Pd" => Color::rgb_u8(0, 105, 133),
        "Ag" => Color::rgb_u8(192, 192, 192),
        "Cd" => Color::rgb_u8(255, 217, 143),
        "In" => Color::rgb_u8(166, 117, 115),
        "Sn" => Color::rgb_u8(102, 128, 128),
        "Sb" => Color::rgb_u8(158, 99, 181),
        "Te" => Color::rgb_u8(212, 122, 0),
        "I" => Color::rgb_u8(148, 0, 148),
        "Xe" => Color::rgb_u8(66, 158, 176),
        "Cs" => Color::rgb_u8(87, 23, 143),
        "Ba" => Color::rgb_u8(0, 201, 0),
        "La" => Color::rgb_u8(112, 212, 255),
        "Ce" => Color::rgb_u8(255, 255, 199),
        "Pr" => Color::rgb_u8(217, 255, 199),
        "Nd" => Color::rgb_u8(199, 255, 199),
        "Pm" => Color::rgb_u8(163, 255, 199),
        "Sm" => Color::rgb_u8(143, 255, 199),
        "Eu" => Color::rgb_u8(97, 255, 199),
        "Gd" => Color::rgb_u8(69, 255, 199),
        "Tb" => Color::rgb_u8(48, 255, 199),
        "Dy" => Color::rgb_u8(31, 255, 199),
        "Ho" => Color::rgb_u8(0, 255, 156),
        "Er" => Color::rgb_u8(0, 230, 117),
        "Tm" => Color::rgb_u8(0, 212, 82),
        "Yb" => Color::rgb_u8(0, 191, 56),
        "Lu" => Color::rgb_u8(0, 171, 36),
        "Hf" => Color::rgb_u8(77, 194, 255),
        "Ta" => Color::rgb_u8(77, 166, 255),
        "W" => Color::rgb_u8(33, 148, 214),
        "Re" => Color::rgb_u8(38, 125, 171),
        "Os" => Color::rgb_u8(38, 102, 150),
        "Ir" => Color::rgb_u8(23, 84, 135),
        "Pt" => Color::rgb_u8(208, 208, 224),
        "Au" => Color::rgb_u8(255, 209, 35),
        "Hg" => Color::rgb_u8(184, 184, 208),
        "Tl" => Color::rgb_u8(166, 84, 77),
        "Pb" => Color::rgb_u8(87, 89, 97),
        "Bi" => Color::rgb_u8(158, 79, 181),
        "Po" => Color::rgb_u8(171, 92, 0),
        "At" => Color::rgb_u8(117, 79, 69),
        "Rn" => Color::rgb_u8(66, 130, 150),
        "Fr" => Color::rgb_u8(66, 0, 102),
        "Ra" => Color::rgb_u8(0, 125, 0),
        "Ac" => Color::rgb_u8(112, 171, 250),
        "Th" => Color::rgb_u8(0, 186, 255),
        "Pa" => Color::rgb_u8(0, 161, 255),
        "U" => Color::rgb_u8(0, 143, 255),
        "Np" => Color::rgb_u8(0, 128, 255),
        "Pu" => Color::rgb_u8(0, 107, 255),
        "Am" => Color::rgb_u8(84, 92, 242),
        "Cm" => Color::rgb_u8(120, 92, 227),
        "Bk" => Color::rgb_u8(138, 79, 227),
        "Cf" => Color::rgb_u8(161, 54, 212),
        "Es" => Color::rgb_u8(179, 31, 212),
        "Fm" => Color::rgb_u8(179, 31, 186),
        "Md" => Color::rgb_u8(179, 13, 166),
        "No" => Color::rgb_u8(189, 13, 135),
        "Lr" => Color::rgb_u8(199, 0, 102),
        "Rf" => Color::rgb_u8(204, 0, 89),
        "Db" => Color::rgb_u8(209, 0, 79),
        "Sg" => Color::rgb_u8(217, 0, 69),
        "Bh" => Color::rgb_u8(224, 0, 56),
        "Hs" => Color::rgb_u8(230, 0, 46),
        "Mt" => Color::rgb_u8(235, 0, 38),
        _ => Color::RED,
    }
}

pub fn spawn_molecule(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lines: ResMut<DebugLines>,
    mol: Res<Molecule>,
) {
    let mol = &mol.inner;
    for (i, a) in mol.atoms() {
        let [x, y, z] = a.position();
        let radius = ((a.get_cov_radius().unwrap_or(0.5) + 0.5) / 3.0) as f32;
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius,
                    ..Default::default()
                })),
                material: materials.add(get_color(a).into()),
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
            material: materials.add(Color::GRAY.into()),
            transform,
            ..default()
        });
    }
    // lattice
    if let Some(lat) = mol.get_lattice() {
        show_lattice(lat, &mut lines);
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
                mouse_wheel_zoom_sensitivity: 0.01,
                smoothing_weight: 0.08,
                ..Default::default()
            },
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ))
        .insert(PickingCameraBundle::default());
}
// deffe145 ends here

// [[file:../bevy.note::8ec82258][8ec82258]]
#[derive(Debug, Clone)]
pub struct MoleculePlugin {
    mol: Molecule,
}

impl MoleculePlugin {
    pub fn from_mol(inner: gchemol_core::Molecule) -> Self {
        Self { mol: Molecule { inner } }
    }
}

impl Plugin for MoleculePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.mol.clone())
            .add_startup_system(spawn_molecule)
            .add_plugin(LookTransformPlugin)
            .add_plugin(DebugLinesPlugin::default())
            .add_plugin(OrbitCameraPlugin::default());
    }
}
// 8ec82258 ends here
