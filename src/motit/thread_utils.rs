use std::sync::mpsc::{self, Receiver, Sender};

pub enum ActionID {
    START = 1,
    END = 2,
    ERROR = 3,
    SUCCESS = 4
}

pub struct ThreadConnector<T>
{
    pub publisher:Sender<T>,
    pub subscriber:Receiver<T>
}


impl<T> ThreadConnector<T> {
    pub fn new()->ThreadConnector<T>
    {
        let (send_vec, receive_vec) = mpsc::channel::<T>();

        
        ThreadConnector{publisher:send_vec, subscriber:receive_vec}
    }
}