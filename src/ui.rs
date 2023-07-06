// [[file:../bevy.note::*imports][imports:1]]
#![deny(warnings)]
#![deny(clippy::all)]
#![allow(non_camel_case_types)]

use bevy::prelude::*;
// imports:1 ends here

// [[file:../bevy.note::45bd6a9d][45bd6a9d]]
macro_rules! enum_value {
    ($v:expr) => {{
        serde_json::to_string($v).unwrap().trim_matches('"').to_string()
    }};
}

macro_rules! show_combo_box_enum {
    ($id:literal, $ui:ident, $var:expr, $type:ty, $width:literal) => {
        let s = enum_value!(&$var);
        egui::ComboBox::from_id_source($id)
            .width($width)
            .selected_text(s)
            .show_ui($ui, |ui| {
                for t in enum_iterator::all::<$type>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut $var, t.into(), s);
                }
            });
    };
}
// 45bd6a9d ends here

// [[file:../bevy.note::8d1285a1][8d1285a1]]
mod compute;
mod cp2k;
mod gaussian;
mod orca;
mod template;
mod vasp;
// 8d1285a1 ends here

// [[file:../bevy.note::02f2343f][02f2343f]]
/// Text label attached to an Atom
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

    #[allow(dead_code)]
    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }
}
// 02f2343f ends here

// [[file:../bevy.note::13082bcf][13082bcf]]
#[derive(Debug, Resource)]
pub struct UiState {
    label_atoms_checked: bool,
    message: String,
    periodic_table_window_open: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            label_atoms_checked: false,
            message: "Tip: You can press `q` to exit.".to_owned(),
            periodic_table_window_open: false,
        }
    }
}
// 13082bcf ends here

// [[file:../bevy.note::31ecf2a0][31ecf2a0]]
/// possible ui actions
#[derive(Debug, Clone)]
enum Action {
    /// Nothing to do
    None,
    /// Load trajectory from file
    Load,
    /// Save trajectory to file
    Save,
    /// Clear loaded molecules
    Clear,
    /// Create label for each atom
    LabelAtoms,
    /// Remove lattice
    UnbuildCrystal,
}

#[derive(Debug, Default, Clone)]
struct UiApp {}
// 31ecf2a0 ends here

// [[file:../bevy.note::4c72e4a9][4c72e4a9]]
fn create_label_text(asset_server: &Res<AssetServer>, text: impl Into<String>, visible: bool) -> TextBundle {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let style = Style {
        position_type: PositionType::Absolute,
        position: UiRect { ..Default::default() },
        ..Default::default()
    };

    let mut text = TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 14.0,
            ..default()
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(style);

    text.visibility = crate::base::visibility(visible);
    text
}

/// Update atom label position by projecting 3D atom position to 2D
/// screen
fn update_atom_labels_with_camera(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut label_style_query: Query<(&AtomLabel, &mut Style, &CalculatedSize, &ComputedVisibility)>,
    transform_query: Query<&Transform>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
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
}
// 4c72e4a9 ends here

// [[file:../bevy.note::f1cac934][f1cac934]]
/// Atom label related event
pub enum AtomLabelEvent {
    Create((Entity, String)),
    Delete,
}

