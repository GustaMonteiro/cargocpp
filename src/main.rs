mod argument_parser;

use clap::Parser;

use crate::argument_parser::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, std } => {
            println!("COMMAND: New");
            println!("   -> Creating new project called: {} with C++{}", name, std);
        }
        Commands::Build { clean } => {
            println!("COMMAND: Build");
            if *clean {
                println!("   -> Flag --clean set. Cleaning before building...");
            }
            println!("   -> Building project...");
        }
        Commands::Clean => {
            println!("COMMAND: Clean");
            println!("   -> Cleaning...");
        }
        Commands::Run { quiet } => {
            println!("COMMAND: Run");
            if *quiet {
                println!("   -> Running in quiet mode (-q)...");
            } else {
                println!("   -> Running project...");
            }
        }
        Commands::Add { names } => {
            println!("COMMAND: Add");
            println!("   -> Packages to be added: {:?}", names);
        }
    }
}
