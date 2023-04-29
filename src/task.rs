// [[file:../bevy.note::fc11019a][fc11019a]]
// #![deny(warnings)]
//! Task for remote execution
//!
//! # Example
//!
//! ```ignore
//! let (rx, tx) = Task::new().split();
//! 
//! // client side
//! tx_input1 = tx.clone();
//! tx_input2 = tx.clone();
//! let out1 = tx_input1.send("test input 1")?;
//! let out2 = tx_input2.send("test input 2")?;
//! 
//! // server side
//! if let Some(RemoteIO(input, tx_out)) = rx.recv().await {
//!     // compute with job input
//!     let output = compute_with(input)?;
//!     // send job output to client side
//!     tx_out.send(output)?;
//! } else {
//!     // task channel closed
//!     // ...
//! }
//! ```
// fc11019a ends here

// [[file:../bevy.note::d0a34fac][d0a34fac]]
use gut::prelude::*;
use tokio::sync::{mpsc, oneshot};

use std::fmt::Debug;
use std::marker::Send;
// d0a34fac ends here

// [[file:../bevy.note::*types][types:1]]
type Computed<O> = O;
type TxInput<I, O> = mpsc::Sender<RemoteIO<I, O>>;

/// The receiver of task for remote execution
pub type RxInput<I, O> = mpsc::Receiver<RemoteIO<I, O>>;
/// The sender of computational results.
pub type TxOutput<O> = oneshot::Sender<Computed<O>>;

/// RemoteIO contains input and output for remote execution. The first field in tuple
/// is job input, and the second is for writing job output.
#[derive(Debug)]
pub struct RemoteIO<I, O>(
    /// Input data for starting computation
    pub I,
    /// A oneshot channel for send computational output.
    pub TxOutput<O>,
);
// types:1 ends here

// [[file:../bevy.note::cf62514b][cf62514b]]
/// The client side for remote execution
#[derive(Debug, Clone, Default)]
pub struct TaskSender<I, O> {
    tx_inp: Option<TxInput<I, O>>,
}

impl<I: Debug, O: Debug + Send> TaskSender<I, O> {
    /// Ask remote side compute with `input` and return the computed.
    pub async fn send(&self, input: impl Into<I>) -> Result<Computed<O>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.tx_inp
            .as_ref()
            .expect("task input")
            .send(RemoteIO(input.into(), tx))
            .await
            .map_err(|err| format_err!("task send error: {err:?}"))?;
        let computed = rx.await?;
        Ok(computed)
    }
}
// cf62514b ends here

// [[file:../bevy.note::778f4e75][778f4e75]]
/// The server side for remote execution
#[derive(Debug)]
pub struct TaskReceiver<I, O> {
    rx_inp: RxInput<I, O>,
}

impl<I, O> TaskReceiver<I, O> {
    /// Receives the next task for this receiver.
    pub async fn recv(&mut self) -> Option<RemoteIO<I, O>> {
        self.rx_inp.recv().await
    }
}

fn new_interactive_task<I, O>() -> (TaskReceiver<I, O>, TaskSender<I, O>) {
    let (tx_inp, rx_inp) = tokio::sync::mpsc::channel(1);

    let server = TaskReceiver { rx_inp };
    let client = TaskSender { tx_inp: tx_inp.into() };

    (server, client)
}
// 778f4e75 ends here

// [[file:../bevy.note::2afcba30][2afcba30]]
/// A Task channel for remote execution (multi-producer, single-consumer)
pub struct Task<I, O> {
    sender: TaskSender<I, O>,
    receiver: TaskReceiver<I, O>,
}

impl<I, O> Task<I, O> {
    /// Create a task channel for computation of molecule in client/server
    /// architecture
    pub fn new() -> Self {
        let (receiver, sender) = new_interactive_task();
        Self { sender, receiver }
    }

    /// Splits a single task into separate read and write half
    pub fn split(self) -> (TaskReceiver<I, O>, TaskSender<I, O>) {
        match self {
            Self {
                sender: tx,
                receiver: rx,
            } => (rx, tx),
        }
    }
}
// 2afcba30 ends here