// Create/hide/show atom labels
fn handle_atom_label_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<AtomLabelEvent>,
    label_query: Query<Entity, With<AtomLabel>>,
    frame_query: Query<(Entity, &crate::base::FrameIndex, &Visibility), With<crate::base::Atom>>,
) {
    for event in events.iter() {
        match event {
            AtomLabelEvent::Create((entity, text)) => {
                debug!("create label for entity {entity:?} with {text:?}");
                // NOTE: visibility hierarchy not work here
                // let child = commands.spawn((label, AtomLabel::new(*entity))).id();
                // commands.entity(*entity).add_child(child);
                let (_, iframe, vis) = frame_query.iter().find(|part| part.0 == *entity).unwrap();
                if vis != Visibility::Hidden {
                    let label = create_label_text(&asset_server, text, true);
                    commands.spawn((label, AtomLabel::new(*entity))).insert(*iframe);
                }
            }
            AtomLabelEvent::Delete => {
                debug!("delete label ...");
                for entity in label_query.iter() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
// f1cac934 ends here

// [[file:../bevy.note::3fa34d4c][3fa34d4c]]
impl UiApp {
    fn load_trajectory(&mut self, mut state: ResMut<UiState>, mut writer: EventWriter<crate::net::StreamEvent>) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("auto detect", &["*"])
            .add_filter("*.xyz", &["xyz", "pxyz"])
            .add_filter("*.mol2", &["mol2"])
            .add_filter("*.pdf", &["pdb", "ent"])
            .add_filter("*.mol", &["mol", "sdf"])
            .add_filter("*.cif", &["cif"])
            .add_filter("*.xsd", &["xsd"])
            .add_filter("*.cjson", &["cjson"])
            .add_filter("Gaussian (*.com, *.gjf)", &["com", "gjf"])
            .add_filter("VASP (*.vasp)", &["vasp"])
            .pick_file()
        {
            if let Ok(mols) = gchemol::io::read(path) {
                let mols: Vec<_> = mols
                    // create bonds if necessary
                    .map(|mut m| {
                        if m.nbonds() == 0 {
                            let lat = m.unbuild_crystal();
                            m.rebond();
                            m.lattice = lat;
                            info!("bonds created.");
                        }
                        m
                    })
                    .collect();
                let n = mols.len();
                let command = crate::net::RemoteCommand::Load(mols);
                writer.send(crate::net::StreamEvent(command));
                state.message = format!("{n} Molecules loaded.");
            }
        }
    }

    fn save_trajectory(&mut self, traj: ResMut<crate::molecule::MoleculeTrajectory>, mut state: ResMut<UiState>) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            traj.save_as(path.as_ref());
            state.message = format!("Molecules saved to {path:?}");
        }
    }
}
// 3fa34d4c ends here

// [[file:../bevy.note::e26673e2][e26673e2]]
impl UiApp {
    fn clear_molecules(
        &mut self,
        mut commands: Commands,
        mut state: ResMut<UiState>,
        mut label_events: EventWriter<AtomLabelEvent>,
        molecule_query: Query<Entity, With<crate::base::Molecule>>,
    ) {
        if let Ok(molecule_entity) = molecule_query.get_single() {
            info!("remove molecule");
            commands.entity(molecule_entity).despawn_recursive();
            // also remove atom labels
            label_events.send(AtomLabelEvent::Delete);
        } else {
            state.message = "No molecule present".into();
        }
    }
}
// e26673e2 ends here

// [[file:../bevy.note::ed37221a][ed37221a]]
impl UiApp {
    fn label_atoms(
        &mut self,
        state: ResMut<UiState>,
        mut label_events: EventWriter<AtomLabelEvent>,
        atoms_query: Query<(Entity, &crate::base::AtomIndex, &crate::base::Atom)>,
    ) {
        if state.label_atoms_checked {
            info!("create atoms labels ...");
            for (entity, atom_index, atom) in atoms_query.iter() {
                let label = atom.get_label(atom_index.0);
                if !label.is_empty() {
                    label_events.send(AtomLabelEvent::Create((entity, label)));
                }
            }
        } else {
            info!("delete atoms labels ...");
            label_events.send(AtomLabelEvent::Delete);
        }
    }
}
// ed37221a ends here

// [[file:../bevy.note::bccb8119][bccb8119]]
mod panel {
    use super::{Action, UiApp, UiState};

    use crate::base::AtomIndex;
    use crate::ui::AtomLabelEvent;

    use bevy::app::AppExit;
    use bevy::prelude::*;
    use bevy_egui::{egui, EguiContexts};

    pub fn side_panels(
        mut state: ResMut<UiState>,
        mut contexts: EguiContexts,
        mut commands: Commands,
        molecule_query: Query<Entity, With<crate::base::Molecule>>,
        label_events: EventWriter<AtomLabelEvent>,
        atoms_query: Query<(Entity, &AtomIndex, &crate::base::Atom)>,
        traj: ResMut<crate::molecule::MoleculeTrajectory>,
        writer: EventWriter<crate::net::StreamEvent>,
        mut current_frame: ResMut<crate::base::CurrentFrame>,
        mut app_exit_events: ResMut<Events<AppExit>>,
        selected_atoms: Res<crate::molecule::SelectedAtoms>,
    ) {
        let ctx = contexts.ctx_mut();

        // use light theme
        let mut style = egui::Style::default();
        style.visuals = egui::Visuals::light();
        ctx.set_style(style);

        let mut action = Action::None;
        let mut app = UiApp::default();

        // ui for periodic table
        egui::Window::new("Periodic Table")
            .id(egui::Id::new("periodic_table"))
            // will be activated by menu item: tools/periodic table
            .open(&mut state.periodic_table_window_open)
            .anchor(egui::Align2::CENTER_TOP, [0.0, 0.0])
            .collapsible(false)
            .default_width(500.0)
            .show(ctx, super::periodic_table::show);

        egui::TopBottomPanel::top("top_panel").resizable(true).show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("🗁 Load …").on_hover_text("Load molecules from file").clicked() {
                        action = Action::Load;
                        ui.close_menu();
                    }
                    if ui.button("💾 Save…").clicked() {
                        action = Action::Save;
                        ui.close_menu();
                    }
                    if ui.button("✖ Quit").clicked() {
                        app_exit_events.send(AppExit);
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("rebond").clicked() {
                        // …
                    }
                    // Remove all molecules
                    if ui.button("Clear Molecule").clicked() {
                        action = Action::Clear;
                        ui.close_menu();
                    }
                });
                ui.menu_button("View", |ui| {
                    // Put molecule in the center of view
                    if ui.button("recenter").clicked() {
                        state.message = "no implemented yet".into();
                    }
                });
                ui.menu_button("Select", |ui| {
                    if ui.button("Select all").clicked() {
                        state.message = "no implemented yet".into();
                    }
                    if ui.button("Select none").clicked() {
                        state.message = "no implemented yet".into();
                    }
                    if ui.button("show selected").clicked() {
                        if !selected_atoms.0.is_empty() {
                            if let Ok(s) = gut::utils::abbreviate_numbers_human_readable(&selected_atoms.0) {
                                state.message = format!("selected atoms: {s}");
                            }
                        }
                    }
                });
                ui.menu_button("Crystal", |ui| {
                    if ui.button("Unbuild crystal").clicked() {
                        action = Action::UnbuildCrystal;
                    }
                    if ui.button("Edit unit cell…").clicked() {
                        state.message = "no implemented yet".into();
                    }
                    if ui.button("Wrap atoms to unit cell").clicked() {
                        state.message = "no implemented yet".into();
                    }
                    if ui.button("Build supercell…").clicked() {
                        state.message = "no implemented yet".into();
                    }
                });

                ui.menu_button("Tools", |ui| {
                    if ui.button("Periodic table…").clicked() {
                        state.periodic_table_window_open = true;
                        ui.close_menu();
                    }
                    if ui.button("Input files generator…").clicked() {
                        ui.close_menu();
                        // Spawn a second window
                        let _second_window_id = commands
                            .spawn(Window {
                                title: "Input files generator".to_owned(),
                                present_mode: bevy::window::PresentMode::AutoVsync,
                                ..Default::default()
                            })
                            .id();
                    }
                });
                ui.menu_button("Task", |ui| {
                    if ui.button("Geometry Optimization").clicked() {
                        state.message = "no implemented yet".into();
                    }
                    if ui.button("Phonon").clicked() {
                        state.message = "no implemented yet".into();
                    }
                });
            });
        });

        egui::SidePanel::left("left_panel").resizable(true).show(ctx, |ui| {
            ui.label("Available actions");
            ui.separator();
            // label atoms by serial numbers
            if ui.checkbox(&mut state.label_atoms_checked, "Label atoms").clicked() {
                action = Action::LabelAtoms;
            }
            // show animation control button
            let nframes = traj.mols.len();
            if let Some(iframe) = current_frame.index(nframes) {
                ui.horizontal(|ui| {
                    if ui.button("Backward").clicked() {
                        current_frame.prev();
                        state.message = format!("Frame {iframe}");
                    }
                    if ui.button("Forward").clicked() {
                        current_frame.next();
                        state.message = format!("Frame {iframe}");
                    }
                });
            }

            // show ui for molecule control
            if !traj.mols.is_empty() {
                super::molecule_traj::show(ui, &traj.mols, &mut current_frame);
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

        egui::TopBottomPanel::bottom("bottom_panel").resizable(true).show(ctx, |ui| {
            ui.label(&state.message);
        });

        match action {
            Action::None => {}
            Action::Load => app.load_trajectory(state, writer),
            Action::Save => app.save_trajectory(traj, state),
            Action::Clear => app.clear_molecules(commands, state, label_events, molecule_query),
            Action::LabelAtoms => app.label_atoms(state, label_events, atoms_query),
            _ => {
                state.message = format!("handler for action {action:?} is not implemented yet");
            }
        }
    }
}
// bccb8119 ends here

// [[file:../bevy.note::a06e5732][a06e5732]]
mod molecule_traj {
    use bevy_egui::egui;
    use egui::Ui;
    use gchemol::Molecule;

    pub fn show(ui: &mut Ui, traj: &[Molecule], current_frame: &mut crate::base::CurrentFrame) {
        egui::CollapsingHeader::new("Molecule list").default_open(true).show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    for (i, mol) in traj.iter().enumerate() {
                        let _ = ui.selectable_value(&mut current_frame.0, i as isize, mol.title());
                    }
                })
            });
        });
    }
}
// a06e5732 ends here

