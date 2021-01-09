use rumqttc::{Client, QoS};
use std::thread;
use std::time::Duration;
use byteorder::{ByteOrder, LittleEndian};

fn create_move_message(x: i16, y: i16, z: i16, s: i16) -> [u8; 8] {
    let mut buf = [0u8; 8];
    LittleEndian::write_i16(&mut buf, x);
    LittleEndian::write_i16(&mut buf[2..], y);
    LittleEndian::write_i16(&mut buf[4..], z);
    LittleEndian::write_i16(&mut buf[6..], s);
    buf
}

fn create_grip_message(a: i16, s: i16) -> [u8; 4] {
    let mut buf = [0u8; 4];
    LittleEndian::write_i16(&mut buf, a);
    LittleEndian::write_i16(&mut buf[2..], s);
    buf
}

pub fn arm_move(client: &mut Client, x: i16, y: i16, z: i16, s: i16, pause: u64) {
    let message = create_move_message(x, y, z, s);
    client.publish("robot_arm", QoS::AtMostOnce, false, message).unwrap();
    thread::sleep(Duration::from_millis(pause));
}

pub fn arm_grip(client: &mut Client, a: i16, s: i16, pause: u64) {
    let message = create_grip_message(a, s);
    client.publish("robot_arm", QoS::AtMostOnce, false, message).unwrap();
    thread::sleep(Duration::from_millis(pause));
}