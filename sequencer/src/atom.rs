use crate::context::Context;
use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::collections::HashSet;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

pub fn init() {
    handshake();
    turn_all_lights_off();
}

pub fn handshake() {
    let packet = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/atom/note_off".to_string(),
        args: vec![
            rosc::OscType::Int(16),
            rosc::OscType::Int(0),
            rosc::OscType::Int(127),
        ],
    }))
    .unwrap();

    send_osc_to_o2m(packet);
}

pub fn update(current_context: &Context, next_context: &Context) {
    let current_context_active_pads = active_pads(current_context);
    let next_context_active_pads = active_pads(next_context);

    for pad_added in next_context_active_pads.difference(&current_context_active_pads) {
        turn_light_on(*pad_added);
    }

    for pad_removed in current_context_active_pads.difference(&next_context_active_pads) {
        turn_light_off(*pad_removed);
    }
}

fn active_pads(context: &Context) -> HashSet<i32> {
    context
        .track
        .active_sixteenths_with_note_number(context.active_note_number)
        .iter()
        .map(|s| sixteenth_to_note_number(*s))
        .collect()
}

fn sixteenth_to_note_number(sixteenth: i32) -> i32 {
    sixteenth + 36
}

fn turn_all_lights_off() {
    for n in 36..52 {
        turn_light_off(n);
    }
}

fn turn_light_on(note_number: i32) {
    let packet = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/atom/note_on".to_string(),
        args: vec![
            rosc::OscType::Int(1),
            rosc::OscType::Int(note_number),
            rosc::OscType::Int(127),
        ],
    }))
    .unwrap();

    send_osc_to_o2m(packet);
}

fn turn_light_off(note_number: i32) {
    let packet = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/atom/note_on".to_string(),
        args: vec![
            rosc::OscType::Int(1),
            rosc::OscType::Int(note_number),
            rosc::OscType::Int(0),
        ],
    }))
    .unwrap();

    send_osc_to_o2m(packet);
}

fn send_osc_to_o2m(packet: Vec<u8>) {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let to_addr = SocketAddrV4::from_str("127.0.0.1:57200").unwrap();

    sock.send_to(&packet, to_addr).unwrap();
}

#[cfg(test)]
use crate::control::Message;

#[cfg(test)]
use crate::track::Track;

#[test]
fn test_active_pads() {
    let context = Context {
        track: Track::empty(),
        active_note_number: 1,
    };

    let messages = vec![Message::NoteOn { note_number: 37 }];
    let mut processed_context = context.process_messages(messages);

    assert_eq!(1, active_pads(&processed_context).len());
    assert!(active_pads(&processed_context).contains(&37));

    // active_note_number was 1 when steps added, so no pads active
    // when we increment the active_note_number
    processed_context.active_note_number = 2;
    assert_eq!(0, active_pads(&processed_context).len());
}
