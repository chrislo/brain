extern crate rosc;

use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

mod event;
use event::Event;

fn main() {
    let host_addr = SocketAddrV4::from_str("127.0.0.1:49161").unwrap();
    let to_addr = SocketAddrV4::from_str("127.0.0.1:49162").unwrap();
    let sock = UdpSocket::bind(host_addr).unwrap();

    loop {
        let event = Event;

        sock.send_to(&event.to_osc_message(), to_addr).unwrap();

        thread::sleep(Duration::from_millis(500));
    }
}
