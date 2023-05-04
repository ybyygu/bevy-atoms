// [[file:../../bevy.note::d351781a][d351781a]]
use gut::cli::*;
use gut::prelude::*;

use gchemol::prelude::*;
use gchemol::Molecule;
use gosh_repl::{Actionable, Interpreter};
// d351781a ends here

// [[file:../../bevy.note::88699612][88699612]]
#[derive(Parser, Debug)]
#[clap(disable_help_subcommand = true)]
enum Cmd {
    /// Quit shell.
    #[command(name = "quit", alias = "q", alias = "exit")]
    Quit {},

    /// Show available commands.
    #[command(name = "help", alias = "h", alias = "?")]
    Help {},

    /// Load molecule for remote view
    #[command(name = "load")]
    Load {
        #[clap(name = "FILENAME")]
        molfile: String,
    },

    /// Connect to gchemol-view server
    #[command(name = "connect")]
    Connect {
        #[arg(default_value = "127.0.0.1:3039")]
        server: String,
    },

    #[command(name = "label")]
    /// Label atoms with their serial numbers
    Label {
        #[arg(short, long)]
        delete: bool,
    },

    #[command(name = "delete")]
    /// Delete current molecule
    Delete {
        //
    },
}
// 88699612 ends here

// [[file:../../bevy.note::a252f98f][a252f98f]]
use reqwest::blocking::Client;

#[derive(Debug, Clone)]
struct Action {
    client: Option<Client>,
    server: String,
}

impl Action {
    fn client(&mut self) -> Result<&mut Client> {
        if let Some(client) = self.client.as_mut() {
            Ok(client)
        } else {
            bail!("not connected yet")
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        let client = reqwest::blocking::Client::builder().build().expect("reqwest client");

        Self {
            client: client.into(),
            server: "127.0.0.1:3039".to_owned(),
        }
    }
}

impl Actionable for Action {
    type Command = Cmd;

    /// parse REPL commands from shell line input using clap
    fn try_parse_from<I, T>(iter: I) -> Result<Self::Command>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let r = Cmd::try_parse_from(iter)?;
        Ok(r)
    }

    /// Take action on REPL commands. Return Ok(true) will exit shell
    /// loop.
    fn act_on(&mut self, cmd: &Cmd) -> Result<bool> {
        match cmd {
            Cmd::Quit {} => return Ok(true),

            Cmd::Help {} => {
                let mut app = Cmd::command();
                if let Err(err) = app.print_help() {
                    eprintln!("clap error: {err:?}");
                }
                println!("");
            }

            Cmd::Load { molfile } => {
                let mol = Molecule::from_file(molfile)?;
                let server = &self.server;
                let uri = format!("http://{server}/view-molecule");
                let resp = self.client()?.post(&uri).json(&mol).send()?.text()?;
                println!("{resp:?}");
            }

            Cmd::Connect { server } => {
                println!("connect to {server}");
                self.server = server.to_owned();
            }

            Cmd::Label { delete } => {
                let server = &self.server;
                let uri = format!("http://{server}/label-atoms");
                let resp = self.client()?.post(&uri).send()?.text()?;
                println!("{resp:?}");
            }

            Cmd::Delete {} => {
                let server = &self.server;
                let uri = format!("http://{server}/delete-molecule");
                let resp = self.client()?.post(&uri).send()?.text()?;
                println!("{resp:?}");
            }

            o => {
                eprintln!("{:?}: not implemented yet!", o);
            }
        }

        Ok(false)
    }
}
// a252f98f ends here

// [[file:../../bevy.note::7a8f9dd7][7a8f9dd7]]
/// Prepend name to list
fn main() -> Result<()> {
    let action = Action::default();
    let x = Interpreter::new(action).with_prompt("gchemol-view> ").run::<Cmd>();

    Ok(())
}
// 7a8f9dd7 ends here