// [[file:../bevy.note::c153256c][c153256c]]
mod periodic_table {
    use bevy_egui::egui;
    use egui::{Button, Color32, Ui};
    use gchemol::Atom;

    // https://github.com/eliaxelang007/Periodic-Table-Rs/blob/master/_data_formatting/stage_2/_out.json
    fn electronic_configuration(symbol: &str) -> String {
        match symbol {
            "H" => format!("1s¹"),
            "He" => format!("1s²"),
            "Li" => format!("[He] 2s¹"),
            "Be" => format!("[He] 2s²"),
            "B" => format!("[He] 2s² 2p¹"),
            "C" => format!("[He] 2s² 2p²"),
            "N" => format!("[He] 2s² 2p³"),
            "O" => format!("[He] 2s² 2p⁴"),
            "F" => format!("[He] 2s² 2p⁵"),
            "Ne" => format!("[He] 2s² 2p⁶"),
            "Na" => format!("[Ne] 3s¹"),
            "Mg" => format!("[Ne] 3s²"),
            "Al" => format!("[Ne] 3s² 3p¹"),
            "Si" => format!("[Ne] 3s² 3p²"),
            "P" => format!("[Ne] 3s² 3p³"),
            "S" => format!("[Ne] 3s² 3p⁴"),
            "Cl" => format!("[Ne] 3s² 3p⁵"),
            "Ar" => format!("[Ne] 3s² 3p⁶"),
            "K" => format!("[Ar] 4s¹"),
            "Ca" => format!("[Ar] 4s²"),
            "Sc" => format!("[Ar] 4s² 3d¹"),
            "Ti" => format!("[Ar] 4s² 3d²"),
            "V" => format!("[Ar] 4s² 3d³"),
            "Cr" => format!("[Ar] 3d⁵ 4s¹"),
            "Mn" => format!("[Ar] 4s² 3d⁵"),
            "Fe" => format!("[Ar] 4s² 3d⁶"),
            "Co" => format!("[Ar] 4s² 3d⁷"),
            "Ni" => format!("[Ar] 4s² 3d⁸"),
            "Cu" => format!("[Ar] 4s¹ 3d¹⁰"),
            "Zn" => format!("[Ar] 4s² 3d¹⁰"),
            "Ga" => format!("[Ar] 4s² 3d¹⁰ 4p¹"),
            "Ge" => format!("[Ar] 4s² 3d¹⁰ 4p²"),
            "As" => format!("[Ar] 4s² 3d¹⁰ 4p³"),
            "Se" => format!("[Ar] 4s² 3d¹⁰ 4p⁴"),
            "Br" => format!("[Ar] 4s² 3d¹⁰ 4p⁵"),
            "Kr" => format!("[Ar] 4s² 3d¹⁰ 4p⁶"),
            "Rb" => format!("[Kr] 5s¹"),
            "Sr" => format!("[Kr] 5s²"),
            "Y" => format!("[Kr] 5s² 4d¹"),
            "Zr" => format!("[Kr] 5s² 4d²"),
            "Nb" => format!("[Kr] 5s¹ 4d⁴"),
            "Mo" => format!("[Kr] 5s¹ 4d⁵"),
            "Tc" => format!("[Kr] 5s² 4d⁵"),
            "Ru" => format!("[Kr] 5s¹ 4d⁷"),
            "Rh" => format!("[Kr] 5s¹ 4d⁸"),
            "Pd" => format!("[Kr] 4d¹⁰"),
            "Ag" => format!("[Kr] 5s¹ 4d¹⁰"),
            "Cd" => format!("[Kr] 5s² 4d¹⁰"),
            "In" => format!("[Kr] 5s² 4d¹⁰ 5p¹"),
            "Sn" => format!("[Kr] 5s² 4d¹⁰ 5p²"),
            "Sb" => format!("[Kr] 5s² 4d¹⁰ 5p³"),
            "Te" => format!("[Kr] 5s² 4d¹⁰ 5p⁴"),
            "I" => format!("[Kr] 5s² 4d¹⁰ 5p⁵"),
            "Xe" => format!("[Kr] 5s² 4d¹⁰ 5p⁶"),
            "Cs" => format!("[Xe] 6s¹"),
            "Ba" => format!("[Xe] 6s²"),
            "La" => format!("[Xe] 6s² 5d¹"),
            "Ce" => format!("[Xe] 6s² 4f¹ 5d¹"),
            "Pr" => format!("[Xe] 6s² 4f³"),
            "Nd" => format!("[Xe] 6s² 4f⁴"),
            "Pm" => format!("[Xe] 6s² 4f⁵"),
            "Sm" => format!("[Xe] 6s² 4f⁶"),
            "Eu" => format!("[Xe] 6s² 4f⁷"),
            "Gd" => format!("[Xe] 6s² 4f⁷ 5d¹"),
            "Tb" => format!("[Xe] 6s² 4f⁹"),
            "Dy" => format!("[Xe] 6s² 4f¹⁰"),
            "Ho" => format!("[Xe] 6s² 4f¹¹"),
            "Er" => format!("[Xe] 6s² 4f¹²"),
            "Tm" => format!("[Xe] 6s² 4f¹³"),
            "Yb" => format!("[Xe] 6s² 4f¹⁴"),
            "Lu" => format!("[Xe] 6s² 4f¹⁴ 5d¹"),
            "Hf" => format!("[Xe] 6s² 4f¹⁴ 5d²"),
            "Ta" => format!("[Xe] 6s² 4f¹⁴ 5d³"),
            "W" => format!("[Xe] 6s² 4f¹⁴ 5d⁴"),
            "Re" => format!("[Xe] 6s² 4f¹⁴ 5d⁵"),
            "Os" => format!("[Xe] 6s² 4f¹⁴ 5d⁶"),
            "Ir" => format!("[Xe] 6s² 4f¹⁴ 5d⁷"),
            "Pt" => format!("[Xe] 6s¹ 4f¹⁴ 5d⁹"),
            "Au" => format!("[Xe] 6s¹ 4f¹⁴ 5d¹⁰"),
            "Hg" => format!("[Xe] 6s² 4f¹⁴ 5d¹⁰"),
            "Tl" => format!("[Xe] 6s² 4f¹⁴ 5d¹⁰ 6p¹"),
            "Pb" => format!("[Xe] 6s² 4f¹⁴ 5d¹⁰ 6p²"),
            "Bi" => format!("[Xe] 6s² 4f¹⁴ 5d¹⁰ 6p³"),
            "Po" => format!("[Xe] 6s² 4f¹⁴ 5d¹⁰ 6p⁴"),
            "At" => format!("[Xe] 6s² 4f¹⁴ 5d¹⁰ 6p⁵"),
            "Rn" => format!("[Xe] 6s² 4f¹⁴ 5d¹⁰ 6p⁶"),
            "Fr" => format!("[Rn] 7s¹"),
            "Ra" => format!("[Rn] 7s²"),
            "Ac" => format!("[Rn] 7s² 6d¹"),
            "Th" => format!("[Rn] 7s² 6d²"),
            "Pa" => format!("[Rn] 7s² 5f² 6d¹"),
            "U" => format!("[Rn] 7s² 5f³ 6d¹"),
            "Np" => format!("[Rn] 7s² 5f⁴ 6d¹"),
            "Pu" => format!("[Rn] 7s² 5f⁶"),
            "Am" => format!("[Rn] 7s² 5f⁷"),
            "Cm" => format!("[Rn] 7s² 5f⁷ 6d¹"),
            "Bk" => format!("[Rn] 7s² 5f⁹"),
            "Cf" => format!("[Rn] 7s² 5f¹⁰"),
            "Es" => format!("[Rn] 7s² 5f¹¹"),
            "Fm" => format!("[Rn] 5f¹² 7s²"),
            "Md" => format!("[Rn] 7s² 5f¹³"),
            "No" => format!("[Rn] 7s² 5f¹⁴"),
            "Lr" => format!("[Rn] 7s² 5f¹⁴ 6d¹"),
            "Rf" => format!("[Rn] 7s² 5f¹⁴ 6d²"),
            "Db" => format!("[Rn] 7s² 5f¹⁴ 6d³"),
            "Sg" => format!("[Rn] 7s² 5f¹⁴ 6d⁴"),
            "Bh" => format!("[Rn] 7s² 5f¹⁴ 6d⁵"),
            "Hs" => format!("[Rn] 7s² 5f¹⁴ 6d⁶"),
            "Mt" => format!("[Rn] 7s² 5f¹⁴ 6d⁷ (calculated)"),
            "Ds" => format!("[Rn] 7s² 5f¹⁴ 6d⁸ (predicted)"),
            "Rg" => format!("[Rn] 7s² 5f¹⁴ 6d⁹ (predicted)"),
            "Cn" => format!("[Rn] 7s² 5f¹⁴ 6d¹⁰ (predicted)"),
            "Nh" => format!("[Rn] 5f¹⁴ 6d¹⁰ 7s² 7p¹ (predicted)"),
            "Fl" => format!("[Rn] 7s² 7p² 5f¹⁴ 6d¹⁰ (predicted)"),
            "Mc" => format!("[Rn] 7s² 7p³ 5f¹⁴ 6d¹⁰ (predicted)"),
            "Lv" => format!("[Rn] 7s² 7p⁴ 5f¹⁴ 6d¹⁰ (predicted)"),
            "Ts" => format!("[Rn] 7s² 7p⁵ 5f¹⁴ 6d¹⁰ (predicted)"),
            "Og" => format!("[Rn] 7s² 7p⁶ 5f¹⁴ 6d¹⁰ (predicted)"),
            _ => format!("todo"),
        }
    }

