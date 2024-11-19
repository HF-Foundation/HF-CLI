use clap::{Parser, Subcommand};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("invalid target triplet")]
    InvalidTargetTriplet,
}

#[derive(Debug, Clone)]
struct TargetTriplet {
    host: String,
    vendor: String,
    system: String,
}

impl std::str::FromStr for TargetTriplet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 3 {
            return Err(ParseError::InvalidTargetTriplet);
        }

        Ok(TargetTriplet {
            host: parts[0].to_string(),
            vendor: parts[1].to_string(),
            system: parts[2].to_string(),
        })
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Compile {
        /// Sets the optimisation level. 0 = no optimisation, 3 = maximum optimisation.
        #[arg(short, long, default_value_t = 0)]
        opt: u8,

        /// Sets the target triplet. Can also be used to specify a target configuration file.
        #[arg(long, value_parser = clap::value_parser!(TargetTriplet))]
        target: Option<TargetTriplet>,

        files: Vec<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Compile { opt, target, files } => {
            println!("Optimisation level: {}", opt);
            println!("Target: {:?}", target);
            println!("Files: {:?}", files);
        }
    }
}
