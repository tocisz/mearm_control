use std::thread;
use rumqttc::Client;

use crate::mov::*;
use std::time::Duration;

#[allow(dead_code)]
pub fn test(mut client: Client) {
    thread::spawn(move || loop {
        arm_read(&mut client);
        thread::sleep(Duration::from_millis(1000));
    });
}