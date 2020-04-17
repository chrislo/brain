extern crate crossbeam;
extern crate rosc;

use crossbeam::crossbeam_channel::unbounded;
use std::net::UdpSocket;
use std::thread;
use std::time::Instant;

mod event;
use event::Event;

mod measure;
use measure::Measure;

mod track;
use track::Track;

mod control;

fn main() {
    let (s, r) = unbounded();

    let event_thread = thread::spawn(move || {
        let bpm = 120.0;
        let tick_length = Measure(1, 96);
        let mut current_tick = Measure(1, 96);

        let mut track = Track::new();

        loop {
            match r.try_recv() {
                Ok(msg) => {
                    track.add_event(Event { start: msg });
                }
                _ => {}
            }

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

    let sock = UdpSocket::bind("0.0.0.0:49161").unwrap();
    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, _addr)) => {
                let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                let message = control::parse_packet(packet);
                match message {
                    Ok(msg) => s.send(msg).unwrap(),
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
