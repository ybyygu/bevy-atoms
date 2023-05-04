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
mod menu {
    use bevy::prelude::*;
    use bevy_egui::{egui, EguiContexts, EguiPlugin};

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
            .add_plugin(EguiPlugin)
            // .add_system(menu::example_system)
            .add_system(handle_atom_label_events)
            .add_system(update_atom_labels_with_camera);
    }
}
// f9bfb184 ends here
