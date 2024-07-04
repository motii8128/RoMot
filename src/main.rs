use motit::motit::{interface, controller, thread};
use std::thread as th;

fn main() {
    let controller_reporter = thread::Connector::<thread::ActionID>::new();
    let con_pubsub = thread::Connector::<interface::DualShock4>::new();
    let mut controller_driver = controller::DualShock4Driver::new(controller::SERIAL).unwrap();

    th::spawn(move || controller_driver.read(controller_reporter.sender, con_pubsub.sender));

    loop {
        let input = con_pubsub.receiver.recv().unwrap();

        println!("x:{}, y:{}", input.sticks.left_x, input.sticks.left_y);
    }
}
