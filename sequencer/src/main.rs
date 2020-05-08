extern crate crossbeam;
extern crate rosc;

use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::SocketAddrV4;
use std::str::FromStr;

use crossbeam::crossbeam_channel::unbounded;
use std::net::UdpSocket;
use std::thread;
use std::time::Instant;

use sequencer::atom;
use sequencer::config;
use sequencer::context::Context;
use sequencer::control::parse_incoming_osc_message;
use sequencer::control::Message;
use sequencer::track::Track;
use std::time::Duration;

fn main() {
    config::parse();
    atom::init();

    let (s, r) = unbounded();

    let event_thread = thread::spawn(move || {
        let mut current_tick_number = 0;

        let mut current_context = Context {
            track: Track::empty(),
            active_note_number: 1,
            swing_amount: 0,
            bpm: 120.0,
        };

        loop {
            send_clock();

            let now = Instant::now();
            let next_tick_number = current_tick_number + 1;

            let events = current_context.events(current_tick_number);
            for event in events {
                event.send_via_osc();
            }

            let messages = r.try_iter().collect();
            let next_context = current_context.process_messages(messages);

            atom::update(&current_context, &next_context);

            let elapsed_time = now.elapsed();
            let sleep_time = tick_duration(current_context.bpm) - elapsed_time;
            thread::sleep(sleep_time);

            current_tick_number = next_tick_number;
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
                    Message::Unhandled => {}
                    _ => s.send(message).unwrap(),
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

fn tick_duration(bpm: f32) -> Duration {
    let ms_per_beat = (60. / bpm) * 1000.;
    let length_of_measure_in_beats = 4. / 96 as f32;
    let length_of_measure_in_ms = (length_of_measure_in_beats * ms_per_beat) as u64;

    Duration::from_millis(length_of_measure_in_ms)
}

fn send_clock() {
    let packet = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/*/clock".to_string(),
        args: vec![],
    }))
    .unwrap();

    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let to_addr = SocketAddrV4::from_str("127.0.0.1:57200").unwrap();

    sock.send_to(&packet, to_addr).unwrap();
}
