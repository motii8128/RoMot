use motit::motit::thread::ThreadHandler;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, reciever) = mpsc::channel();

    thread::spawn(move|| {
        sender.send(expensive_computation()).unwrap();
    });
    
    // Do some useful work for awhile
    
    // Let's see what that answer was
    println!("{:?}", receiver.recv().unwrap());
}
