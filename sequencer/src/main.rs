extern crate rosc;

use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

mod event;
use event::Event;

mod measure;
use measure::Measure;

struct Pattern {
    notes: Vec<Measure>,
}

impl Pattern {
    fn new(notes: Vec<Measure>) -> Pattern {
        Pattern { notes: notes }
    }

    fn notes_between(&self, start: Measure, end: Measure) -> Vec<Measure> {
        let start_float = start.reduce_to_one_bar().to_float();
        let end_float = end.reduce_to_one_bar().to_float();

        self.notes
            .clone()
            .into_iter()
            .filter(|n| n.to_float() > start_float && n.to_float() <= end_float)
            .collect::<Vec<Measure>>()
    }
}

#[test]
fn test_notes_between() {
    let pattern = Pattern::new(vec![Measure(2, 16)]);

    let notes = pattern.notes_between(Measure(1, 16), Measure(3, 16));
    assert_eq!(Measure(2, 16), notes[0]);

    let notes = pattern.notes_between(Measure(3, 16), Measure(4, 16));
    assert!(notes.is_empty());

    let notes = pattern.notes_between(Measure(17, 16), Measure(19, 16));
    assert_eq!(Measure(2, 16), notes[0]);

    let pattern = Pattern::new(vec![Measure(1, 16)]);
    let notes = pattern.notes_between(Measure(1, 32), Measure(2, 32));
    assert_eq!(Measure(1, 16), notes[0]);
}

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
