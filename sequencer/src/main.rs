extern crate crossbeam;
extern crate rosc;

use crossbeam::crossbeam_channel::unbounded;
use rosc::OscPacket;
use std::net::UdpSocket;
use std::thread;
use std::time::Instant;

mod event;
use event::Event;

mod measure;
use measure::Measure;

mod track;
use track::Track;

fn main() {
    let (s, r) = unbounded();

    let event_thread = thread::spawn(move || {
        let bpm = 120.0;
        let tick_length = Measure(1, 96);
        let mut current_tick = Measure(1, 96);

        let mut track = Track::new(vec![]);

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
                let message = parse_packet(packet);
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

fn parse_packet(packet: OscPacket) -> Result<Measure, &'static str> {
    match packet {
        OscPacket::Message(msg) => match msg.args[0] {
            rosc::OscType::Int(i) => Ok(Measure(i - 35, 16)),
            _ => Err("Unable to handle packet"),
        },
        _ => Err("Unable to handle packet"),
    }
}

#[cfg(test)]
use rosc::OscMessage;

#[test]
fn test_parse_packet() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/10/note_on".to_string(),
        args: vec![rosc::OscType::Int(36)],
    });
    let msg = parse_packet(packet);
    assert_eq!(msg.unwrap(), Measure(1, 16));

    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/10/note_on".to_string(),
        args: vec![rosc::OscType::Int(51)],
    });
    let msg = parse_packet(packet);
    assert_eq!(msg.unwrap(), Measure(16, 16));
}
