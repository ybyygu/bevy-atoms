// [[file:../bevy.note::*imports][imports:1]]
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
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            label_atoms_checked: false,
            message: "Tip: You can press `q` to exit.".to_owned(),
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

    text.visibility = crate::player::visibility(visible);
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
    mut label_query: Query<Entity, With<AtomLabel>>,
    mut frame_query: Query<(Entity, &crate::player::FrameIndex, &Visibility), With<crate::player::Atom>>,
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
    fn load_trajectory(
        &mut self,
        traj: ResMut<crate::molecule::MoleculeTrajectory>,
        mut state: ResMut<UiState>,
        mut writer: EventWriter<crate::net::StreamEvent>,
    ) {
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
            use gchemol::io::prelude::*;
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
        mut molecule_query: Query<Entity, With<crate::player::Molecule>>,
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
        mut state: ResMut<UiState>,
        mut label_events: EventWriter<AtomLabelEvent>,
        mut atoms_query: Query<(Entity, &crate::player::AtomIndex, &crate::player::Atom)>,
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

    use crate::player::AtomIndex;
    use crate::ui::AtomLabelEvent;

    use bevy::app::AppExit;
    use bevy::{prelude::*, render::camera::Projection, window::PrimaryWindow};
    use bevy_egui::{egui, EguiContexts, EguiPlugin};

    pub fn side_panels(
        mut state: ResMut<UiState>,
        mut contexts: EguiContexts,
        mut commands: Commands,
        mut molecule_query: Query<Entity, With<crate::player::Molecule>>,
        mut label_events: EventWriter<AtomLabelEvent>,
        mut atoms_query: Query<(Entity, &AtomIndex, &crate::player::Atom)>,
        mut traj: ResMut<crate::molecule::MoleculeTrajectory>,
        mut writer: EventWriter<crate::net::StreamEvent>,
        mut current_frame: ResMut<crate::player::CurrentFrame>,
        mut app_exit_events: ResMut<Events<AppExit>>,
    ) {
        let ctx = contexts.ctx_mut();

        // use light theme
        let mut style = egui::Style::default();
        style.visuals = egui::Visuals::light();
        ctx.set_style(style);

        let mut action = Action::None;
        let mut app = UiApp::default();
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
                        state.message = "no implemented yet".into();
                    }
                    if ui.button("Input files generator…").clicked() {
                        ui.close_menu();
                        // Spawn a second window
                        let second_window_id = commands
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

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

        egui::TopBottomPanel::bottom("bottom_panel").resizable(true).show(ctx, |ui| {
            ui.label(&state.message);
        });

        match action {
            Action::None => {}
            Action::Load => app.load_trajectory(traj, state, writer),
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

// [[file:../bevy.note::50cf0041][50cf0041]]
mod input {
    use super::compute::State;
    use bevy::prelude::*;
    use bevy::window::PrimaryWindow;
    use bevy_egui::{egui, EguiContext};

    pub fn input_generator_window_system(
        mut state: ResMut<super::compute::State>,
        mut egui_ctx: Query<&mut EguiContext, Without<PrimaryWindow>>,
        mut traj: ResMut<crate::molecule::MoleculeTrajectory>,
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
        use bevy_egui::EguiPlugin;

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
