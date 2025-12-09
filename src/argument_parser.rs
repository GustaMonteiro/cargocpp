use clap::{Parser, Subcommand};

const VALID_STD_VERSIONS: [&str; 4] = ["14", "17", "20", "23"];

fn parse_std_version(s: &str) -> Result<String, String> {
    if VALID_STD_VERSIONS.contains(&s) {
        Ok(s.to_string())
    } else {
        Err(format!(
            "Invalid C++ Standard: '{s}'. C++ standards are: {}",
            VALID_STD_VERSIONS.join(", ")
        ))
    }
}

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
    
        /// C++ standard to be used (e.g.: 14, 17, 20). Default: 20
        #[arg(
            long, 
            default_value_t = String::from("20"),
            value_parser = parse_std_version
        )]
        std: String,
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
