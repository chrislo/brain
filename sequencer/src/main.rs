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
use sequencer::output::Output;
use std::time::Duration;

fn main() {
    let o2m_output = Output::o2m();
    let sampler_output = Output::sampler();

    config::parse();
    atom::init();

    let (s, r) = unbounded();

    thread::spawn(move || {
        let mut current_context = Context::default();

        loop {
            o2m_output.send(output::clock_message());

            let now = Instant::now();

            let events = current_context.events();
            for event in events {
                sampler_output.send(event.to_osc_message());
            }

            let messages = r.try_iter().collect();
            let next_context = current_context.process_messages(messages).advance_tick();

            let o2m_messages = atom::update(&current_context, &next_context);
            for message in o2m_messages {
                o2m_output.send(message);
            }

            let elapsed_time = now.elapsed();
            let sleep_time = tick_duration(current_context.bpm) - elapsed_time;
            thread::sleep(sleep_time);

            current_context = next_context;
        }
    });

    let sock = UdpSocket::bind("127.0.0.1:57120").unwrap();

    loop {
        if let Some(msg) = input::process_incoming_message(&sock) {
            s.send(msg).unwrap()
        }
    }
}

fn tick_duration(bpm: f32) -> Duration {
    let ms_per_beat = (60. / bpm) * 1000.;
    let length_of_measure_in_beats = 4. / 96_f32;
    let length_of_measure_in_ms = (length_of_measure_in_beats * ms_per_beat) as u64;

    Duration::from_millis(length_of_measure_in_ms)
}
