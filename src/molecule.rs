// [[file:../bevy.note::a83ae206][a83ae206]]
use crate::player::*;

use bevy::prelude::*;

use bevy_mod_picking::{PickableBundle, PickingCameraBundle};
// for lattice
use bevy_prototype_debug_lines::*;
// a83ae206 ends here

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

// [[file:../bevy.note::711fbcb5][711fbcb5]]
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

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
            let distance = transform.translation.distance(camera_position);
            transform.translation = camera_position;
        }
    }

    let (camera, camera_transform) = camera_query.single();
    let viewport = camera.world_to_viewport(camera_transform, Vec3::new(2.144404, 2.2027268, 2.6483808));
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

/// Visilization state
#[derive(Resource, Clone, Debug, Default)]
pub struct VisilizationState {
    pub display_label: bool,
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

// [[file:../bevy.note::8139ae6a][8139ae6a]]
#[derive(Component)]
pub struct AtomLabel {
    entity: Entity,
    offset: Vec3,
}

impl AtomLabel {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            offset: Vec3::ZERO,
        }
    }

    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }
}

fn create_label(asset_server: &Res<AssetServer>, text: String, visible: bool) -> TextBundle {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let style = Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            // top: Val::Px(50.),
            // right: Val::Px(15.0),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut text = TextBundle::from_section(
        text,
        TextStyle {
            font: font.clone(),
            font_size: 22.0,
            ..default()
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(style);

    let visibility = if visible { Visibility::Visible } else { Visibility::Hidden };
    text.visibility = visibility;
    text
}

fn update_atom_labels_with_camera(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut label_style_query: Query<(&AtomLabel, &mut Style, &CalculatedSize, &ComputedVisibility)>,
    transform_query: Query<&Transform>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();

    let window = windows.single();
    for (label, mut style, calc_size, visibility) in &mut label_style_query {
        if visibility.is_visible() {
            let label_size = calc_size.size;
            if let Ok(atom_transform) = transform_query.get(label.entity) {
                let atom_position = atom_transform.translation;
                if let Some(screen_position) = camera.world_to_viewport(camera_transform, atom_position) {
                    style.position.left = Val::Px(screen_position.x - label_size.x * 0.5 + label.offset.x);
                    style.position.top = Val::Px(window.height() - (screen_position.y + label_size.y * 0.5 + label.offset.y));
                } else {
                    // A hack to hide the text when the it's behind the camera
                    style.position.bottom = Val::Px(-1000.0);
                }
            }
        }
    }
}
// 8139ae6a ends here

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
    vis_state: Res<VisilizationState>,
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
    asset_server: Res<AssetServer>,
    traj: Res<MoleculeTrajectory>,
    // global_transforms: Query<&GlobalTransform>,
) {
    // light
    // ambient light
    setup_lights(&mut commands);

    // mouse: zoom, rotate and translate
    commands
        .spawn(Camera3dBundle::default())
        .insert(PanOrbitCamera::default())
        .insert(PickingCameraBundle::default());

    // create atoms and bonds
    for (fi, mol) in traj.mols.iter().enumerate() {
        // only show the first molecule on startup
        let visible = fi == 0;
        for (i, a) in mol.atoms() {
            let mut atom = Atom::new(a);
            atom.set_visible(visible);
            let mut atom_bundle = AtomBundle::new(atom, &mut meshes, &mut materials);
            let parent_entity = commands.spawn(atom_bundle).insert(AtomIndex(i)).insert(FrameIndex(fi)).id();

            // create atom labels
            let text = create_label(&asset_server, format!("{i}"), false);
            let child_entity = commands
                .spawn(text)
                .insert(AtomLabel::new(parent_entity))
                .insert(FrameIndex(fi))
                .id();
            // FIXME: add hierarcy will make label invisible
            // commands.entity(parent_entity).add_child(child_entity);
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
        use bevy::app::StartupSet::PostStartup;
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.insert_resource(self.traj.clone())
            .insert_resource(CurrentFrame(0))
            .insert_resource(VisilizationState::default())
            .add_plugin(DebugLinesPlugin::default())
            .add_plugin(PanOrbitCameraPlugin)
            // .add_plugin(WorldInspectorPlugin::new())
            .add_system(update_light_with_camera);

        match self.traj.mols.len() {
            0 => {
                eprintln!("No molecule loaded!");
            }
            1 => {
                app.add_startup_system(spawn_molecules)
                    // .add_system(update_atom_labels)
                    .add_system(update_atom_labels_with_camera);
            }
            _ => {
                app.add_startup_system(spawn_molecules)
                    .add_system(update_atom_labels_with_camera)
                    .add_system(frame_control.after(update_atom_labels_with_camera))
                    // .add_system(create_atom_label)
                    // .add_system(play_animation.in_base_set(PostStartup));
                    .add_system(play_animation.after(update_atom_labels_with_camera));
            }
        }
    }
}
// 8ec82258 ends here
