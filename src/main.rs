use motit::motit::{controller, interface::{self, SerialPacket}, serial, thread_utils::{self, check_task}};
use std::thread;

fn main() {
    let controller_node = thread_utils::ThreadConnector::<interface::DualShock4>::new(
        "DualShock4Node".to_string());
    let mut controller_driver = controller::DualShock4Driver::new(controller::SERIAL).unwrap();
    let serial_node = thread_utils::ThreadConnector::<interface::SerialPacket>::new(
        "SerialNode".to_string());
    let mut serial_driver = serial::SerialDriver::new("/dev/ttyUSB0".to_string(), 115200);

    thread::spawn(move || controller_driver.read(controller_node.task_sender, controller_node.publisher));
    check_task(&controller_node.name, controller_node.task_receiver.recv().unwrap());

    thread::spawn(move || serial_driver.write_task(serial_node.subscriber, serial_node.task_sender));
    check_task(&serial_node.name, serial_node.task_receiver.recv().unwrap());

    loop {
        check_task(&controller_node.name, controller_node.task_receiver.recv().unwrap());
        let controller_input = controller_node.subscriber.recv().unwrap();
        

        let x_value = (controller_input.sticks.left_x*100.0) as i32+100;
        let y_value = (controller_input.sticks.left_y*100.0) as i32+100;

        let m1_value = if controller_input.dpad.up_key{
            200
        }
        else if controller_input.dpad.down_key{
            0
        }
        else
        {
            100
        };

        let m2_value = if controller_input.btns.r1
        {
            200
        }
        else if controller_input.btns.l1
        {
            0
        }
        else
        {
            100
        };

        let packet = SerialPacket{
            x:x_value,
            y:y_value,
            m1:m1_value,
            m2:m2_value
        };

        println!("{}", format!("{},{},{},{}", packet.x, packet.y, packet.m1, packet.m2));

        let _ = serial_node.publisher.send(packet).unwrap();
        check_task(&serial_node.name, serial_node.task_receiver.recv().unwrap());
    }
}
