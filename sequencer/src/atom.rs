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

    pub fn from_midi_note_number(note_number: i32) -> Pad {
        Pad::new(note_number - 35)
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

    match select_button_message(next_context) {
        Some(msg) => osc_messages.push(msg),
        None => {}
    }

    osc_messages
}

fn current_pad(context: &Context) -> Option<Pad> {
    match context.mode {
        Mode::StepEdit => {
            let current_position = context.step_sequencer.current_position(context.tick);
            Some(Pad::new(current_position))
        }
        Mode::Euclidean => {
            let current_position = context.euclidean_sequencer.current_position(context.tick);
            Some(Pad::new(current_position))
        }
        _ => None,
    }
}

fn active_pads(context: &Context) -> HashSet<Pad> {
    match context.mode {
        Mode::StepEdit => context
            .step_sequencer
            .active_sixteenths()
            .iter()
            .map(|s| Pad::new(*s))
            .collect(),
        Mode::Euclidean => context
            .euclidean_sequencer
            .active_sixteenths()
            .iter()
            .map(|s| Pad::new(*s))
            .collect(),
        Mode::Step | Mode::StepSelect => context
            .step_sequencer
            .active_notes(context.tick)
            .iter()
            .map(|n| Pad::from_midi_note_number(*n))
            .collect(),
        Mode::SequencerSelect => HashSet::new(),
    }
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

fn select_button_on_message() -> OscMessage {
    OscMessage {
        addr: message_to_addr("control_change".to_string()),
        args: vec![
            rosc::OscType::Int(1),
            rosc::OscType::Int(103),
            rosc::OscType::Int(127),
        ],
    }
}

fn select_button_off_message() -> OscMessage {
    OscMessage {
        addr: message_to_addr("control_change".to_string()),
        args: vec![
            rosc::OscType::Int(1),
            rosc::OscType::Int(103),
            rosc::OscType::Int(0),
        ],
    }
}

fn select_button_message(context: &Context) -> Option<OscMessage> {
    match context.mode {
        Mode::StepSelect => {
            if context.tick % 12 == 0 {
                Some(select_button_on_message())
            } else if (context.tick + 6) % 12 == 0 {
                Some(select_button_off_message())
            } else {
                None
            }
        }
        _ => {
            if context.tick % 12 == 0 {
                Some(select_button_off_message())
            } else {
                None
            }
        }
    }
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
    assert!(active_pads(&context).contains(&Pad::new(2)));
}

#[test]
fn test_active_pads_euclidean_sequencer() {
    let context = Context {
        euclidean_sequencer: EuclideanSequencer::empty().increment_onsets(),
        mode: Mode::Euclidean,
        ..Context::default()
    };

    assert_eq!(1, active_pads(&context).len());
    assert!(active_pads(&context).contains(&Pad::new(1)));
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
    assert!(active_pads(&context).contains(&Pad::new(1)));
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

#[test]
fn test_from_midi_note_number() {
    let pad = Pad::from_midi_note_number(36);
    assert_eq!(1, pad.number);
}

#[test]
fn test_select_button_message() {
    let context = Context {
        mode: Mode::StepSelect,
        tick: 0,
        ..Context::default()
    };
    let message = select_button_message(&context).unwrap();
    assert_eq!("/atom/control_change", message.addr);
    assert_eq!(rosc::OscType::Int(1), message.args[0]);
    assert_eq!(rosc::OscType::Int(103), message.args[1]);
    assert_eq!(rosc::OscType::Int(127), message.args[2]);

    let context = Context {
        mode: Mode::StepSelect,
        tick: 6,
        ..Context::default()
    };
    let message = select_button_message(&context).unwrap();
    assert_eq!("/atom/control_change", message.addr);
    assert_eq!(rosc::OscType::Int(1), message.args[0]);
    assert_eq!(rosc::OscType::Int(103), message.args[1]);
    assert_eq!(rosc::OscType::Int(0), message.args[2]);

    let context = Context {
        mode: Mode::StepSelect,
        tick: 12,
        ..Context::default()
    };
    let message = select_button_message(&context).unwrap();
    assert_eq!("/atom/control_change", message.addr);
    assert_eq!(rosc::OscType::Int(1), message.args[0]);
    assert_eq!(rosc::OscType::Int(103), message.args[1]);
    assert_eq!(rosc::OscType::Int(127), message.args[2]);

    let context = Context {
        mode: Mode::Step,
        tick: 1,
        ..Context::default()
    };
    let message = select_button_message(&context);
    assert!(message.is_none());

    let context = Context {
        mode: Mode::Step,
        tick: 0,
        ..Context::default()
    };
    let message = select_button_message(&context).unwrap();
    assert_eq!("/atom/control_change", message.addr);
    assert_eq!(rosc::OscType::Int(1), message.args[0]);
    assert_eq!(rosc::OscType::Int(103), message.args[1]);
    assert_eq!(rosc::OscType::Int(0), message.args[2]);
}
