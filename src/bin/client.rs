// [[file:../../bevy.note::d351781a][d351781a]]
use clap::CommandFactory;
use clap::{Parser, Subcommand};
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl, Result};

use gchemol::prelude::*;
use gchemol::Molecule;
// d351781a ends here

// [[file:../../bevy.note::cb0b9648][cb0b9648]]
use reqwest::blocking::Client;

struct Context {
    client: Option<Client>,
    server: String,
}

impl Default for Context {
    fn default() -> Self {
        let client = reqwest::blocking::Client::builder().build().expect("reqwest client");

        Self {
            client: client.into(),
            server: "127.0.0.1:3039".to_owned(),
        }
    }
}

#[derive(Parser)]
#[command(name = "load")]
/// Load molecule for remote view
struct Load {
    molfile: String,
}

#[derive(Parser)]
#[command(name = "connect")]
struct Connect {
    #[arg(default_value = "127.0.0.1:3039")]
    server: String,
}

fn connect(args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    if let Some(server) = args.get_one::<String>("server") {
        context.server = server.to_owned();
        Ok(Some(format!("connect to {server}")))
    } else {
        Ok(Some(format!("invalid")))
    }
}

/// Write "Hello" with given name
fn load(args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    // FIXME: remove unwrap
    if let Some(molfile) = args.get_one::<String>("molfile") {
        if let Some(client) = context.client.as_mut() {
            let mol = Molecule::from_file(molfile).unwrap();
            let server = &context.server;
            let uri = format!("http://{server}/view-molecule");
            let resp = client.post(&uri).json(&mol).send().unwrap().text().unwrap();
            Ok(Some(format!("serve resp: {resp}")))
        } else {
            Ok(Some(format!("invalid")))
        }
    } else {
        Ok(Some(format!("invalid")))
    }
}
// cb0b9648 ends here

// [[file:../../bevy.note::7a8f9dd7][7a8f9dd7]]
/// Prepend name to list
fn main() -> Result<()> {
    let mut repl = Repl::new(Context::default())
        .with_name("gchemol-view")
        .with_version("v0.2.0")
        .with_description("A simple molecule viewer")
        .with_banner("Welcome to gchemol-view")
        .with_command(Connect::command(), connect)
        .with_command(Load::command(), load);

    repl.run()
}
// 7a8f9dd7 ends here
