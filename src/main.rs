use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    thread::spawn(||{
        for i in 0..100
        {
            println!("Sub Task:{}", i);

            thread::sleep(Duration::from_millis(100));
        }
    });

    
    for i in 0..10
    {
        println!("Main Task:{}", i);

        thread::sleep(Duration::from_millis(1000));
    }
}
