// [[file:../bevy.note::495e3d25][495e3d25]]
use bevy::prelude::*;
use gut::prelude::Result;
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

    type TaskState = crate::task::TaskSender<Molecule, ()>;

    #[axum::debug_handler]
    pub(super) async fn view_molecule(State(client): State<TaskState>, Json(mol): Json<Molecule>) -> Result<(), AppError> {
        // let computed = client.send(mol).await?;
        // Ok(Json(computed))
        Ok(())
    }
}
// 27477258 ends here

// [[file:../bevy.note::bd70c1ac][bd70c1ac]]
use std::net::{SocketAddr, ToSocketAddrs};

use tokio::{
    runtime::Runtime,
    sync::mpsc::{channel, Receiver, Sender},
};

#[derive(Resource)]
struct NetworkServer {
    runtime: Runtime,

    /// Handle to task that listens for new connections.
    listener_task: Option<tokio::task::JoinHandle<()>>,
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
        }
    }

    pub fn listen(&mut self) -> Result<()> {
        debug!("Started listening");
        self.listener_task = Some(self.runtime.spawn(start_server()));

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

async fn start_server() {
    use axum::{routing::get, Router};

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&"0.0.0.0:3039".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// bd70c1ac ends here

// [[file:../bevy.note::0e0418a7][0e0418a7]]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkServer::new())
            .insert_resource(NetworkSettings::default());
    }
}
// 0e0418a7 ends here
