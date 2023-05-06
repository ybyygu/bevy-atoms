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
fn create_label_text(asset_server: &Res<AssetServer>, text: impl Into<String>) -> TextBundle {
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

    text.visibility = Visibility::Visible;
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
    Delete(Entity),
}

// Create/hide/show atom labels
fn handle_atom_label_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<AtomLabelEvent>,
    mut label_query: Query<(Entity, &AtomLabel), With<AtomLabel>>,
) {
    for event in events.iter() {
        match event {
            AtomLabelEvent::Create((entity, text)) => {
                debug!("create label for entity {entity:?} with {text:?}");
                let label = create_label_text(&asset_server, text);
                commands.spawn((label, AtomLabel::new(*entity)));
            }
            AtomLabelEvent::Delete(entity) => {
                debug!("delete label for entity {entity:?}");
                for (entity, label) in label_query.iter() {
                    if label.entity == entity {
                        commands.entity(entity).despawn();
                        debug!("label for {entity:?} deleted");
                    }
                }
            }
        }
    }
}
// f1cac934 ends here

// [[file:../bevy.note::bccb8119][bccb8119]]
mod panel {
    use bevy::{prelude::*, render::camera::Projection, window::PrimaryWindow};
    use bevy_egui::{egui, EguiContexts, EguiPlugin};

    #[derive(Default, Resource)]
    pub struct OccupiedScreenSpace {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    }

    const CAMERA_TARGET: Vec3 = Vec3::ZERO;

    #[derive(Resource, Deref, DerefMut)]
    struct OriginalCameraTransform(Transform);

    pub fn side_panels(mut contexts: EguiContexts, mut occupied_screen_space: ResMut<OccupiedScreenSpace>) {
        let ctx = contexts.ctx_mut();

        occupied_screen_space.left = egui::SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Left resizeable panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .width();
        occupied_screen_space.right = egui::SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Right resizeable panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .width();
        occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Top resizeable panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .height();
        occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Bottom resizeable panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .height();
    }

    fn update_camera_transform_system(
        occupied_screen_space: Res<OccupiedScreenSpace>,
        original_camera_transform: Res<OriginalCameraTransform>,
        windows: Query<&Window, With<PrimaryWindow>>,
        mut camera_query: Query<(&Projection, &mut Transform)>,
    ) {
        let (camera_projection, mut transform) = match camera_query.get_single_mut() {
            Ok((Projection::Perspective(projection), transform)) => (projection, transform),
            _ => unreachable!(),
        };

        let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
        let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
        let frustum_width = frustum_height * camera_projection.aspect_ratio;

        let window = windows.single();

        let left_taken = occupied_screen_space.left / window.width();
        let right_taken = occupied_screen_space.right / window.width();
        let top_taken = occupied_screen_space.top / window.height();
        let bottom_taken = occupied_screen_space.bottom / window.height();
        transform.translation = original_camera_transform.translation
            + transform.rotation.mul_vec3(Vec3::new(
                (right_taken - left_taken) * frustum_width * 0.5,
                (top_taken - bottom_taken) * frustum_height * 0.5,
                0.0,
            ));
    }

    pub fn example_system(mut contexts: EguiContexts) {
        egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
            ui.label("world");
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
            .init_resource::<panel::OccupiedScreenSpace>()
            .add_system(panel::side_panels)
            .add_system(handle_atom_label_events)
            .add_system(update_atom_labels_with_camera);
    }
}
// f9bfb184 ends here
