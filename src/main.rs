mod db;
mod serial;
mod timer;

use clap::{Parser, Subcommand};
use core::str;
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
pub(crate) mod commands;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// List all available ports
    List,
    /// Execute the listener
    Run {
        #[arg(long, short)]
        /// Port name. For example: COM7
        port: String,
        #[arg(default_value_t = 4800, long, short)]
        /// Baud rate for the specified port
        baud: u32,
        #[arg(long, short)]
        /// Database file path
        db_path: String,
    },
}

fn main() {
    setup_logging();
    let args = Args::parse();

    match args.cmd {
        Commands::List => commands::list::invoke(),
        Commands::Run {
            port,
            baud,
            db_path,
        } => commands::run::invoke(port, baud, db_path),
    }
}

fn setup_logging() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("frontrunner.log").unwrap(),
        ),
    ])
    .unwrap();
}
