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
    let event_thread = thread::spawn(move || {
        let bpm = 120.0;
        let tick_length = Measure(1, 96);
        let mut current_tick = Measure(1, 96);

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

        loop {
            let now = Instant::now();
            let next_tick = current_tick + tick_length;
            let events = track.events_between(current_tick, next_tick);
            for event in events {
                event.send_via_osc();
            }

            let elapsed_time = now.elapsed();
            let sleep_time = tick_length.to_duration(bpm) - elapsed_time;
            thread::sleep(sleep_time);

            current_tick = next_tick;
        }
    });

    event_thread.join().unwrap();
}
