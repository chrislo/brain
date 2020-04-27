use crate::context::Context;
use crate::track::Step;
use rosc::encoder;
use rosc::{OscMessage, OscPacket};
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
    let current_context_steps = current_context
        .track
        .active_steps_with_note_number(current_context.active_note_number);
    let next_context_steps = next_context
        .track
        .active_steps_with_note_number(next_context.active_note_number);

    for step_added in next_context_steps.difference(&current_context_steps) {
        turn_light_on(step_to_note_number(*step_added));
    }

    for step_removed in current_context_steps.difference(&next_context_steps) {
        turn_light_off(step_to_note_number(*step_removed));
    }
}

fn step_to_note_number(step: Step) -> i32 {
    step.measure.0 + 35
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
