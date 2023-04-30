// [[file:../bevy.note::495e3d25][495e3d25]]
use bevy::prelude::*;
use gut::prelude::Result;

use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
// 495e3d25 ends here

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
mod handlers {
    use super::app_error::AppError;
    use axum::extract::State;
    use axum::Json;
    use gchemol_core::Molecule;

    pub type TaskState = crate::task::TaskSender<Molecule, ()>;

    #[axum::debug_handler]
    pub(super) async fn view_molecule(State(client): State<TaskState>, Json(mol): Json<Molecule>) -> Result<(), AppError> {
        let resp = client.send(mol).await?;
        Ok(())
    }
}
// 27477258 ends here

// [[file:../bevy.note::fe9e7673][fe9e7673]]
impl NetworkServer {
    pub async fn serve_remote_view(task_tx: crate::task::TaskSender<gchemol_core::Molecule, ()>) -> Result<()> {
        use axum::{routing::get, Router};

        use axum::routing::post;
        use gchemol_core::Molecule;
        use handlers::view_molecule;

        let app = Router::new().route("/view-molecule", post(view_molecule)).with_state(task_tx);
        axum::Server::bind(&"127.0.0.1:3039".parse().unwrap())
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }

    pub fn handle_molecule_view_task(&mut self) -> Option<gchemol_core::Molecule> {
        use crate::task::RemoteIO;

        if let Some(rx) = &mut self.task_rx {
            // Execute the future, blocking the current thread until completion
            if let Some(mol) = self.runtime.block_on(async {
                info!("hello");
                if let Some(RemoteIO(mol, tx_out)) = rx.recv().await {
                    // compute with job input
                    // let output = compute_with(mol)?;
                    // send job output to client side
                    tx_out.send(()).ok()?;
                    Some(mol)
                } else {
                    // task channel closed
                    None
                }
            }) {
                Some(mol)
            } else {
                None
            }
        } else {
            None
        }
    }
}
// fe9e7673 ends here

// [[file:../bevy.note::bd70c1ac][bd70c1ac]]
use std::net::{SocketAddr, ToSocketAddrs};

use tokio::{
    runtime::Runtime,
    sync::mpsc::{channel, Receiver, Sender},
};

#[derive(Resource)]
struct NetworkServer {
    /// tokio runtime
    runtime: Runtime,
    /// handle to task that listens for new connections.
    listener_task: Option<tokio::task::JoinHandle<std::result::Result<(), bevy::asset::Error>>>,

    /// the channel for server side to get molecule from client side
    task_rx: Option<crate::task::TaskReceiver<gchemol_core::Molecule, ()>>,
}

/// Settings to configure the network, both client and server
#[derive(Resource, Default)]
struct NetworkSettings {
    address: Option<SocketAddr>,
}

impl NetworkServer {
    pub(crate) fn new() -> NetworkServer {
        NetworkServer {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not build tokio runtime"),
            listener_task: None,
            task_rx: None,
        }
    }

    pub fn listen(&mut self) -> Result<()> {
        use gchemol_core::Molecule;

        info!("Started listening for molecule from client side.");
        let (task_rx, task_tx) = crate::task::Task::<Molecule, ()>::new().split();
        self.task_rx = task_rx.into();
        self.listener_task = Some(self.runtime.spawn(Self::serve_remote_view(task_tx)));

        Ok(())
    }

    /// Disconnect all clients and stop listening.
    ///
    /// # NOTE
    /// will do nothing if no active listening.
    pub fn stop(&mut self) {
        if let Some(conn) = self.listener_task.take() {
            conn.abort();
        }
    }
}
// bd70c1ac ends here

// [[file:../bevy.note::0e0418a7][0e0418a7]]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkServer::new())
            .insert_resource(NetworkSettings::default());
            // .add_startup_system(start_server);
            // .add_system(remote_view);
    }
}
// 0e0418a7 ends here
