use crate::motit::{thread_utils, interface::Packet};

use serialport::{self, SerialPort};

use std::time::Duration;
use std::sync::mpsc::{Receiver, Sender};

pub struct SerialDriver
{
    pub port_path:String,
    pub baud_rate:u32,
    pub port:Box<dyn SerialPort>,
    pub packet_subscriber:Receiver<Packet>,
    pub task_checker:Sender<u8>
}

impl SerialDriver {
    pub fn new(path:String, baudrate:u32, packet_subscriber_:Receiver<Packet>, task_checker_:Sender<u8>)->SerialDriver
    {
        let por = serialport::new(path.as_str(), baudrate).timeout(Duration::from_millis(100)).open().unwrap();

        SerialDriver{port_path:path, baud_rate:baudrate, port:por, packet_subscriber:packet_subscriber_, task_checker:task_checker_}
    }
    pub fn get_serial_info(&self)->(&str, u32)
    {
        (self.port_path.as_str(), self.baud_rate)
    }

    pub fn task(&mut self)
    {
        let _ = self.task_checker.send(thread_utils::START);

        loop {
            let packet = self.packet_subscriber.recv().unwrap();

            // let buf = format!("{},{},{},{}e", packet.x, packet.y, packet.m1, packet.m2);
            let buf = format!("s{},{},{},{},{}e", packet.x, packet.y, packet.ro, packet.m1, packet.m2);
            println!("{}", buf);

            match self.port.write(buf.as_bytes()) {
                Ok(_)=>{
                    let _ = self.task_checker.send(thread_utils::SUCCESS);
                }
                Err(_)=>{
                    let _ = self.task_checker.send(thread_utils::ERROR);
                }
            }
        }
    }
}