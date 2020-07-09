use crate::config;
use crate::context::{Context, Mode};
use crate::output;
use rosc::OscMessage;
use std::collections::HashSet;

#[derive(Hash, Eq, Copy, Clone, Debug)]
struct Pad {
    number: i32,
}

impl PartialEq for Pad {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Pad {
    pub fn new(number: i32) -> Pad {
        Pad { number: number }
    }

    pub fn from_sequence_number(sequence_number: usize) -> Pad {
        Pad::new(sequence_number as i32 + 1)
    }

    pub fn note_number(&self) -> i32 {
        self.number + 35
    }

    fn turn_light_on_message(&self) -> OscMessage {
        OscMessage {
            addr: message_to_addr("note_on".to_string()),
            args: vec![
                rosc::OscType::Int(1),
                rosc::OscType::Int(self.note_number()),
                rosc::OscType::Int(127),
            ],
        }
    }

    fn turn_light_off_message(&self) -> OscMessage {
        OscMessage {
            addr: message_to_addr("note_on".to_string()),
            args: vec![
                rosc::OscType::Int(1),
                rosc::OscType::Int(self.note_number()),
                rosc::OscType::Int(0),
            ],
        }
    }
}

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
        Some(pad) => {
            current_context_active_pads.insert(pad);
        }
        None => {}
    }

    let mut next_context_active_pads = active_pads(next_context);

    match current_pad(next_context) {
        Some(pad) => {
            next_context_active_pads.insert(pad);
        }
        None => {}
    }

    let mut osc_messages = vec![];

    for pad_added in next_context_active_pads.difference(&current_context_active_pads) {
        osc_messages.push(pad_added.turn_light_on_message());
    }

    for pad_removed in current_context_active_pads.difference(&next_context_active_pads) {
        osc_messages.push(pad_removed.turn_light_off_message());
    }

    osc_messages
}

fn current_pad(context: &Context) -> Option<Pad> {
    match context.mode {
        Mode::SequenceEdit => {
            let current_step = context.selected_sequence().current_step(context.tick);
            Some(Pad::new(current_step.0))
        }
        _ => None,
    }
}

fn active_pads(context: &Context) -> HashSet<Pad> {
    match context.mode {
        Mode::SequenceEdit => context
            .selected_sequence()
            .active_steps()
            .iter()
            .map(|s| Pad::new(s.0))
            .collect(),
        Mode::Performance => active_sequences(context)
            .iter()
            .map(|i| Pad::from_sequence_number(*i))
            .collect(),
    }
}

fn active_sequences(context: &Context) -> HashSet<usize> {
    let mut active_sequences = HashSet::new();

    for (idx, sequence) in context.sequences.iter().enumerate() {
        if !sequence.events_for_tick(context.tick).is_empty() {
            active_sequences.insert(idx);
        }
    }

    active_sequences
}

fn turn_all_lights_off() {
    for n in 1..16 {
        output::send_osc_message_to_o2m(Pad::new(n).turn_light_off_message());
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

#[test]
fn test_active_pads_step_sequencer() {
    let context = Context::default()
        .select_sequence(1)
        .toggle_step_for_selected_sequence(2);

    assert_eq!(1, active_pads(&context).len());
    assert!(active_pads(&context).contains(&Pad::new(2)));
}

#[test]
fn test_active_pads_step_mode() {
    let context = Context::default()
        .select_sequence(0)
        .toggle_step_for_selected_sequence(1)
        .set_mode(Mode::Performance);

    // the first step of the first sequence is active so for tick 0
    // (the Context::default tick) active_pads should contain Pad 1
    assert_eq!(1, active_pads(&context).len());
    assert!(active_pads(&context).contains(&Pad::new(1)));
}

#[test]
fn test_active_sequences() {
    let context = Context::default()
        .select_sequence(0)
        .toggle_step_for_selected_sequence(1);

    // the first step of the first sequence is active so for tick 0
    // (the Context::default tick) active_sequences should contain 0
    assert_eq!(1, active_sequences(&context).len());
    assert!(active_sequences(&context).contains(&0));
}

#[test]
fn test_turn_light_on_message() {
    let message = Pad::new(1).turn_light_on_message();

    assert_eq!("/atom/note_on", message.addr);
    assert_eq!(rosc::OscType::Int(1), message.args[0]);
    assert_eq!(rosc::OscType::Int(36), message.args[1]);
    assert_eq!(rosc::OscType::Int(127), message.args[2]);
}

#[test]
fn test_turn_light_off_message() {
    let message = Pad::new(1).turn_light_off_message();

    assert_eq!("/atom/note_on", message.addr);
    assert_eq!(rosc::OscType::Int(1), message.args[0]);
    assert_eq!(rosc::OscType::Int(36), message.args[1]);
    assert_eq!(rosc::OscType::Int(0), message.args[2]);
}
