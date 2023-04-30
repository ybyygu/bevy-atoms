// [[file:../bevy.note::495e3d25][495e3d25]]
use bevy::prelude::*;
use gut::prelude::Result;

use gchemol_core::Molecule;
// 495e3d25 ends here

// [[file:../bevy.note::02dd4467][02dd4467]]
use std::net::{SocketAddr, ToSocketAddrs};

/// Settings to configure the network, both client and server
#[derive(Resource, Default)]
struct NetworkSettings {
    address: Option<SocketAddr>,
}

#[derive(Resource)]
struct NetworkServer {}

impl NetworkServer {
    pub(crate) fn new() -> Self {
        todo!();
    }
}
// 02dd4467 ends here

// [[file:../bevy.note::bbb42a57][bbb42a57]]
mod app_error {
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use gut::prelude::Error;

    // Make our own error that wraps `anyhow::Error`.
    pub struct AppError(Error);

    impl<E> From<E> for AppError
    where
        E: Into<Error>,
    {
        fn from(err: E) -> Self {
            Self(err.into())
        }
    }

    // Tell axum how to convert `AppError` into a response.
    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {}", self.0)).into_response()
        }
    }
}
// bbb42a57 ends here

// [[file:../bevy.note::27477258][27477258]]
mod routes {
    use super::app_error::AppError;
    use gchemol_core::Molecule;
    use gut::prelude::Result;

    use axum::extract::State;
    use axum::Json;

    pub type TaskChannelTx = crate::task::TaskSender<Molecule, ()>;

    #[axum::debug_handler]
    async fn view_molecule(State(client): State<TaskChannelTx>, Json(mol): Json<Molecule>) -> Result<(), AppError> {
        let resp = client.send(mol).await?;
        Ok(())
    }

    /// Start remote view service listening on molecules from remote client side.
    pub async fn serve_remote_view(task_tx: TaskChannelTx) -> Result<()> {
        use axum::{routing::get, Router};

        use axum::routing::post;
        use gchemol_core::Molecule;

        let app = Router::new().route("/view-molecule", post(view_molecule)).with_state(task_tx);
        axum::Server::bind(&"127.0.0.1:3039".parse().unwrap())
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
// 27477258 ends here

// [[file:../bevy.note::d1858fa3][d1858fa3]]
use bevy::tasks::{IoTaskPool, Task};
use futures_lite::future;

#[derive(Resource)]
struct RemoteMoleculeTask(Task<Option<Molecule>>);

/// Receive molecule sent from client side
async fn recv_molecule_from(mut task_rx: crate::task::TaskReceiver<Molecule, ()>) -> Option<Molecule> {
    use crate::task::RemoteIO;

    info!("hello");
    if let Some(RemoteIO(mol, tx_out)) = task_rx.recv().await {
        // compute with job input
        // let output = compute_with(mol)?;
        // send job output to client side
        tx_out.send(()).ok()?;
        Some(mol)
    } else {
        // task channel closed
        None
    }
}

/// Start remote view service listening on molecules from remote client side.
pub fn start_remote_view_service(mut commands: Commands) {
    info!("starting remote view service ...");

    let (task_rx, task_tx) = crate::task::Task::<Molecule, ()>::new().split();
    IoTaskPool::get().spawn(routes::serve_remote_view(task_tx)).detach();

    let task = IoTaskPool::get().spawn(recv_molecule_from(task_rx));
    commands.insert_resource(RemoteMoleculeTask(task));
}

pub fn handle_remote_molecule(mut task: ResMut<RemoteMoleculeTask>) {
    if let Some(mol_task) = future::block_on(future::poll_once(&mut task.0)) {
        if let Some(mol) = mol_task {
            let natoms = mol.natoms();
            info!("get molecule: {natoms}");
        } else {
            debug!("mol task channel closed");
        }
    } else {
        debug!("not ready");
    }
}
// d1858fa3 ends here

// [[file:../bevy.note::0e0418a7][0e0418a7]]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkServer::new())
            .insert_resource(NetworkSettings::default())
            .add_startup_system(start_remote_view_service)
            .add_system(handle_remote_molecule);
    }
}
// 0e0418a7 ends here
