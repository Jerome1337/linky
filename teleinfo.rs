use serialport;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;

fn main() {
    println!("HELLO SERIAL");

    let mut serial_port = serialport::new("/dev/ttyS0", 1200)
        .data_bits(DataBits::Seven)
        .flow_control(FlowControl::None)
        .parity(Parity::Even)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    let mut frame: Vec<u8> = Vec::new();
    loop {
        let mut serial_buf: Vec<u8> = vec![0; 1];

        serial_port.read(serial_buf.as_mut_slice()).expect("Found no data!");
        
        //let test = &serial_buf[0];
        if serial_buf == b"\x02" {
            continue;
        } else if serial_buf == b"\x03" {
            let test = String::from_utf8_lossy(&frame);
            println!("{}", test);
            frame.clear();

            continue;
        }

        frame = [frame, serial_buf].concat();
    }
}
