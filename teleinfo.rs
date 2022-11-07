use serialport;
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    println!("HELLO SERIAL");

    let serial_port = serialport::new("/dev/ttyS0", 1200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    let mut reader = BufReader::new(serial_port);
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();

    while line.contains("b'\x02'") {
        reader.read_line(&mut line).unwrap();
    }

    reader.read_line(&mut line).unwrap();

    println!("{}", line);
}
