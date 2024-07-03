use std::sync::mpsc::{self, Receiver, Sender};

pub enum ThreadID {
    START = 1,
    END = 2,
    ERROR = 99,
    SUCCESS = 100
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