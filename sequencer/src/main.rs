extern crate rosc;

use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

mod event;
use event::Event;

mod measure;
use measure::Measure;

mod track;
use track::Track;

fn main() {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let to_addr = SocketAddrV4::from_str("127.0.0.1:49162").unwrap();

    let bpm = 120.0;
    let tick_length = Measure(1, 96);

    let track = Track::new(vec![
        Event {
            start: Measure(1, 4),
        },
        Event {
            start: Measure(2, 4),
        },
        Event {
            start: Measure(3, 4),
        },
        Event {
            start: Measure(4, 4),
        },
    ]);

    let mut tick_counter = 1;
    loop {
        let events = track.events_between(Measure(tick_counter, 96), Measure(tick_counter + 1, 96));

        for event in events {
            sock.send_to(&event.to_osc_message(), to_addr).unwrap();
        }

        tick_counter += 1;
        thread::sleep(Duration::from_millis(tick_length.to_ms(bpm)));
    }
}