    fn new_element_button(ui: &mut Ui, symbol: &str, color: Color32) {
        let atom = Atom::new(symbol, [0.0; 3]);
        let name = atom.kind().name();
        let number = format!("{}", atom.number());
        let cov_radius = format!("{}", atom.get_cov_radius().map(|x| x.to_string()).unwrap_or("N/A".into()));
        let vdw_radius = format!("{}", atom.get_vdw_radius().map(|x| x.to_string()).unwrap_or("N/A".into()));
        let symbol_txt = format!("{symbol:2}");
        ui.add(Button::new(symbol_txt).fill(color)).on_hover_ui(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.label(name);
            });
            ui.horizontal(|ui| {
                ui.label("Number:");
                ui.label(number);
            });
            ui.horizontal(|ui| {
                ui.label("Covalent radius:");
                ui.label(cov_radius);
            });
            ui.horizontal(|ui| {
                ui.label("Van der Waals radius:");
                ui.label(vdw_radius);
            });
            ui.horizontal(|ui| {
                ui.label("Electronic configuration:");
                ui.label(electronic_configuration(symbol));
            });
        });
    }

    pub fn show(ui: &mut Ui) {
        egui::Grid::new("grid_periodic_table")
            .striped(true)
            .num_columns(18)
            .min_col_width(15.0)
            .show(ui, |ui| {
                new_element_button(ui, "H", Color32::WHITE);
                // void space
                for _ in 0..16 {
                    ui.label("");
                }
                new_element_button(ui, "He", Color32::WHITE);
                ui.end_row();
                // 2
                new_element_button(ui, "Li", Color32::WHITE);
                new_element_button(ui, "Be", Color32::WHITE);
                // void space
                for _ in 0..10 {
                    ui.label("");
                }
                new_element_button(ui, "B", Color32::WHITE);
                new_element_button(ui, "C", Color32::WHITE);
                new_element_button(ui, "N", Color32::WHITE);
                new_element_button(ui, "O", Color32::WHITE);
                new_element_button(ui, "F", Color32::WHITE);
                new_element_button(ui, "Ne", Color32::WHITE);
                ui.end_row();
                // 3
                new_element_button(ui, "Na", Color32::WHITE);
                new_element_button(ui, "Mg", Color32::WHITE);
                // void space
                for _ in 0..10 {
                    ui.label("");
                }
                new_element_button(ui, "Al", Color32::WHITE);
                new_element_button(ui, "Si", Color32::WHITE);
                new_element_button(ui, "P", Color32::WHITE);
                new_element_button(ui, "S", Color32::WHITE);
                new_element_button(ui, "Cl", Color32::WHITE);
                new_element_button(ui, "Ar", Color32::WHITE);
                ui.end_row();
                new_element_button(ui, "K", Color32::WHITE);
                new_element_button(ui, "Ca", Color32::WHITE);
                new_element_button(ui, "Sc", Color32::WHITE);
                new_element_button(ui, "Ti", Color32::WHITE);
                new_element_button(ui, "V", Color32::WHITE);
                new_element_button(ui, "Cr", Color32::WHITE);
                new_element_button(ui, "Mn", Color32::WHITE);
                new_element_button(ui, "Fe", Color32::WHITE);
                new_element_button(ui, "Co", Color32::WHITE);
                new_element_button(ui, "Ni", Color32::WHITE);
                new_element_button(ui, "Cu", Color32::WHITE);
                new_element_button(ui, "Zn", Color32::WHITE);
                new_element_button(ui, "Ga", Color32::WHITE);
                new_element_button(ui, "Ge", Color32::WHITE);
                new_element_button(ui, "As", Color32::WHITE);
                new_element_button(ui, "Se", Color32::WHITE);
                new_element_button(ui, "Br", Color32::WHITE);
                new_element_button(ui, "Kr", Color32::WHITE);
                ui.end_row();
                new_element_button(ui, "Rb", Color32::WHITE);
                new_element_button(ui, "Sr", Color32::WHITE);
                new_element_button(ui, "Y", Color32::WHITE);
                new_element_button(ui, "Zr", Color32::WHITE);
                new_element_button(ui, "Nb", Color32::WHITE);
                new_element_button(ui, "Mo", Color32::WHITE);
                new_element_button(ui, "Tc", Color32::WHITE);
                new_element_button(ui, "Ru", Color32::WHITE);
                new_element_button(ui, "Rh", Color32::WHITE);
                new_element_button(ui, "Pd", Color32::WHITE);
                new_element_button(ui, "Ag", Color32::WHITE);
                new_element_button(ui, "Cd", Color32::WHITE);
                new_element_button(ui, "In", Color32::WHITE);
                new_element_button(ui, "Sn", Color32::WHITE);
                new_element_button(ui, "Sb", Color32::WHITE);
                new_element_button(ui, "Te", Color32::WHITE);
                new_element_button(ui, "I", Color32::WHITE);
                new_element_button(ui, "Xe", Color32::WHITE);
                ui.end_row();
                new_element_button(ui, "Cs", Color32::WHITE);
                new_element_button(ui, "Ba", Color32::WHITE);
                new_element_button(ui, "La", Color32::WHITE);
                new_element_button(ui, "Hf", Color32::WHITE);
                new_element_button(ui, "Ta", Color32::WHITE);
                new_element_button(ui, "W", Color32::WHITE);
                new_element_button(ui, "Re", Color32::WHITE);
                new_element_button(ui, "Os", Color32::WHITE);
                new_element_button(ui, "Ir", Color32::WHITE);
                new_element_button(ui, "Pt", Color32::WHITE);
                new_element_button(ui, "Au", Color32::WHITE);
                new_element_button(ui, "Hg", Color32::WHITE);
                new_element_button(ui, "Tl", Color32::WHITE);
                new_element_button(ui, "Pb", Color32::WHITE);
                new_element_button(ui, "Bi", Color32::WHITE);
                new_element_button(ui, "Po", Color32::WHITE);
                new_element_button(ui, "At", Color32::WHITE);
                new_element_button(ui, "Rn", Color32::WHITE);
            });
    }
}
// c153256c ends here

