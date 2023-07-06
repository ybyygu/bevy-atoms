// [[file:../bevy.note::25b936c5][25b936c5]]
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub fn visibility(visible: bool) -> Visibility {
    if visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    }
}
// 25b936c5 ends here

// [[file:../bevy.note::3c4dc50f][3c4dc50f]]
use std::sync::OnceLock;

// the templates loaded from files
static ATOM_COLORS: OnceLock<Vec<Color>> = OnceLock::new();
// 3c4dc50f ends here

// [[file:../bevy.note::14379cd1][14379cd1]]
fn create_line_segment(
    pi: Vec3,
    pj: Vec3,
    visible: bool,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    radius: f32,
    color: Color,
) -> PbrBundle {
    let center = (pi + pj) / 2.0;
    let dij = pj - pi;
    let lij = dij.length();
    let rot = Quat::from_rotation_arc(Vec3::Y, dij.normalize());
    let transform = Transform::from_translation(center).with_rotation(rot);

    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder {
            radius,
            height: lij,
            ..default()
        })),
        visibility: visibility(visible),
        material: materials.add(color.into()),
        transform,
        ..default()
    }
}
// 14379cd1 ends here

// [[file:../bevy.note::52696eea][52696eea]]
#[derive(Resource, Clone, Debug, Default)]
pub struct CurrentFrame(pub isize);

impl CurrentFrame {
    /// Return the index of current frame. Return None if `nframes` is 0.
    pub fn index(&self, nframes: usize) -> Option<usize> {
        // no trajecotry loaded
        if nframes == 0 {
            None
        } else {
            // % operator not work for negative number. We need Euclidean division.
            // https://users.rust-lang.org/t/why-works-differently-between-rust-and-python/83911
            let n = self.0.rem_euclid(nframes as isize) as usize;
            Some(n)
        }
    }

    pub fn next(&mut self) {
        self.0 += 1;
    }

    pub fn prev(&mut self) {
        self.0 -= 1;
    }
}
// 52696eea ends here

// [[file:../bevy.note::5cf783bd][5cf783bd]]
fn get_atom_display_size(a: &gchemol::Atom) -> f64 {
    // ((a.get_cov_radius().unwrap_or(0.5) + 0.5) / 2.0) as f32
    let r = a.get_cov_radius().unwrap_or(1.0);
    (r * 0.3 + 0.7) * 0.45
}
// 5cf783bd ends here

