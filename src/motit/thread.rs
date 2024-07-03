use std::sync::mpsc::{self, Receiver, Sender};
use crate::motit::log;

pub enum ThreadID {
    START = 1,
    END = 2,
    ERROR = 3,
    SUCCESS = 4
}

pub struct ThreadHandler
{
    pub sender:Sender<ThreadID>,
    pub receiver:Receiver<ThreadID>
}

impl ThreadHandler {
    pub fn new()->ThreadHandler
    {
        let (send_vec, receive_vec) = mpsc::channel::<ThreadID>();

        
        ThreadHandler{sender:send_vec, receiver:receive_vec}

    }
}
