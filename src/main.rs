use motit::motit::thread_utils::{ThreadConnector, check_task};
use motit::motit::controller::{DualShock4Driver, BLE};
use motit::motit::interface::{Packet, DualShock4};
use motit::motit::serial::SerialDriver;
use std::thread;

fn main() {
    let controller_node = ThreadConnector::<DualShock4>::new(
        "DualShock4Node".to_string());
    let mut controller_driver = DualShock4Driver::new(
        BLE,controller_node.publisher, controller_node.task_sender).unwrap();
    let serial_node = ThreadConnector::<Packet>::new(
        "SerialNode".to_string());
    let mut serial_driver = SerialDriver::new(
        "/dev/ttyACM0".to_string(), 115200,serial_node.subscriber, serial_node.task_sender);

    thread::spawn(move || controller_driver.task());
    check_task(&controller_node.name, controller_node.task_receiver.recv().unwrap());

    thread::spawn(move || serial_driver.task());
    check_task(&serial_node.name, serial_node.task_receiver.recv().unwrap());

    thread::sleep(std::time::Duration::from_millis(1000));

    loop {
        check_task(&controller_node.name, controller_node.task_receiver.recv().unwrap());
        let controller_input = controller_node.subscriber.recv().unwrap();
        

        let x_value = (controller_input.sticks.left_x*10.0) as i32+10;
        let y_value = (controller_input.sticks.left_y*10.0) as i32+10;
        let ro_v = (controller_input.sticks.right_x*10.0) as i32+ 10;

        let m1_value = if controller_input.dpad.up_key{
            20
        }
        else if controller_input.dpad.down_key{
            0
        }
        else
        {
            10
        };

        let m2_value = if controller_input.btns.r1
        {
            20
        }
        else if controller_input.btns.l1
        {
            0
        }
        else
        {
            10
        };

        let packet = Packet{
            x:x_value,
            y:y_value,
            ro:ro_v,
            m1:m1_value,
            m2:m2_value
        };
        let _ = serial_node.publisher.send(packet).unwrap();
        check_task(&serial_node.name, serial_node.task_receiver.recv().unwrap());
    }
}
