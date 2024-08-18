mod db;
mod serial;
mod timer;

use crate::db::{create_laptime, fetch_id, update_lap, DbError};
use crate::serial::{list_ports, listen_com_port};
use crate::timer::LapTime;
use clap::{Parser, Subcommand};
use comfy_table::Table;
use core::str;
use log::{error, info};
use odbc::safe::AutocommitOn;
use odbc::{create_environment_v3, Connection};
use serialport::SerialPortType;
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
use std::process::exit;
use SerialPortType::UsbPort;

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
    },
}

const CONNECTION_STRING: &str = r"Driver={Microsoft Access Driver (*.mdb, *.accdb)};Dbq=C:\Users\Alex\Downloads\WSL2022\WSL2022.accdb;Uid=Admin;";

fn main() {
    setup_logging();
    let args = Args::parse();

    match args.cmd {
        Commands::List => {
            let ports = list_ports().expect("Unable to list ports");
            let mut table = Table::new();
            table.set_header(vec!["Name", "Serial", "PID"]);

            for p in ports
            {
                if let UsbPort(info) = p.port_type {
                    table.add_row(vec![
                        p.port_name.clone(),
                        info.serial_number.unwrap_or("-".into()),
                        info.pid.to_string(),
                    ]);
                }
            }

            println!("{}", table);
        }
        Commands::Run { port, baud } => {
            let env = match create_environment_v3() {
                Ok(env) => env,
                _ => {
                    error!("Unable to open DB");
                    exit(1);
                }
            };

            let conn = env
                .connect_with_connection_string(CONNECTION_STRING)
                .expect("Unable to open connection");

            listen_com_port(port.leak(), baud, |msg| {
                let time = LapTime::from(msg);
                info!("{}", &time);
                persist_lap(&conn, time)
            });
        }
    }
}

///
///
/// # Arguments
///
/// * `conn`: Database connection
/// * `lap`: LapTime to persist
///
/// returns: ()
///
fn persist_lap(conn: &Connection<AutocommitOn>, lap: LapTime) {
    let _x = match fetch_id(conn) {
        Ok(id) => {
            match update_lap(conn, id, &lap) {
                Ok(_) => info!("Updated Id: {}", id),
                Err(err) => error!("{}", err),
            }
            ()
        }
        Err(DbError::NoId) => {
            if let Err(err) = create_laptime(conn, &lap) {
                error!("{}", err);
            } else {
                info!("Inserted new lap")
            }
        }
        Err(err) => {
            error!("{}", err);
            ()
        }
    };
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
