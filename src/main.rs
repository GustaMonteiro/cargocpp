mod argument_parser;
mod routines;

use clap::Parser;

use crate::argument_parser::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, std } => {
            routines::new(name, std);
        }
        Commands::Build { clean } => {
            routines::build(clean);
        }
        Commands::Clean => {
            routines::clean();
        }
        Commands::Run { quiet } => {
            routines::run(quiet);
        }
        Commands::Add { names } => {
            routines::add(names);
        }
    }
}
