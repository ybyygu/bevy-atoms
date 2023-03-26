// [[file:../bevy.note::a83ae206][a83ae206]]
use bevy::prelude::*;

use bevy_mod_picking::{PickableBundle, PickingCameraBundle};
// a83ae206 ends here

// [[file:../bevy.note::031857dd][031857dd]]
#[derive(Clone, Copy, Debug, Component)]
pub struct Atom {
    element: usize,
}

#[derive(Clone, Copy, Debug, Component)]
pub struct Bond;

/// 定义原子或化学键的位置
#[derive(Component, Clone, Copy, Debug, Deref, DerefMut)]
pub struct Position(Vec3);

/// 定义原子或化学键的编号
#[derive(Clone, Copy, Debug, Component)]
pub struct Index(pub u64);

#[derive(Clone, Debug, Component)]
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
// 031857dd ends here

// [[file:../bevy.note::deffe145][deffe145]]
/// Move atoms according to their positions
pub fn move_atoms(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut query {
        transform.translation = position.0;
    }
}

pub fn setup_atoms(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        })
        // .insert(Atom)
        .insert(Position(Vec3::new(0.0, 0.0, 4.0)))
        .insert(PickableBundle::default());
}
// deffe145 ends here
