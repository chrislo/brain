use crate::config;
use crate::context::{Context, Mode};
use crate::output;
use rosc::OscMessage;
use std::collections::HashSet;

pub fn init() {
    handshake();
    turn_all_lights_off();
}

pub fn handshake() {
    let message = OscMessage {
        addr: message_to_addr("note_off".to_string()),
        args: vec![
            rosc::OscType::Int(16),
            rosc::OscType::Int(0),
            rosc::OscType::Int(127),
        ],
    };
    output::send_osc_message_to_o2m(message);
}

pub fn update(current_context: &Context, next_context: &Context) -> Vec<OscMessage> {
    let mut current_context_active_pads = active_pads(current_context);

    match current_pad(current_context) {
        Some(n) => {
            current_context_active_pads.insert(n);
        }
        None => {}
    }

    let mut next_context_active_pads = active_pads(next_context);

    match current_pad(next_context) {
        Some(n) => {
            next_context_active_pads.insert(n);
        }
        None => {}
    }

    let mut osc_messages = vec![];

    for pad_added in next_context_active_pads.difference(&current_context_active_pads) {
        osc_messages.push(turn_light_on_message(*pad_added));
    }

    for pad_removed in current_context_active_pads.difference(&next_context_active_pads) {
        osc_messages.push(turn_light_off_message(*pad_removed));
    }

    osc_messages
}

fn current_pad(context: &Context) -> Option<i32> {
    match context.mode {
        Mode::StepEdit => {
            let current_position = context.step_sequencer.current_position(context.tick);
            Some(sixteenth_to_note_number(current_position))
        }
        Mode::Euclidean => {
            let current_position = context.euclidean_sequencer.current_position(context.tick);
            Some(sixteenth_to_note_number(current_position))
        }
        _ => None,
    }
}

fn active_pads(context: &Context) -> HashSet<i32> {
    match context.mode {
        Mode::StepEdit => context
            .step_sequencer
            .active_sixteenths()
            .iter()
            .map(|s| sixteenth_to_note_number(*s))
            .collect(),
        Mode::Euclidean => context
            .euclidean_sequencer
            .active_sixteenths()
            .iter()
            .map(|s| sixteenth_to_note_number(*s))
            .collect(),
        Mode::Step | Mode::StepSelect => context.step_sequencer.active_notes(context.tick),
    }
}

fn sixteenth_to_note_number(sixteenth: i32) -> i32 {
    sixteenth + 35
}

fn turn_all_lights_off() {
    for n in 36..52 {
        output::send_osc_message_to_o2m(turn_light_off_message(n));
    }
}

fn turn_light_on_message(note_number: i32) -> OscMessage {
    OscMessage {
        addr: message_to_addr("note_on".to_string()),
        args: vec![
            rosc::OscType::Int(1),
            rosc::OscType::Int(note_number),
            rosc::OscType::Int(127),
        ],
    }
}

fn turn_light_off_message(note_number: i32) -> OscMessage {
    OscMessage {
        addr: message_to_addr("note_on".to_string()),
        args: vec![
            rosc::OscType::Int(1),
            rosc::OscType::Int(note_number),
            rosc::OscType::Int(0),
        ],
    }
}

fn message_to_addr(message: String) -> String {
    let controller_addr = config::controller_addr();
    format!("/{}/{}", controller_addr, message)
}

#[test]
fn test_message_to_addr() {
    assert_eq!("/atom/note_on", message_to_addr("note_on".to_string()));
}

#[cfg(test)]
use crate::step_sequencer::StepSequencer;

#[cfg(test)]
use crate::euclidean_sequencer::EuclideanSequencer;

#[test]
fn test_active_pads_step_sequencer() {
    let context = Context {
        step_sequencer: StepSequencer::empty().toggle_sixteenth(2),
        mode: Mode::StepEdit,
        ..Context::default()
    };

    assert_eq!(1, active_pads(&context).len());
    assert!(active_pads(&context).contains(&37));
}

#[test]
fn test_active_pads_euclidean_sequencer() {
    let context = Context {
        euclidean_sequencer: EuclideanSequencer::empty().increment_onsets(),
        mode: Mode::Euclidean,
        ..Context::default()
    };

    assert_eq!(1, active_pads(&context).len());
    assert!(active_pads(&context).contains(&36));
}

#[test]
fn test_active_pads_step_mode() {
    let context = Context {
        step_sequencer: StepSequencer::empty().toggle_sixteenth(1),
        mode: Mode::Step,
        ..Context::default()
    };

    // the first sixteenth is active, so in Step mode pad 1 should
    // flash on tick 0
    assert_eq!(1, active_pads(&context).len());
    assert!(active_pads(&context).contains(&36));
}

#[test]
fn test_turn_light_on_message() {
    let message = turn_light_on_message(35);

    assert_eq!("/atom/note_on", message.addr);
    assert_eq!(rosc::OscType::Int(1), message.args[0]);
    assert_eq!(rosc::OscType::Int(35), message.args[1]);
    assert_eq!(rosc::OscType::Int(127), message.args[2]);
}

#[test]
fn test_turn_light_off_message() {
    let message = turn_light_off_message(35);

    assert_eq!("/atom/note_on", message.addr);
    assert_eq!(rosc::OscType::Int(1), message.args[0]);
    assert_eq!(rosc::OscType::Int(35), message.args[1]);
    assert_eq!(rosc::OscType::Int(0), message.args[2]);
}
