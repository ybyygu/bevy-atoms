// [[file:../../bevy.note::cb0b9648][cb0b9648]]
use clap::CommandFactory;
use clap::{Parser, Subcommand};
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl, Result};

#[derive(Parser)]
#[command(name = "hello")]
/// Does awesome things
struct Cli {
    who: String,
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}

/// Write "Hello" with given name
fn hello<T>(args: ArgMatches, _context: &mut T) -> Result<Option<String>> {
    Ok(Some(format!("Hello, {}", args.get_one::<String>("who").unwrap())))
}

fn main() -> Result<()> {
    let mut repl = Repl::new(())
        .with_name("gchemol-view")
        .with_version("v0.2.0")
        .with_description("A simple molecule viewer")
        .with_banner("Welcome to gchemol-view")
        .with_command(Cli::command(), hello);


    repl.run()
}
// cb0b9648 ends here
