use std::net::UdpSocket;
use crate::motit::{interface::Packet, thread_utils};
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

    pub fn send_task(&mut self, packet_receiver:Receiver<Packet>, reporter:Sender<u8>)
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

pub struct MCUDriver
{
    pub dest:String,
    pub socket:UdpSocket
}