// [[file:../bevy.note::4f2c9201][4f2c9201]]
fn get_atom_color(atom: &gchemol::Atom) -> Color {
    // initialize atom colors
    let atom_colors = ATOM_COLORS.get_or_init(|| {
        vec![
            Color::rgb_u8(20, 20, 20),    // dummy atom
            Color::rgb_u8(255, 255, 255), // H
            Color::rgb_u8(217, 255, 255), // He
            Color::rgb_u8(204, 128, 255), // Li
            Color::rgb_u8(194, 255, 0),   // Be
            Color::rgb_u8(255, 181, 181), // B
            Color::rgb_u8(144, 144, 144), // C
            Color::rgb_u8(48, 80, 248),   // N
            Color::rgb_u8(255, 13, 13),
            Color::rgb_u8(144, 224, 80),
            Color::rgb_u8(179, 227, 245),
            Color::rgb_u8(171, 92, 242),
            Color::rgb_u8(138, 255, 0),
            Color::rgb_u8(191, 166, 166),
            Color::rgb_u8(240, 200, 160),
            Color::rgb_u8(255, 128, 0),
            Color::rgb_u8(255, 255, 48),
            Color::rgb_u8(31, 240, 31),
            Color::rgb_u8(128, 209, 227),
            Color::rgb_u8(143, 64, 212),
            Color::rgb_u8(61, 255, 0),
            Color::rgb_u8(230, 230, 230),
            Color::rgb_u8(191, 194, 199),
            Color::rgb_u8(166, 166, 171),
            Color::rgb_u8(138, 153, 199),
            Color::rgb_u8(156, 122, 199),
            Color::rgb_u8(224, 102, 51),
            Color::rgb_u8(240, 144, 160),
            Color::rgb_u8(80, 208, 80),
            Color::rgb_u8(200, 128, 51),
            Color::rgb_u8(125, 128, 176),
            Color::rgb_u8(194, 143, 143),
            Color::rgb_u8(102, 143, 143),
            Color::rgb_u8(189, 128, 227),
            Color::rgb_u8(255, 161, 0),
            Color::rgb_u8(166, 41, 41),
            Color::rgb_u8(92, 184, 209),
            Color::rgb_u8(112, 46, 176),
            Color::rgb_u8(0, 255, 0),
            Color::rgb_u8(148, 255, 255),
            Color::rgb_u8(148, 224, 224),
            Color::rgb_u8(115, 194, 201),
            Color::rgb_u8(84, 181, 181),
            Color::rgb_u8(59, 158, 158),
            Color::rgb_u8(36, 143, 143),
            Color::rgb_u8(10, 125, 140),
            Color::rgb_u8(0, 105, 133),
            Color::rgb_u8(192, 192, 192),
            Color::rgb_u8(255, 217, 143),
            Color::rgb_u8(166, 117, 115),
            Color::rgb_u8(102, 128, 128),
            Color::rgb_u8(158, 99, 181),
            Color::rgb_u8(212, 122, 0),
            Color::rgb_u8(148, 0, 148),
            Color::rgb_u8(66, 158, 176),
            Color::rgb_u8(87, 23, 143),
            Color::rgb_u8(0, 201, 0),
            Color::rgb_u8(112, 212, 255),
            Color::rgb_u8(255, 255, 199),
            Color::rgb_u8(217, 255, 199),
            Color::rgb_u8(199, 255, 199),
            Color::rgb_u8(163, 255, 199),
            Color::rgb_u8(143, 255, 199),
            Color::rgb_u8(97, 255, 199),
            Color::rgb_u8(69, 255, 199),
            Color::rgb_u8(48, 255, 199),
            Color::rgb_u8(31, 255, 199),
            Color::rgb_u8(0, 255, 156),
            Color::rgb_u8(0, 230, 117),
            Color::rgb_u8(0, 212, 82),
            Color::rgb_u8(0, 191, 56),
            Color::rgb_u8(0, 171, 36),
            Color::rgb_u8(77, 194, 255),
            Color::rgb_u8(77, 166, 255),
            Color::rgb_u8(33, 148, 214),
            Color::rgb_u8(38, 125, 171),
            Color::rgb_u8(38, 102, 150),
            Color::rgb_u8(23, 84, 135),
            Color::rgb_u8(208, 208, 224),
            Color::rgb_u8(255, 209, 35),
            Color::rgb_u8(184, 184, 208),
            Color::rgb_u8(166, 84, 77),
            Color::rgb_u8(87, 89, 97),
            Color::rgb_u8(158, 79, 181),
            Color::rgb_u8(171, 92, 0),
            Color::rgb_u8(117, 79, 69),
            Color::rgb_u8(66, 130, 150),
            Color::rgb_u8(66, 0, 102),
            Color::rgb_u8(0, 125, 0),
            Color::rgb_u8(112, 171, 250),
            Color::rgb_u8(0, 186, 255),
            Color::rgb_u8(0, 161, 255),
            Color::rgb_u8(0, 143, 255),
            Color::rgb_u8(0, 128, 255),
            Color::rgb_u8(0, 107, 255),
            Color::rgb_u8(84, 92, 242),
            Color::rgb_u8(120, 92, 227),
            Color::rgb_u8(138, 79, 227),
            Color::rgb_u8(161, 54, 212),
            Color::rgb_u8(179, 31, 212),
            Color::rgb_u8(179, 31, 186),
            Color::rgb_u8(179, 13, 166),
            Color::rgb_u8(189, 13, 135),
            Color::rgb_u8(199, 0, 102),
            Color::rgb_u8(204, 0, 89),
            Color::rgb_u8(209, 0, 79),
            Color::rgb_u8(217, 0, 69),
            Color::rgb_u8(224, 0, 56),
            Color::rgb_u8(230, 0, 46),
            Color::rgb_u8(235, 0, 38),
        ]
    });
    let n = atom.number();
    if n >= atom_colors.len() {
        Color::RED
    } else {
        atom_colors[n]
    }
}
// 4f2c9201 ends here

// [[file:../bevy.note::0b92cef9][0b92cef9]]
#[derive(Clone, Copy, Debug, Component)]
pub struct AtomIndex(pub usize);

/// Represent the displayed atom in 3D viewer
#[derive(Clone, Debug, Component)]
pub struct Atom {
    color: Color,
    visible: bool,
    radius: f32,
    position: Vec3,
    /// The text of label to be displayed
    label: Option<String>,
}

impl Atom {
    pub fn new(a: &gchemol::Atom) -> Self {
        let radius = get_atom_display_size(a) as f32;
        let color = get_atom_color(a);
        let position = a.position().map(|v| v as f32).into();

        Self {
            position,
            color,
            radius,
            visible: true,
            label: a.get_label().map(|x| x.to_owned()),
        }
    }

    /// Return the text of atom label.
    pub fn get_label(&self, sn: usize) -> String {
        self.label.clone().unwrap_or(sn.to_string())
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

#[derive(Bundle)]
pub struct AtomBundle {
    pbr: PbrBundle,
    atom: Atom,
}

impl AtomBundle {
    pub fn new(atom: Atom, mut meshes: &mut ResMut<Assets<Mesh>>, mut materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        let visibility = visibility(atom.visible);

        Self {
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: atom.radius,
                    ..Default::default()
                })),
                material: materials.add(atom.color.into()),
                transform: Transform::from_translation(atom.position),
                visibility,
                ..default()
            },
            atom,
            // label: AtomLabel::None,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.atom.position
    }

    pub fn global_transform(&self) -> GlobalTransform {
        self.pbr.global_transform
    }
}
// 0b92cef9 ends here

