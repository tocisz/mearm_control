mod config;
mod mov;
mod test1;
mod test2;
mod test3;
mod keyboard;

use rumqttc::Client;

use keyboard::Msg;

fn main() {
    // get_mqtt_options() should be defined in config.rs (but configuration data is not committed)
    let (client, mut connection) = Client::new(config::get_mqtt_options(), 10);
    let mut b = bus::Bus::new(2);
    let mut rx = b.add_rx();
    // test1::test1(client);
    // test2::test2(client, b.add_rx());
    test3::test3(client);
    keyboard::wait_for_key_press(b);

    // Iterate to poll the eventloop for connection progress
    for (_i, _notification) in connection.iter().enumerate() {
        // println!("Notification = {:?}", notification);
        match rx.try_recv() {
            Ok(Msg::STOP) => {
                break;
            }
            _ => {}
        }
    }
}