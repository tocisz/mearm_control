mod config;
mod mov;
mod test1;
mod test2;
mod test3;
mod keyboard;
mod test_read;

use rumqttc::{Client, QoS, Event, Incoming};

use keyboard::Msg;
use byteorder::{LittleEndian, ByteOrder};

const SUBSCRIBE_TOPIC: &'static str = "robot_arm/o";

fn main() {
    // get_mqtt_options() should be defined in config.rs (but configuration data is not committed)
    let (mut client, mut connection) = Client::new(config::get_mqtt_options(), 10);
    client.subscribe(SUBSCRIBE_TOPIC, QoS::AtLeastOnce).unwrap();
    let mut b = bus::Bus::new(2);
    let mut rx = b.add_rx();
    // test1::test1(client);
    // test2::test2(client, b.add_rx());
    test3::test3(client);
    // test_read::test(client);
    keyboard::wait_for_key_press(b);

    // Iterate to poll the event loop for connection progress
    for notification in connection.iter() {
        match notification {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                if p.topic == SUBSCRIBE_TOPIC {
                    print_position(p.payload.as_ref());
                } else {
                    println!("Topic: {}, Payload: {:?}", p.topic, p.payload);
                }
            }
            Ok(Event::Incoming(i)) => println!("Incoming = {:?}", i),
            Ok(Event::Outgoing(o)) => println!("Outgoing = {:?}", o),
            Err(err) => println!("ConnectionError = {:?}", err)
        }
        match rx.try_recv() {
            Ok(Msg::STOP) => {
                break;
            }
            _ => {}
        }
    }
}

fn print_position(p0: &[u8]) {
    let x = LittleEndian::read_i16(p0);
    let y = LittleEndian::read_i16(&p0[2..]);
    let z = LittleEndian::read_i16(&p0[4..]);
    let g = LittleEndian::read_i16(&p0[6..]);
    println!("Position: {} {} {} {}", x, y, z, g);
}