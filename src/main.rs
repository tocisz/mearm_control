mod config;

use rumqttc::{MqttOptions, Client, QoS, Packet};
use std::time::Duration;
use std::{thread, io};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use byteorder::{ByteOrder, LittleEndian};

use rumqttc::Event::Incoming;
use std::thread::JoinHandle;

use bus::{Bus, BusReader};
use std::io::Read;

use rand::prelude::*;
use std::ops::Deref;

const STEP: i16 = 10;

#[derive(Clone)]
enum Msg {
    STOP,
    Up(i16),
    Down(i16),
    Left(i16),
    Right(i16),
    Front(i16),
    Back(i16),
    Close,
    Open,
}

fn test1(mut client: Client) {
    thread::spawn(move || loop {
        // arm_grip(&mut client, 90, 50, 100);
        arm_move(&mut client, 0, 120, 110, 50, 1000);
        arm_grip(&mut client, 180, 180, 100);
        arm_grip(&mut client, 0, 180, 100);
        arm_grip(&mut client, 180, 180, 100);
        arm_grip(&mut client, 90, 180, 100);

        arm_move(&mut client, 0, 120, 0, 50, 1000);
        arm_move(&mut client, 0, 220, 0, 50, 1000);
        arm_grip(&mut client, 180, 50, 1000);
    });
}

fn test2(mut client: Client, mut rx: BusReader<Msg>) {
    let mut pos: [i16; 3] = [0, 100, 50];
    let mut open = true;
    thread::spawn(move || loop {
        match rx.recv() {
            Ok(Msg::Left(d)) => pos[0] -= d,
            Ok(Msg::Right(d)) => pos[0] += d,
            Ok(Msg::Front(d)) => pos[1] += d,
            Ok(Msg::Back(d)) => pos[1] -= d,
            Ok(Msg::Up(d)) => pos[2] += d,
            Ok(Msg::Down(d)) => pos[2] -= d,
            Ok(Msg::Close) => {
                if open {
                    arm_grip(&mut client, 180, 50, 100);
                }
                open = false;
            }
            Ok(Msg::Open) => {
                if !open {
                    arm_grip(&mut client, 90, 50, 100);
                }
                open = true;
            }
            _ => {}
        }
        arm_move(&mut client, pos[0], pos[1], pos[2], 50, 100);
        println!("position: {} {} {} ({})", pos[0], pos[1], pos[2], if open { "open" } else { "closed" });
    });
}

fn dist(a: &(i16,i16), b: &(i16,i16)) -> i16 {
    f32::sqrt(
        f32::powi(i16::abs(b.0-a.0) as f32, 2)
            + f32::powi(i16::abs(b.1-a.1) as f32, 2)
    ) as i16
}

fn dist2time(d: i16, s: i16) -> u64 {
    (i16::abs(d) as u64) * 1300 / 6 / (s as u64) + 100
}

fn move_piece(client: &mut Client, p0: &(i16, i16), p1: &(i16, i16), p2: &(i16, i16)) {
    let high = 70;
    let empty_high = 30;
    let med = 0;
    let low = -30;

    let move_speed = 10;
    let grip_speed = 200;

    let d0 = dist(p0, p1);
    let t0 = dist2time(d0, move_speed);
    let d1 = dist(p1, p2);
    let t1 = dist2time(d1, move_speed);
    let short_move = d1 <= 75;
    let h = if short_move { med } else { high };

    arm_move(client, p1.0, p1.1, empty_high, move_speed, t0);
    arm_move(client, p1.0, p1.1, low, move_speed, dist2time(low - empty_high, move_speed));
    arm_grip(client, 180, grip_speed, 200);
    arm_move(client, p1.0, p1.1, h, move_speed, dist2time(h - low, move_speed));
    arm_move(client, p2.0, p2.1, h, move_speed, t1);
    arm_move(client, p2.0, p2.1, low, move_speed, dist2time(h - low, move_speed));
    arm_grip(client, 110, grip_speed, 200);
    arm_move(client, p2.0, p2.1, empty_high, move_speed, dist2time(empty_high - low, move_speed));
}