// [[file:../bevy.note::50cf0041][50cf0041]]
mod input {
    use bevy::prelude::*;
    use bevy::window::PrimaryWindow;
    use bevy_egui::{egui, EguiContext};

    pub fn input_generator_window_system(
        mut state: ResMut<super::compute::State>,
        mut egui_ctx: Query<&mut EguiContext, Without<PrimaryWindow>>,
        traj: ResMut<crate::molecule::MoleculeTrajectory>,
    ) {
        let Ok(mut ctx) = egui_ctx.get_single_mut() else { return; };
        let ctx = ctx.get_mut();
        // Switch to light mode
        ctx.set_visuals(egui::Visuals::light());
        // `Molecule` is required for input file generator
        // FIXME: select which molecule to render?
        let mol = traj.mols.iter().last().cloned();
        state.show(ctx, mol);
    }
}
// 50cf0041 ends here

// [[file:../bevy.note::f9bfb184][f9bfb184]]
#[derive(Debug, Clone, Default)]
pub struct LabelPlugin {
    //
}

impl Plugin for LabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AtomLabelEvent>()
            .init_resource::<UiState>()
            .init_resource::<compute::State>()
            .add_system(panel::side_panels)
            .add_system(input::input_generator_window_system)
            .add_system(handle_atom_label_events)
            .add_system(update_atom_labels_with_camera);
    }
}
// f9bfb184 ends here
