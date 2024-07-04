use motit::motit::{controller, interface, serial, thread_utils::{self, check_task}};
use std::thread;

fn main() {
    let controller_node = thread_utils::ThreadConnector::<interface::DualShock4>::new(
        "DualShock4Node".to_string());
    let mut controller_driver = controller::DualShock4Driver::new(controller::SERIAL).unwrap();
    let serial_node = thread_utils::ThreadConnector::<interface::SerialPacket>::new(
        "SerialNode".to_string());
    let mut serial_driver = serial::SerialDriver::new("/dev/ttyACM0".to_string(), 115200);

    thread::spawn(move || controller_driver.read(controller_node.task_sender, controller_node.publisher));
    check_task(controller_node.name, controller_node.task_receiver.recv().unwrap());

    thread::spawn(move || serial_driver.write_task(serial_node.subscriber, serial_node.task_sender));
    check_task(serial_node.name, serial_node.task_receiver.recv().unwrap());

    loop {
        let controller_input = controller_node.subscriber.recv().unwrap();

        let x_value = (controller_input.sticks.left_x*100.0) as i32+100;
        let y_value = (controller_input.sticks.left_y*100.0) as i32+100;
    }
}
