// [[file:../bevy.note::495e3d25][495e3d25]]
use bevy::prelude::*;
use serde::*;

use gchemol_core::Molecule;
use gut::prelude::Result;
// 495e3d25 ends here

// [[file:../bevy.note::e05220eb][e05220eb]]
mod console;
// e05220eb ends here

// [[file:../bevy.note::02dd4467][02dd4467]]
// Using crossbeam_channel instead of std as std `Receiver` is `!Sync`
use crossbeam_channel::{Receiver, Sender};

type RemoteCommandReceiver = Receiver<RemoteCommand>;
type RemoteCommandSender = Sender<RemoteCommand>;

fn new_channel() -> (RemoteCommandSender, RemoteCommandReceiver) {
    crossbeam_channel::bounded(1)
}

/// Command that can be evoked from the remote client side
#[derive(Debug, Deserialize, Serialize)]
pub enum RemoteCommand {
    /// Label atom
    Label,
    /// Delete molecule
    Delete,
    /// Load molecule
    Load(Vec<Molecule>),
}

/// Settings to configure the network, both client and server
#[derive(Resource, Default)]
struct NetworkSettings {
    address: Option<std::net::SocketAddr>,
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
    use super::RemoteCommand;
    use super::RemoteCommandSender;
    use gchemol_core::Molecule;
    use gut::prelude::Result;

    use axum::extract::State;
    use axum::Json;
    use crossbeam_channel::{Receiver, Sender};

    #[axum::debug_handler]
    async fn view_molecule(State(tx): State<RemoteCommandSender>, Json(mol): Json<Molecule>) -> Result<(), AppError> {
        super::info!("handle client request: view-molecule mol: {}", mol.title());
        let mols = vec![mol];
        tx.send(RemoteCommand::Load(mols)).unwrap();
        Ok(())
    }

    /// Start remote view service listening on molecules from remote client side.
    pub async fn serve_remote_view(task_tx: RemoteCommandSender) -> Result<()> {
        use axum::routing::post;
        use axum::{routing::get, Router};

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
    #![deny(warnings)]
    use super::server::NetworkServer;
    use super::{RemoteCommand, RemoteCommandReceiver};

    use bevy::prelude::*;

    #[derive(Resource, Deref)]
    pub struct StreamReceiver(RemoteCommandReceiver);
    pub struct StreamEvent(RemoteCommand);

    // This system reads from the receiver and sends events to Bevy
    pub fn read_molecule_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
        for from_stream in receiver.try_iter() {
            info!("get mol event");
            events.send(StreamEvent(from_stream));
        }
    }

    pub fn handle_remote_molecule_view(
        mut reader: EventReader<StreamEvent>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
        molecule_query: Query<Entity, With<crate::player::Molecule>>,
    ) {
        for (_per_frame, StreamEvent(cmd)) in reader.iter().enumerate() {
            match cmd {
                RemoteCommand::Load(mols) => {
                    // FIXME: rewrite
                    let mol = &mols[0];
                    info!("handle received mol: {}", mol.title());
                    // remove existing molecule
                    if let Ok(molecule_entity) = molecule_query.get_single() {
                        info!("molecule removed");
                        commands.entity(molecule_entity).despawn_recursive();
                    }
                    // show molecule on received
                    crate::player::spawn_molecule(mol, true, 0, &mut commands, &mut meshes, &mut materials, &mut lines);
                    break;
                }
                _ => {
                    //
                }
            }
        }
    }

    /// listen on client requests for remote view of molecule
    pub fn setup_remote_view_service(mut commands: Commands) {
        info!("starting remote view service ...");

        let mut server = NetworkServer::new();
        // listen on client requests for remote view of molecule
        let rx = server.listen();
        commands.insert_resource(StreamReceiver(rx));
        commands.insert_resource(server);
    }

    pub fn stop_server_on_exit(mut exit_events: EventReader<bevy::app::AppExit>, mut server: ResMut<NetworkServer>) {
        for _ in exit_events.iter() {
            server.stop();
            break;
        }
    }
}
// a39e37a0 ends here

// [[file:../bevy.note::2408ae28][2408ae28]]
mod server {
    #![deny(warnings)]

    use super::{new_channel, RemoteCommandReceiver};
    use bevy::prelude::*;
    use tokio::runtime::Runtime;

    #[derive(Resource)]
    pub struct NetworkServer {
        /// tokio runtime
        runtime: Runtime,

        /// handle to task that listens for new connections.
        listener_task: Option<tokio::task::JoinHandle<Result<(), bevy::asset::Error>>>,
    }

    impl NetworkServer {
        pub fn new() -> Self {
            Self {
                runtime: tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Could not build tokio runtime"),
                listener_task: None,
            }
        }

        /// listen on client requests for remote view of molecule
        pub fn listen(&mut self) -> RemoteCommandReceiver {
            let (tx, rx) = new_channel();
            let h1 = self.runtime.spawn(super::routes::serve_remote_view(tx));
            self.listener_task = h1.into();
            rx
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
            .add_system(systems::stop_server_on_exit)
            .add_system(systems::handle_remote_molecule_view);
    }
}
// 0e0418a7 ends here
