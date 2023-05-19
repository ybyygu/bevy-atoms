// [[file:../bevy.note::*imports][imports:1]]
use bevy::prelude::*;
// imports:1 ends here

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
            font: font.clone(),
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
    windows: Query<&Window>,
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

// [[file:../bevy.note::bccb8119][bccb8119]]
mod panel {
    use crate::player::AtomIndex;
    use crate::ui::AtomLabelEvent;

    use bevy::{prelude::*, render::camera::Projection, window::PrimaryWindow};
    use bevy_egui::{egui, EguiContexts, EguiPlugin};

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
    ) {
        let ctx = contexts.ctx_mut();

        // use light theme
        let mut style = egui::Style::default();
        style.visuals = egui::Visuals::light();
        ctx.set_style(style);

        egui::SidePanel::left("left_panel").resizable(true).show(ctx, |ui| {
            ui.label("Available operations:");
            ui.separator();
            // open file dialog
            ui.horizontal(|ui| {
                if ui.button("Load …").clicked() {
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
                // save file dialog
                if ui.button("Save …").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        traj.save_as(path.as_ref());
                        state.message = format!("Molecules saved to {path:?}");
                    }
                }
            });
            // label atoms by serial numbers
            if ui.checkbox(&mut state.label_atoms_checked, "Label atoms").clicked() {
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
            // Remove all molecules
            if ui.button("Clear Molecule").clicked() {
                if let Ok(molecule_entity) = molecule_query.get_single() {
                    info!("remove molecule");
                    commands.entity(molecule_entity).despawn_recursive();
                    // also remove atom labels
                    label_events.send(AtomLabelEvent::Delete);
                } else {
                    state.message = "No molecule present".into();
                }
            }
            // Put molecule in the center of view
            if ui.button("Recenter Molecule").clicked() {
                state.message = "no implemented yet".into();
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

        egui::TopBottomPanel::top("top_panel").resizable(true).show(ctx, |ui| {
            ui.horizontal(|ui| {
                let nframes = traj.mols.len();
                if let Some(iframe) = current_frame.index(nframes) {
                    if ui.button("Backward").clicked() {
                        current_frame.prev();
                        state.message = format!("Frame {iframe}");
                    }
                    if ui.button("Forward").clicked() {
                        current_frame.next();
                        state.message = format!("Frame {iframe}");
                    }
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").resizable(true).show(ctx, |ui| {
            ui.label(&state.message);
        });
    }
}
// bccb8119 ends here

// [[file:../bevy.note::f9bfb184][f9bfb184]]
#[derive(Debug, Clone, Default)]
pub struct LabelPlugin {
    //
}

impl Plugin for LabelPlugin {
    fn build(&self, app: &mut App) {
        use bevy_egui::EguiPlugin;

        app.add_event::<AtomLabelEvent>()
            .init_resource::<panel::UiState>()
            .add_system(panel::side_panels)
            .add_system(handle_atom_label_events)
            .add_system(update_atom_labels_with_camera);
    }
}
// f9bfb184 ends here
