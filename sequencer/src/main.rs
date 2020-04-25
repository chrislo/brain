extern crate crossbeam;
extern crate rosc;

use crossbeam::crossbeam_channel::unbounded;
use std::net::UdpSocket;
use std::thread;
use std::time::Instant;

use sequencer::atom;
use sequencer::context::Context;
use sequencer::control::parse_incoming_osc_message;
use sequencer::control::Message;
use sequencer::measure::Measure;
use sequencer::track::Track;

fn main() {
    atom::init();

    let (s, r) = unbounded();

    let event_thread = thread::spawn(move || {
        let bpm = 120.0;
        let tick_length = Measure(1, 96);
        let mut current_tick = Measure(1, 96);

        let mut current_context = Context {
            track: Track::empty(),
        };

        loop {
            let now = Instant::now();
            let next_tick = current_tick + tick_length;

            let events = current_context
                .track
                .events_between(current_tick, next_tick);
            for event in events {
                event.send_via_osc();
            }

            let messages = r.try_iter().collect();
            let next_context = current_context.process_messages(messages);

            atom::update(&current_context, &next_context);

            let elapsed_time = now.elapsed();
            let sleep_time = tick_length.to_duration(bpm) - elapsed_time;
            thread::sleep(sleep_time);

            current_tick = next_tick;
            current_context = next_context;
        }
    });

    let sock = UdpSocket::bind("127.0.0.1:57120").unwrap();
    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, _addr)) => {
                let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                let message = parse_incoming_osc_message(packet);
                match message {
                    Message::NoteOn { .. } => s.send(message).unwrap(),
                    _ => {}
                }
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }

    event_thread.join().unwrap();
}
