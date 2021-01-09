use std::thread;
use rumqttc::Client;
use bus::BusReader;

use crate::mov::*;
use crate::keyboard::Msg;

#[allow(dead_code)]
pub fn test2(mut client: Client, mut rx: BusReader<Msg>) {
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
