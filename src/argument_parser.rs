use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Cargocpp", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new project with the given name
    New {
        /// New project's name
        name: String,
    },

    /// Build the project
    Build {
        /// Cleans the build artifacts before building
        #[arg(long)]
        clean: bool,
    },

    /// Clean all build artifacts
    Clean,

    /// Run the project
    Run {
        /// Don't show build output (quiet mode)
        #[arg(short, long)]
        quiet: bool,
    },

    /// Add one or more dependencies to the project
    Add {
        /// Dependencies
        #[arg(num_args = 1..)]
        names: Vec<String>,
    },
}
