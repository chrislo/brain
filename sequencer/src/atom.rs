use crate::config;
use crate::context::Context;
use crate::output;
use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::collections::HashSet;

pub fn init() {
    handshake();
    turn_all_lights_off();
}

pub fn handshake() {
    let packet = encoder::encode(&OscPacket::Message(OscMessage {
        addr: message_to_addr("note_off".to_string()),
        args: vec![
            rosc::OscType::Int(16),
            rosc::OscType::Int(0),
            rosc::OscType::Int(127),
        ],
    }))
    .unwrap();

    output::send_osc_to_o2m(packet);
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
        .step_sequencer
        .active_sixteenths()
        .iter()
        .map(|s| sixteenth_to_note_number(*s))
        .collect()
}

fn sixteenth_to_note_number(sixteenth: i32) -> i32 {
    sixteenth + 35
}

fn turn_all_lights_off() {
    for n in 36..52 {
        turn_light_off(n);
    }
}

fn turn_light_on(note_number: i32) {
    let packet = encoder::encode(&OscPacket::Message(OscMessage {
        addr: message_to_addr("note_on".to_string()),
        args: vec![
            rosc::OscType::Int(1),
            rosc::OscType::Int(note_number),
            rosc::OscType::Int(127),
        ],
    }))
    .unwrap();

    output::send_osc_to_o2m(packet);
}

fn turn_light_off(note_number: i32) {
    let packet = encoder::encode(&OscPacket::Message(OscMessage {
        addr: message_to_addr("note_on".to_string()),
        args: vec![
            rosc::OscType::Int(1),
            rosc::OscType::Int(note_number),
            rosc::OscType::Int(0),
        ],
    }))
    .unwrap();

    output::send_osc_to_o2m(packet);
}

fn message_to_addr(message: String) -> String {
    let controller_addr = config::controller_addr();
    format!("/{}/{}", controller_addr, message)
}

#[cfg(test)]
use crate::input::Message;

#[test]
fn test_message_to_addr() {
    assert_eq!("/atom/note_on", message_to_addr("note_on".to_string()));
}

#[test]
fn test_active_pads() {
    let context = Context::default();

    let messages = vec![Message::NoteOn { note_number: 37 }];
    let processed_context = context.process_messages(messages);

    assert_eq!(1, active_pads(&processed_context).len());
    assert!(active_pads(&processed_context).contains(&37));

    // active_note_number was 1 when steps added, so no pads active
    // when we increment the active_note_number
    let messages = vec![Message::Right];
    let processed_context = processed_context.process_messages(messages);
    assert_eq!(0, active_pads(&processed_context).len());
}
