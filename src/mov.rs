use rumqttc::{Client, QoS};
use std::thread;
use std::time::Duration;
use byteorder::{ByteOrder, LittleEndian};

enum CommandType {
    READ = 0, // read current position
    MOVE, // move to position with constant speed
    GRIP, // change gripper position
    MOVE_SEQ // TODO sequence of positions (every 20 ms) [what about gripper?]
}

fn create_move_message(x: i16, y: i16, z: i16, s: i16) -> [u8; 9] {
    let mut buf = [0u8; 9];
    buf[0] = CommandType::MOVE as u8;
    LittleEndian::write_i16(&mut buf[1..], x);
    LittleEndian::write_i16(&mut buf[3..], y);
    LittleEndian::write_i16(&mut buf[5..], z);
    LittleEndian::write_i16(&mut buf[7..], s);
    buf
}

fn create_grip_message(a: i16, s: i16) -> [u8; 5] {
    let mut buf = [0u8; 5];
    buf[0] = CommandType::GRIP as u8;
    LittleEndian::write_i16(&mut buf[1..], a);
    LittleEndian::write_i16(&mut buf[3..], s);
    buf
}

const PUBLISH_TOPIC: &'static str = "robot_arm/i";

pub fn arm_move(client: &mut Client, x: i16, y: i16, z: i16, s: i16, pause: u64) {
    let message = create_move_message(x, y, z, s);
    client.publish(PUBLISH_TOPIC, QoS::AtMostOnce, false, message).unwrap();
    thread::sleep(Duration::from_millis(pause));
}

pub fn arm_grip(client: &mut Client, a: i16, s: i16, pause: u64) {
    let message = create_grip_message(a, s);
    client.publish(PUBLISH_TOPIC, QoS::AtMostOnce, false, message).unwrap();
    thread::sleep(Duration::from_millis(pause));
}

pub fn arm_read(client: &mut Client) {
    let message = [CommandType::READ as u8; 1];
    client.publish(PUBLISH_TOPIC, QoS::AtMostOnce, false, message).unwrap();
}