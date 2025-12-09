mod argument_parser;
mod files;
mod routines;

use clap::Parser;

use crate::argument_parser::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, std } => {
            routines::new(name, std);
        }
        Commands::Build { clean, release } => {
            routines::build(clean, release);
        }
        Commands::Clean => {
            routines::clean();
        }
        Commands::Run { quiet, release, args } => {
            routines::run(quiet, release, args);
        }
        Commands::Add { names } => {
            routines::add(names);
        }
    }
}
