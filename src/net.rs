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
        Self {}
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

// [[file:../bevy.note::77121fc1][77121fc1]]
use bevy_tokio_tasks::{TaskContext, TokioTasksPlugin, TokioTasksRuntime};

/// Start remote view service listening on molecules from remote client side.
fn start_remote_view_service(mut commands: Commands, runtime: ResMut<TokioTasksRuntime>) {
    info!("starting remote view service ...");

    let (task_rx, task_tx) = crate::task::Task::<Molecule, ()>::new().split();
    runtime.spawn_background_task(|mut ctx| async move {
        self::routes::serve_remote_view(task_tx).await;
        ctx.run_on_main_thread(move |ctx| {
            if let Some(mut clear_color) = ctx.world.get_resource_mut::<ClearColor>() {
                // clear_color.0 = COLORS[color_index];
                // println!("Changed clear color to {:?}", clear_color.0);
            }
        })
        .await;
    });
}
// 77121fc1 ends here

// [[file:../bevy.note::0e0418a7][0e0418a7]]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkSettings::default())
            .add_plugin(TokioTasksPlugin::default())
            .add_startup_system(start_remote_view_service);
        // .add_system(handle_remote_molecule);
    }
}
// 0e0418a7 ends here
