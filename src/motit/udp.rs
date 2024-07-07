use std::net::UdpSocket;
use crate::motit::{interface::{Packet, MCUDevice, MCUDeviceList}, thread_utils};
use std::sync::mpsc::{Receiver, Sender};

pub struct UDPDriver
{
    pub address:String,
    pub dest:String,
    pub socket:UdpSocket
}

impl UDPDriver {
    pub fn new(address_:String, destination:String)->UDPDriver
    {
        let sock = UdpSocket::bind(address_.as_str()).unwrap();

        UDPDriver { address: address_, dest:destination, socket: sock }
    }

    pub fn task(&mut self, packet_receiver:Receiver<Packet>, reporter:Sender<u8>)
    {
        let _ = reporter.send(thread_utils::START);
        loop {
            let packet = packet_receiver.recv().unwrap();

            let buf = format!("s{},{},{},{}e", packet.x, packet.y, packet.m1, packet.m2);

            match self.socket.send_to(buf.as_bytes(), &self.dest) {
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

pub struct MCUSearcher
{
    pub socket:UdpSocket,
    pub publisher:Sender<MCUDevice>,
    pub task_checker:Sender<u8>
}

impl MCUSearcher {
    pub fn new(publisher_:Sender<MCUDevice>, task_checker_:Sender<u8>)->MCUSearcher
    {
        let sock = UdpSocket::bind("0.0.0.0:64203").unwrap();

        MCUSearcher { socket: sock, publisher: publisher_, task_checker: task_checker_ }
    }

    pub fn task(&mut self)
    {
        let mut buf = [0_u8; 256];
        let _ = self.task_checker.send(thread_utils::START);

        loop {
            match self.socket.recv(&mut buf) {
                Ok(size)=>{
                    let read_string = String::from_utf8_lossy(&buf[0..size]).to_string();

                    let (addr, name) = purse_mcu_device(read_string);

                    let _ = self.task_checker.send(thread_utils::SUCCESS);
                    let _ = self.publisher.send(MCUDevice { address_port: addr, device_name: name });
                }
                Err(_)=>{
                    let _ = self.task_checker.send(thread_utils::ERROR);
                }
            }
        }
    }
}

fn purse_mcu_device(str:String)->(String, String)
{
    if str.find(',').is_some()
    {
        let vec:Vec<&str> = str.split(",").collect();

        (vec[0].to_string(), vec[1].to_string())
    }
    else {
        return ("".to_string(),"".to_string());
    }
}

