// [[file:../bevy.note::*imports][imports:1]]
use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};

fn visibility(visible: bool) -> Visibility {
    if visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    }
}
// imports:1 ends here

// [[file:../bevy.note::5cf783bd][5cf783bd]]
fn get_atom_display_size(a: &gchemol_core::Atom) -> f64 {
    // ((a.get_cov_radius().unwrap_or(0.5) + 0.5) / 2.0) as f32
    let r = a.get_cov_radius().unwrap_or(1.0);
    (r * 0.3 + 0.7) * 0.45
}
// 5cf783bd ends here

// [[file:../bevy.note::4f2c9201][4f2c9201]]
fn get_atom_color(atom: &gchemol_core::Atom) -> Color {
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
// 4f2c9201 ends here

// [[file:../bevy.note::0b92cef9][0b92cef9]]
#[derive(Clone, Debug, Component)]
pub struct Atom {
    // symbol: String,
    // label: String,
    color: Color,
    radius: f32,
    position: Vec3,
    visible: bool,
}

impl Atom {
    pub fn new(a: &gchemol_core::Atom) -> Self {
        let radius = get_atom_display_size(a) as f32;
        let color = get_atom_color(a);
        let position = a.position().map(|v| v as f32).into();

        Self {
            position,
            color,
            radius,
            visible: true,
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

#[derive(Component)]
enum AtomLabel {
    None,
    Text(TextBundle),
}

#[derive(Bundle)]
pub struct AtomBundle {
    pbr: PbrBundle,
    atom: Atom,
    label: AtomLabel, // cannot use Option<TextBundle> here for Bundle trait constraint
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
            label: AtomLabel::None,
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
        let center = (pi + pj) / 2.0;
        let dij = pj - pi;
        let lij = dij.length();
        let rot = Quat::from_rotation_arc(Vec3::Y, dij.normalize());
        let transform = Transform::from_translation(center).with_rotation(rot);
        let visibility = visibility(bond.visible);
        Self {
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 0.07,
                    height: lij,
                    ..default()
                })),
                material: materials.add(Color::GRAY.into()),
                transform,
                visibility,
                ..default()
            },
            bond,
        }
    }
}
// 5a5c8b3f ends here

// [[file:../bevy.note::4306e081][4306e081]]
impl AtomBundle {
    pub fn create_label(&mut self, asset_server: &Res<AssetServer>) {
        info!("create label");
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let style = Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(50.),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        };

        let text = TextBundle::from_section(
            "C",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(style);

        self.label = AtomLabel::Text(text);
    }

    pub fn update_label(&mut self, camera: &Camera, camera_global_transform: &GlobalTransform) {
        match &mut self.label {
            AtomLabel::Text(text) => {
                let world_position = text.global_transform.translation() + Vec3::Y;
                match camera.world_to_viewport(camera_global_transform, world_position) {
                    Some(viewport_position) => {
                        text.style.position.top = Val::Px(viewport_position.y);
                        text.style.position.left = Val::Px(viewport_position.x);
                    }
                    None => {
                        // A hack to hide the text when the it's behind the camera
                        text.style.position.bottom = Val::Px(-1000.0);
                    }
                }
            }
            _ => {}
        }
    }
}
// 4306e081 ends here
