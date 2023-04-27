// [[file:../bevy.note::495e3d25][495e3d25]]
use bevy::prelude::*;
// 495e3d25 ends here

// [[file:../bevy.note::bd70c1ac][bd70c1ac]]
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    runtime::Runtime,
    sync::mpsc::{channel, Receiver, Sender},
};

#[derive(Resource)]
pub struct NetworkServer {
    //
}
// bd70c1ac ends here

// [[file:../bevy.note::0e0418a7][0e0418a7]]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        //
    }
}
// 0e0418a7 ends here
