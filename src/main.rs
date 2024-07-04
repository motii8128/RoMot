use motit::motit::{interface, controller, thread_utils, serial};
use std::thread as th;

fn main() {
    let task_reporter_con = thread_utils::ThreadConnector::<thread_utils::ActionID>::new();
    let task_reporter_serial = thread_utils::ThreadConnector::<thread_utils::ActionID>::new();
    let controller_node = thread_utils::ThreadConnector::<interface::DualShock4>::new();
    let mut controller_driver = controller::DualShock4Driver::new(controller::SERIAL).unwrap();
    let mut serial_driver = serial::SerialDriver::new("/dev/ttyACM0".to_string(), 115200);

    th::spawn(move || controller_driver.read(task_reporter_con.publisher, control_node.publisher));

    loop {
        let input = control_node

        println!("x:{}, y:{}", input.sticks.left_x, input.sticks.left_y);
    }
}
