use std::sync::mpsc::{self, Receiver, Sender};

pub enum ActionID {
    START = 1,
    END = 2,
    ERROR = 3,
    SUCCESS = 4
}

pub struct Connector<T>
{
    pub sender:Sender<T>,
    pub receiver:Receiver<T>
}


impl<T> Connector<T> {
    pub fn new()->Connector<T>
    {
        let (send_vec, receive_vec) = mpsc::channel::<T>();

        
        Connector{sender:send_vec, receiver:receive_vec}

    }
}
