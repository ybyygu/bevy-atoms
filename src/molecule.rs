// [[file:../bevy.note::a83ae206][a83ae206]]
use bevy::prelude::*;
// a83ae206 ends here

// [[file:../bevy.note::031857dd][031857dd]]
#[derive(Component)]
pub struct Atom;

#[derive(Component)]
pub struct Bond;

#[derive(Component, Deref, DerefMut)]
pub struct Position(pub Vec3);
// 031857dd ends here
