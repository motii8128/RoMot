use crate::motit::{thread_utils, interface::SerialPacket};

use serialport::{self, SerialPort};

use std::time::Duration;
use std::sync::mpsc::{Receiver, Sender};

pub struct SerialDriver
{
    pub port_path:String,
    pub baud_rate:u32,
    pub port:Box<dyn SerialPort>,
}

impl SerialDriver {
    pub fn new(path:String, baudrate:u32)->SerialDriver
    {
        let por = serialport::new(path.as_str(), baudrate).timeout(Duration::from_millis(100)).open().unwrap();

        SerialDriver{port_path:path, baud_rate:baudrate, port:por}
    }
    pub fn get_serial_info(&self)->(&str, u32)
    {
        (self.port_path.as_str(), self.baud_rate)
    }

    pub fn write_task(&mut self, packet_receiver:Receiver<SerialPacket>, reporter:Sender<u8>)
    {
        let _ = reporter.send(thread_utils::START);
        loop {
            let packet = packet_receiver.recv().unwrap();

            let buf = format!("{},{},{},{}", packet.x, packet.y, packet.m1, packet.m2);

            match self.port.write(buf.as_bytes()) {
                Ok(_)=>{
                    let _ = reporter.send(thread_utils::SUCCESS);
                }
                Err(_)=>{
                    let _ = reporter.send(thread_utils::ERROR);
                }
            }
        }
    }
}