use clap::{Parser, Subcommand};
use std::path::PathBuf;
use thiserror::Error;

use hf_codegen::target::{Arch, CallingConvention, Os, Target};

mod compile;

#[derive(Debug, Error)]
enum ParseError {
    #[error("invalid target triplet")]
    InvalidTargetTriplet,
    #[error("unknown host in target triplet")]
    UnknownTargetTripletHost,
}

#[derive(Debug, Clone)]
struct TargetTriplet {
    target: Target,
}

impl std::str::FromStr for TargetTriplet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 3 {
            return Err(ParseError::InvalidTargetTriplet);
        }

        let host = parts[0].to_string();
        let _vendor = parts[1].to_string();
        let system = parts[2].to_string();

        let arch = match host.as_str() {
            "x86" => Arch::X86,
            "x86_64" => Arch::X86_64,
            "wasm32" => Arch::Wasm32,
            "wasm64" => Arch::Wasm64,
            "aarch64" => Arch::Aarch64,
            "riscv" => Arch::RiscV,
            "mips" => Arch::Mips,
            "powerpc" => Arch::PowerPc,
            "sparc" => Arch::Sparc,
            "z390" => Arch::Z390,
            "m68k" => Arch::M68k,
            "spirv" => Arch::SpirV,
            "riscv32" => Arch::RiscV32,
            "riscv64" => Arch::RiscV64,
            "riscv128" => Arch::RiscV128,
            _ => return Err(ParseError::UnknownTargetTripletHost),
        };

        let os = match system.as_str() {
            "windows" => Some(Os::Windows),
            "linux" => Some(Os::Linux),
            "bsd" => Some(Os::Bsd),
            "solaris" => Some(Os::Solaris),
            "illumos" => Some(Os::Illumos),
            "haiku" => Some(Os::Haiku),
            "redox" => Some(Os::Redox),
            "theseus" => Some(Os::Theseus),
            _ => None,
        };

        let calling_convention = if let Some(os) = os {
            os.calling_convention()
        } else {
            // TODO: Match system to calling convention
            todo!()
        };

        let target = Target::new(arch, calling_convention);

        Ok(TargetTriplet { target })
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

        /// A list of files to compile.
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Compile { opt, target, files } => {
            let target = target.map(|t| t.target).unwrap_or_else(|| Target::native());
            let settings = compile::CompileSettings { optimisation: opt };
            for file in files {
                compile::compile(file, target.clone(), &settings).unwrap();
            }
        }
    }
}
