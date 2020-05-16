extern crate crossbeam;
extern crate rosc;

use crossbeam::crossbeam_channel::unbounded;
use std::net::UdpSocket;
use std::thread;
use std::time::Instant;

use sequencer::atom;
use sequencer::config;
use sequencer::context::Context;
use sequencer::input;
use sequencer::output;
use std::time::Duration;

fn main() {
    config::parse();
    atom::init();

    let (s, r) = unbounded();

    thread::spawn(move || {
        let mut current_tick_number = 0;

        let mut current_context = Context::default();

        loop {
            output::send_clock();

            let now = Instant::now();
            let next_tick_number = current_tick_number + 1;

            let events = current_context.events();
            for event in events {
                output::send_osc_to_sampler(event.to_osc_message());
            }

            let messages = r.try_iter().collect();
            let next_context = current_context.process_messages(messages).advance_tick();

            atom::update(&current_context, &next_context);

            let elapsed_time = now.elapsed();
            let sleep_time = tick_duration(current_context.bpm) - elapsed_time;
            thread::sleep(sleep_time);

            current_tick_number = next_tick_number;
            current_context = next_context;
        }
    });

    let sock = UdpSocket::bind("127.0.0.1:57120").unwrap();

    loop {
        match input::process_incoming_message(&sock) {
            Some(msg) => s.send(msg).unwrap(),
            None => {}
        }
    }
}

fn tick_duration(bpm: f32) -> Duration {
    let ms_per_beat = (60. / bpm) * 1000.;
    let length_of_measure_in_beats = 4. / 96 as f32;
    let length_of_measure_in_ms = (length_of_measure_in_beats * ms_per_beat) as u64;

    Duration::from_millis(length_of_measure_in_ms)
}
