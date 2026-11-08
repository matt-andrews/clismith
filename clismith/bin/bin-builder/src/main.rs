use clap::{Parser, Subcommand};
use cr_compiler::compile;
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
        #[arg(short, long, default_value = "./")]
        src: PathBuf,
        #[arg(short, long, default_value = "./target")]
        target: PathBuf,
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
            //TODO: check for target dir and create if not exist
            compile(&src, &target)?;
            Ok(ExitCode::SUCCESS)
        }
    }
}
