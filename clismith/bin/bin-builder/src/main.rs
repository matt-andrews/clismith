use clap::{Parser, Subcommand};
use cr_compiler::compile;
use cr_core::dir_helpers::{DirType, set_dir};
use std::{env, path::PathBuf, process::ExitCode};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Version,
    Build {
        #[arg(short, long)]
        src: Option<PathBuf>,
        #[arg(short, long)]
        target: Option<PathBuf>,
    },
}

fn main() -> anyhow::Result<ExitCode> {
    let args = Cli::parse();
    match args.command {
        Commands::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(ExitCode::SUCCESS)
        }
        Commands::Build { src, target } => {
            //
            if let Some(src) = src {
                set_dir(DirType::Project, &src);
            }

            if let Some(target) = target {
                set_dir(DirType::Target, &target);
            }

            compile()?;

            Ok(ExitCode::SUCCESS)
        }
    }
}
