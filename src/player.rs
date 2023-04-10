use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // atom
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        })
        // .insert(crate::molecule::Atom)
        // .insert(crate::molecule::Position(Vec3::new(0.0, 0.0, 4.0)))
        .insert(PickableBundle::default());

    // bond
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder::default())),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
    // .insert(crate::molecule::Bond)
    // .insert(crate::molecule::Position(Vec3::new(0.0, 0.0, 4.0)));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });

    commands
        .spawn(Camera3dBundle::default())
        .insert(PickingCameraBundle::default());
}