fn test3(mut client: Client) {
    let row0 = 120;
    let row1 = 190;
    let pos: Vec<(i16, i16)> = vec![
        (-120, row0),
        (-60, row0),
        (0, row0),
        (60, row0),
        (120, row0),
        (-60, row1),
        (0, row1+15),
        (60, row1),
        (120, 60),
        (120, 0),
        (-120, 60),
        (-120, 0),
    ];
    let mut is_empty = vec![true; pos.len()];
    let pebbles = 4;
    for i in 0..pebbles {
        is_empty[i] = false;
    }
    let mut prev = 0;
    let mut src = 1;
    let mut dst = 1;
    thread::spawn(move || {
        arm_move(&mut client, 0, row0, 30, 50, 1000);
        arm_grip(&mut client, 110, 50, 200);
        loop {
            prev = dst;

            loop {
                src = rand::random();
                src = src % pos.len();
                if src != prev && !is_empty[src] {
                    break;
                }
            }

            loop {
                dst = rand::random();
                dst = dst % pos.len();
                if dst != src && is_empty[dst] {
                    break;
                }
            }

            move_piece(&mut client, pos.get(prev).unwrap(), pos.get(src).unwrap(), pos.get(dst).unwrap());
            is_empty[src] = true;
            is_empty[dst] = false;
        }
    });
}

fn wait_for_key_press(mut m: Bus<Msg>) {
    thread::spawn(move || {
        let mut input = String::new();
        loop {
            input.truncate(0);
            io::stdin().read_line(&mut input).unwrap();
            input.truncate(input.len() - 1); // remove newline
            // println!("You said '{}'", input);
            if input == "exit" {
                m.broadcast(Msg::STOP);
                break;
            } else if input == "u" {
                m.broadcast(Msg::Up(STEP));
            } else if input == "d" {
                m.broadcast(Msg::Down(STEP));
            } else if input == "l" {
                m.broadcast(Msg::Left(STEP));
            } else if input == "r" {
                m.broadcast(Msg::Right(STEP));
            } else if input == "f" {
                m.broadcast(Msg::Front(STEP));
            } else if input == "b" {
                m.broadcast(Msg::Back(STEP));
            } else if input == "c" {
                m.broadcast(Msg::Close)
            } else if input == "o" {
                m.broadcast(Msg::Open)
            }
        }
    });
}

fn wait_for_key_press2(mut m: Bus<Msg>) {
    thread::spawn(move || {
        for b in io::stdin().lock().bytes() {
            let bu = b.unwrap();
            if bu == b'e' {
                m.broadcast(Msg::STOP);
                break;
            } else if bu == b'u' {
                m.broadcast(Msg::Up(STEP));
            } else if bu == b'd' {
                m.broadcast(Msg::Down(STEP));
            } else if bu == b'l' {
                m.broadcast(Msg::Left(STEP));
            } else if bu == b'r' {
                m.broadcast(Msg::Right(STEP));
            } else if bu == b'f' {
                m.broadcast(Msg::Front(STEP));
            } else if bu == b'b' {
                m.broadcast(Msg::Back(STEP));
            } else if bu == b'c' {
                m.broadcast(Msg::Close)
            } else if bu == b'o' {
                m.broadcast(Msg::Open)
            }
        }
    });
}

fn main() {
    // get_mqtt_options() should be defined in config.rs (but configuration data is not committed)
    let (mut client, mut connection) = Client::new(config::get_mqtt_options(), 10);
    let mut b = bus::Bus::new(2);
    let mut rx = b.add_rx();
    // test2(client, b.add_rx());
    test3(client);
    wait_for_key_press2(b);

    // Iterate to poll the eventloop for connection progress
    for (i, notification) in connection.iter().enumerate() {
        // println!("Notification = {:?}", notification);
        match rx.try_recv() {
            Ok(Msg::STOP) => {
                break;
            }
            _ => {}
        }
    }
}

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

fn arm_move(client: &mut Client, x: i16, y: i16, z: i16, s: i16, pause: u64) {
    let message = create_move_message(x, y, z, s);
    client.publish("robot_arm", QoS::AtMostOnce, false, message).unwrap();
    thread::sleep(Duration::from_millis(pause));
}

fn arm_grip(client: &mut Client, a: i16, s: i16, pause: u64) {
    let message = create_grip_message(a, s);
    client.publish("robot_arm", QoS::AtMostOnce, false, message).unwrap();
    thread::sleep(Duration::from_millis(pause));
}