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

#[derive(Resource, Default)]
struct RemoteMolecule {
    inner: Option<Molecule>,
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

// [[file:../bevy.note::3977bbe1][3977bbe1]]
mod routes {
    use super::app_error::AppError;
    use gchemol_core::Molecule;
    use gut::prelude::Result;

    use axum::extract::State;
    use axum::Json;
    use crossbeam_channel::{Receiver, Sender};

    #[axum::debug_handler]
    async fn view_molecule(State(tx): State<Sender<Molecule>>, Json(mol): Json<Molecule>) -> Result<(), AppError> {
        super::info!("handle client request: view-molecule mol: {}", mol.title());
        tx.send(mol).unwrap();
        Ok(())
    }

    /// Start remote view service listening on molecules from remote client side.
    pub async fn serve_remote_view(task_tx: Sender<Molecule>) -> Result<()> {
        use axum::{routing::get, Router};

        use axum::routing::post;
        use gchemol_core::Molecule;

        super::info!("start axum service ...");
        let app = Router::new().route("/view-molecule", post(view_molecule)).with_state(task_tx);
        axum::Server::bind(&"127.0.0.1:3039".parse().unwrap())
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
// 3977bbe1 ends here

// [[file:../bevy.note::a39e37a0][a39e37a0]]
mod systems {
    use super::server::NetworkServer;

    use bevy::prelude::*;
    // Using crossbeam_channel instead of std as std `Receiver` is `!Sync`
    use crossbeam_channel::{bounded, Receiver};
    use gchemol_core::Molecule;

    #[derive(Resource, Deref)]
    pub struct StreamReceiver(Receiver<Molecule>);
    pub struct StreamEvent(Molecule);

    // This system reads from the receiver and sends events to Bevy
    pub fn read_molecule_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
        for from_stream in receiver.try_iter() {
            info!("get mol event");
            events.send(StreamEvent(from_stream));
        }
    }

    pub fn handle_remote_molecule_view(
        mut commands: Commands,
        mut reader: EventReader<StreamEvent>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        for (_per_frame, StreamEvent(mol)) in reader.iter().enumerate() {
            info!("handle received mol: {}", mol.title());

            // show a cube on received
            commands.spawn(PbrBundle {
                mesh: meshes.add(shape::Cube::default().into()),
                ..default()
            });
        }
    }

    /// listen on client requests for remote view of molecule
    pub fn setup_remote_view_service(mut commands: Commands) {
        info!("starting remote view service ...");

        let mut server = super::server::NetworkServer::new();
        // listen on client requests for remote view of molecule
        let (tx, rx) = bounded::<Molecule>(1);
        let h1 = server.runtime.spawn(super::routes::serve_remote_view(tx));
        server.listener_task = h1.into();

        commands.insert_resource(server);
        commands.insert_resource(StreamReceiver(rx));
    }
}
// a39e37a0 ends here

// [[file:../bevy.note::2408ae28][2408ae28]]
mod server {
    use bevy::prelude::*;
    use gchemol_core::Molecule;
    use gut::prelude::Result;
    use tokio::runtime::Runtime;

    #[derive(Resource)]
    pub struct NetworkServer {
        /// tokio runtime
        pub runtime: Runtime,

        /// handle to task that listens for new connections.
        pub listener_task: Option<tokio::task::JoinHandle<std::result::Result<(), bevy::asset::Error>>>,
    }

    impl NetworkServer {
        pub fn new() -> NetworkServer {
            NetworkServer {
                runtime: tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Could not build tokio runtime"),
                listener_task: None,
            }
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
}
// 2408ae28 ends here

// [[file:../bevy.note::0e0418a7][0e0418a7]]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkSettings::default())
            .add_event::<systems::StreamEvent>()
            .add_startup_system(systems::setup_remote_view_service)
            .add_system(systems::read_molecule_stream)
            .add_system(systems::handle_remote_molecule_view);
        // .add_plugin(bevy_tokio_tasks::TokioTasksPlugin::default())
        // .add_startup_system(systems::setup_remote_view_service);
        // .add_system(handle_remote_molecule);
    }
}
// 0e0418a7 ends here
