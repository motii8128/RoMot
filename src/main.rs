use std::time::Duration;

use serialport;
use serialport::SerialPort;

fn main() {
    let port: Box<dyn SerialPort> = serialport::new("/dev/ttyACM0", 115200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .timeout(Duration::from_millis(100))
        .open().unwrap();
}
