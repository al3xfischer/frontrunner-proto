use log::{error, info};
use serialport::SerialPortInfo;
use serialport::SerialPortType::UsbPort;
use std::time::Duration;
use std::{io, thread};

const MESSAGE_LENGTH: usize = 23;

pub fn list_ports() -> serialport::Result<Vec<SerialPortInfo>> {
    let ports = serialport::available_ports()?
        .into_iter()
        .filter(|p| matches!(p.port_type, UsbPort(_)))
        .collect();

    Ok(ports)
}

pub fn listen_com_port<G>(name: &str, baud_rate: u32, action: G)
where
    G: Fn(String),
{
    loop {
        let port = serialport::new(name, baud_rate)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .data_bits(serialport::DataBits::Eight)
            .open();

        match port {
            Ok(mut port) => {
                port.write_request_to_send(true)
                    .expect("Unable to to: write request to send");
                info!("Receiving data on {} at {} baud:", &name, &baud_rate);
                let mut serial_buf: Vec<u8> = vec![0; 1000];
                let mut buffer_agg = String::new();
                loop {
                    match port.read(serial_buf.as_mut_slice()) {
                        Ok(t) => {
                            if let Ok(input) = std::str::from_utf8(&serial_buf[..t]) {
                                buffer_agg.push_str(input);

                                if buffer_agg.len() == MESSAGE_LENGTH {
                                    action(buffer_agg.clone());
                                    buffer_agg.clear();
                                }
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => {
                            error!("{:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to open \"{}\". Error: {}", name, e);
                thread::sleep(Duration::from_millis(250))
            }
        }
    }
}
