use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_plugin(LookTransformPlugin)
            .add_plugin(OrbitCameraPlugin::default());
    }
}

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 4,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // atom
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        })
        .insert(Player)
        .insert(PickableBundle::default());

    // bond
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder::default())),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

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
                mouse_wheel_zoom_sensitivity: 0.005,
                smoothing_weight: 0.02,
                ..Default::default()
            },
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ))
        .insert(PickingCameraBundle::default());
}
