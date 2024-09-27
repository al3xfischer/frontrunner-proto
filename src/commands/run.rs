use crate::db::{create_laptime, fetch_id, update_lap, DbError};
use crate::serial::listen_com_port;
use crate::timer::LapTime;
use log::{error, info};
use odbc::safe::AutocommitOn;
use odbc::{create_environment_v3, Connection};
use std::process::exit;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub fn invoke(port: String, baud: u32, db_path: String) {
    let (tx, rx) = channel::<LapTime>();
    listen(tx, port, baud);
    persist(rx, db_path);
}

/// Listens to the given port for messages.
/// Once received, the message gets parsed and passed to the channel for further processing.
///
/// # Arguments
///
/// * `tx`: Transmitter (sender) for the channel
/// * `port`: USB port name
/// * `baud`: Baud rate
///
/// returns: ()
///
fn listen(tx: Sender<LapTime>, port: String, baud: u32) {
    thread::spawn(move || {
        listen_com_port(port.leak(), baud, |msg| {
            let time = LapTime::from(msg);
            info!("{}", &time);
            tx.send(time).unwrap();
        });
    });
}

/// Listens for times sent over the channel and persists them to the DB
///
/// # Arguments
///
/// * `rx`: Receiver for a LapTime (message)
///
/// returns: ()
///
fn persist(rx: Receiver<LapTime>, db_path: String) {
    let env = match create_environment_v3() {
        Ok(env) => env,
        _ => {
            error!("Unable to open DB");
            exit(1);
        }
    };

    let connection_string = create_connection_string(db_path);
    let conn = env
        .connect_with_connection_string(connection_string)
        .expect("Unable to open connection");

    loop {
        match rx.recv() {
            Ok(time) => persist_lap(&conn, time),
            Err(err) => error!("{}", err),
        }
    }
}

/// Saves the given lap to the database
///
/// # Arguments
///
/// * `conn`: Database connection
/// * `lap`: LapTime to persist
///
/// returns: ()
///
fn persist_lap(conn: &Connection<AutocommitOn>, lap: LapTime) {
    match fetch_id(conn) {
        Ok(id) => match update_lap(conn, id, &lap) {
            Ok(_) => info!("Updated Id: {}", id),
            Err(err) => error!("{}", err),
        },
        Err(DbError::NoId) => {
            if let Err(err) = create_laptime(conn, &lap) {
                error!("{}", err);
            } else {
                info!("Inserted new lap")
            }
        }
        Err(err) => {
            error!("{}", err);
        }
    };
}

fn create_connection_string(path: String) -> &'static str {
    format!(
        "Driver={{Microsoft Access Driver (*.mdb, *.accdb)}};Dbq={};Uid=Admin;",
        path
    )
    .leak()
}
