mod gen_passwords;
mod gen_hashes;
mod dump_hashes;

use crate::gen_passwords::gen_passwords;
use crate::gen_hashes::gen_hashes;
use crate::dump_hashes::dump_hashes;

use hashassin_core::args::Algorithm;
use hashassin_core::max_num;
use hashassin_core::max_threads;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(name = "Project1")]
#[command(version = "0.0.1")]
#[command(about = "Generate passwords, hashes and dump hashes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    GenPasswords {
        #[arg(long, default_value_t = 4, value_parser = clap::value_parser!(u8).range(1..256))]
        chars: u8,
        #[arg(long, value_parser = clap::value_parser!(PathBuf))]
        out_file: Option<PathBuf>,
        #[arg(long, default_value_t = 1, value_parser = max_threads)]
        threads: usize,
        #[arg(long, value_parser = max_num)]
        num: usize,
    },
    GenHashes {
        #[arg(long)]
        in_file: PathBuf,
        #[arg(long)]
        out_file: PathBuf,
        #[arg(long, default_value_t = 1, value_parser = max_threads)]
        threads: usize,
        #[arg(long, value_enum)]
        algorithm: Algorithm,
    },
    DumpHashes {
        #[arg(long)]
        in_file: PathBuf,
    },
}

fn main() {
    let args = Cli::parse();

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Application started");

    match args.command {
        Commands::GenPasswords {
            chars,
            out_file,
            threads,
            num,
        } => {
            println!("Generating passwords...");
            gen_passwords(chars, out_file, threads, num);
        }
        Commands::GenHashes {
            in_file,
            out_file,
            threads,
            algorithm,
        } => {
            gen_hashes(in_file, out_file, threads, algorithm);
        }
        Commands::DumpHashes { in_file } => {
            dump_hashes(in_file);
        }
    }
}