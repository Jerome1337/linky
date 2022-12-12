use serialport;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;
use std::collections::HashMap;

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

  let mut complete_frame: Vec<u8> = Vec::new();

  loop {
    let mut serial_buf: Vec<u8> = vec![0; 1];

    serial_port.read(serial_buf.as_mut_slice()).expect("Found no data!");

    if serial_buf == b"\x02" {
      continue;
    } else if serial_buf == b"\x03" {
      let stringified_complete_frame = String::from_utf8_lossy(&complete_frame);
      let string_frames: Vec<&str> = stringified_complete_frame.split('\r').collect();
      let mut info: HashMap<String, String> = HashMap::new();

      for string_frame in string_frames.iter() {
        let splitted_frame: Vec<&str> = string_frame.trim().split(' ').collect();

        if splitted_frame.len() > 1 {
          info.insert(String::from(splitted_frame[0]), String::from(splitted_frame[1]));
        }
      }

      for (mesure_key, value) in info.iter() {
        if is_key_valid(mesure_key) {
          println!("MESURE KEY {} VALUE {}", mesure_key, value);
        }
      }

      complete_frame.clear();

      println!("----------------------------")
    }

    complete_frame = [complete_frame, serial_buf].concat();
  }
}

fn is_key_valid(key: &String) -> bool {
  let mesure_keys = ["BASE", "IMAX", "HCHC", "IINST", "PAPP", "ISOUSC", "ADCO", "HCHP"];
  return mesure_keys.contains(&key.as_str());
}
