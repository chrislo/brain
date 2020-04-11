extern crate rosc;

use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

mod event;
use event::Event;

mod measure;
use measure::Measure;

mod pattern;
use pattern::Pattern;

fn main() {
    let host_addr = SocketAddrV4::from_str("127.0.0.1:49161").unwrap();
    let to_addr = SocketAddrV4::from_str("127.0.0.1:49162").unwrap();
    let sock = UdpSocket::bind(host_addr).unwrap();

    let bpm = 120.0;
    let tick_length = Measure(1, 96);

    let pattern = Pattern::new(vec![
        Measure(1, 4),
        Measure(2, 4),
        Measure(3, 4),
        Measure(4, 4),
    ]);

    let mut tick_counter = 1;
    loop {
        let notes = pattern.notes_between(Measure(tick_counter, 96), Measure(tick_counter + 1, 96));

        for _note in notes {
            let event = Event;
            sock.send_to(&event.to_osc_message(), to_addr).unwrap();
        }

        tick_counter += 1;
        thread::sleep(Duration::from_millis(tick_length.to_ms(bpm)));
    }
}
