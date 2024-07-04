use std::sync::mpsc::{self, Receiver, Sender};
use crate::motit::log;

pub const START:u8 = 1;
pub const END:u8 = 2;
pub const SUCCESS:u8 = 3;
pub const ERROR:u8 = 4;

pub struct ThreadConnector<T>
{
    pub name:String,
    pub publisher:Sender<T>,
    pub subscriber:Receiver<T>,
    pub task_sender:Sender<u8>,
    pub task_receiver:Receiver<u8>,
}


impl<T> ThreadConnector<T> {
    pub fn new(name:String)->ThreadConnector<T>
    {
        let (send_vec, receive_vec) = mpsc::channel::<T>();
        let (ts, tr) = mpsc::channel::<u8>();

        
        ThreadConnector{name:name, publisher:send_vec, subscriber:receive_vec, task_receiver:tr, task_sender:ts}
    }
}

pub fn check_task(name:&str, get_ans:u8)
    {
        if get_ans == SUCCESS
        {
            // log::log_info(name, "Task OK");
        }
        else if get_ans == ERROR
        {
            log::log_error(name, "Found Error");
        }
        else if get_ans == START
        {
            log::log_info(name, "Start Task");
        }
        else {
            log::log_info(name, "End Task");
        }
    }