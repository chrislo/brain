extern crate rosc;

use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

fn main() {
    let host_addr = SocketAddrV4::from_str("127.0.0.1:49161").unwrap();
    let to_addr = SocketAddrV4::from_str("127.0.0.1:49162").unwrap();
    let sock = UdpSocket::bind(host_addr).unwrap();

    struct Event;

    impl Event {
        fn to_osc_message(&self) -> Vec<u8> {
            encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/sampler/1".to_string(),
                args: vec![],
            }))
            .unwrap()
        }
    }

    loop {
        let event = Event;

        sock.send_to(&event.to_osc_message(), to_addr).unwrap();

        thread::sleep(Duration::from_millis(500));
    }
}
