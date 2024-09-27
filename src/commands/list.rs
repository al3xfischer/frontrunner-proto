use comfy_table::Table;
use serialport::SerialPortType::UsbPort;
use crate::serial::list_ports;

pub fn invoke() {
    let ports = list_ports().expect("Unable to list ports");
    let mut table = Table::new();
    table.set_header(vec!["Name", "Serial", "PID"]);

    for p in ports {
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