// [[file:../bevy.note::5a5c8b3f][5a5c8b3f]]
#[derive(Clone, Copy, Debug, Component)]
pub struct BondIndex(usize);

#[derive(Clone, Debug, Component)]
pub struct Bond {
    atom1: Atom,
    atom2: Atom,
    visible: bool,
}

impl Bond {
    pub fn new(atom1: Atom, atom2: Atom) -> Self {
        Self {
            atom1,
            atom2,
            visible: true,
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

#[derive(Bundle)]
pub struct BondBundle {
    pbr: PbrBundle,
    bond: Bond,
}

impl BondBundle {
    pub fn new(bond: Bond, mut meshes: &mut ResMut<Assets<Mesh>>, mut materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        let pi = bond.atom1.position;
        let pj = bond.atom2.position;
        let pbr = create_line_segment(pi, pj, bond.visible, meshes, materials, 0.07, Color::GRAY);

        Self { pbr, bond }
    }
}
// 5a5c8b3f ends here

// [[file:../bevy.note::3b90f445][3b90f445]]
#[derive(Clone, Debug, Component)]
pub struct Molecule;
// 3b90f445 ends here

// [[file:../bevy.note::38660d10][38660d10]]
#[derive(Clone, Debug, Component)]
pub struct Lattice;

fn as_vec3(p: impl Into<[f64; 3]>) -> Vec3 {
    let p = p.into();
    Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32)
}

fn create_lattice(
    lat: &gchemol::Lattice,
    visible: bool,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
) -> [PbrBundle; 12] {
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

    let radius = 0.03;
    [
        create_line_segment(p0, p1, visible, meshes, materials, radius, Color::RED),
        create_line_segment(p0, p2, visible, meshes, materials, radius, Color::YELLOW),
        create_line_segment(p0, p3, visible, meshes, materials, radius, Color::BLUE),
        create_line_segment(p1, p4, visible, meshes, materials, radius, Color::WHITE),
        create_line_segment(p1, p5, visible, meshes, materials, radius, Color::WHITE),
        create_line_segment(p2, p4, visible, meshes, materials, radius, Color::WHITE),
        create_line_segment(p2, p6, visible, meshes, materials, radius, Color::WHITE),
        create_line_segment(p3, p5, visible, meshes, materials, radius, Color::WHITE),
        create_line_segment(p3, p6, visible, meshes, materials, radius, Color::WHITE),
        create_line_segment(p7, p4, visible, meshes, materials, radius, Color::WHITE),
        create_line_segment(p7, p5, visible, meshes, materials, radius, Color::WHITE),
        create_line_segment(p7, p6, visible, meshes, materials, radius, Color::WHITE),
    ]
}
// 38660d10 ends here

// [[file:../bevy.note::c989001a][c989001a]]
// https://github.com/aevyrie/bevy_mod_picking/blob/main/examples/tinted_highlight.rs
use bevy::math::vec4;

const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    // do not react on hover
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial { ..matl.to_owned() })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: Color::YELLOW,
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: Color::YELLOW,
        ..matl.to_owned()
    })),
};
// c989001a ends here

// [[file:../bevy.note::d5c13162][d5c13162]]
#[derive(Clone, Copy, Debug, Component)]
pub struct FrameIndex(pub usize);

pub fn spawn_molecule(
    mol: &gchemol::Molecule,
    visible: bool,
    frame_index: usize,
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // let frame_name = Name::new(format!("frame-{frame_index}"));
    let frame_name = FrameIndex(frame_index);
    commands
        .spawn(SpatialBundle::default())
        .insert(Molecule)
        // for animation control
        .with_children(|commands| {
            // spawn atoms
            for (i, a) in mol.atoms() {
                let mut atom = Atom::new(a);
                atom.set_visible(visible);
                let mut atom_bundle = AtomBundle::new(atom, &mut meshes, &mut materials);
                commands
                    .spawn(atom_bundle)
                    .insert(AtomIndex(i))
                    .insert(frame_name)
                    .insert(PickableBundle::default())
                    .insert(RaycastPickTarget::default())
                    .insert(HIGHLIGHT_TINT);
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
                    .insert(frame_name);
            }

            // lattice
            if let Some(lat) = mol.get_lattice() {
                let vectors = create_lattice(lat, visible, meshes, materials);
                for v in vectors {
                    commands.spawn(v).insert(Lattice).insert(frame_name);
                }
            }
        });
}
// d5c13162 ends here
