use crate::motit::{handler, log};
use serialport::{self, SerialPort};


pub fn serial_write(
    mut port:Box<dyn SerialPort>,
    mut handler:handler::ThreadHandler
)
{
    loop {
        let write_msg = handler.receiver.recv().unwrap();
    }
}