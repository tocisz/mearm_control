use bus::Bus;
use std::{thread, io};
use std::io::Read;

const STEP: i16 = 10;

pub fn wait_for_key_press(mut m: Bus<Msg>) {
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


#[derive(Clone)]
pub enum Msg {
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