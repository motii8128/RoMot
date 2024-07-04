use crate::motit::{thread::ActionID, interface::SerialPacket};

use serialport::{self, SerialPort};

use std::time::Duration;
use std::sync::mpsc::{Receiver, Sender};

pub struct SerialHandler
{
    pub port_path:String,
    pub baud_rate:u32,
    pub port:Box<dyn SerialPort>,
}

impl SerialHandler {
    pub fn new(path:String, baudrate:u32)->SerialHandler
    {
        let por = serialport::new(path.as_str(), baudrate).timeout(Duration::from_millis(100)).open().unwrap();

        SerialHandler{port_path:path, baud_rate:baudrate, port:por}
    }
    pub fn get_serial_info(&self)->(&str, u32)
    {
        (self.port_path.as_str(), self.baud_rate)
    }

    pub fn write_task(&mut self, packet_receiver:Receiver<SerialPacket>, reporter:Sender<ActionID>)
    {
        let _ = reporter.send(ActionID::START);
        loop {
            let packet = packet_receiver.recv().unwrap();

            let buf = format!("s{},{},{},{}e", packet.x, packet.y, packet.m1, packet.m2);

            match self.port.write(buf.as_bytes()) {
                Ok(_)=>{
                    let _ = reporter.send(ActionID::SUCCESS);
                }
                Err(_)=>{
                    let _ = reporter.send(ActionID::ERROR);
                }
            }
        }
    }
}