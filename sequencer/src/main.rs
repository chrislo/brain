extern crate rosc;

use std::thread;
use std::time::Instant;

mod event;
use event::Event;

mod measure;
use measure::Measure;

mod track;
use track::Track;

fn main() {
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
        let now = Instant::now();

        let events = track.events_between(Measure(tick_counter, 96), Measure(tick_counter + 1, 96));
        for event in events {
            event.send_via_osc();
        }

        tick_counter += 1;

        let elapsed_time = now.elapsed();
        let sleep_time = tick_length.to_duration(bpm) - elapsed_time;
        thread::sleep(sleep_time);
    }
}
