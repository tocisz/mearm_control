use std::thread;
use rumqttc::Client;

use crate::mov::*;

#[allow(dead_code)]
pub fn test1(mut client: Client) {
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