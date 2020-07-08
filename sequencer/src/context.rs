use crate::event::Event;
use crate::input::Message;
use crate::sequence::Sequence;
use crate::sequence::Step;
use crate::step_sequencer::StepSequencer;
use std::mem;

#[derive(Debug, Clone)]
pub struct Context {
    pub step_sequencer: StepSequencer,
    pub sequences: Vec<Sequence>,
    pub selected_sequence: usize,
    pub bpm: f32,
    pub mode: Mode,
    pub tick: i32,
    pub shift: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    StepEdit,
    Step,
}

impl Context {
    pub fn default() -> Context {
        let mut sequences = vec![];
        for n in 36..=51 {
            sequences.push(Sequence::with_root_note(n));
        }

        Context {
            step_sequencer: StepSequencer::empty().toggle_sixteenth(1),
            sequences: sequences,
            selected_sequence: 1,
            bpm: 120.0,
            mode: Mode::Step,
            tick: 0,
            shift: false,
        }
    }

    pub fn process_messages(&self, messages: Vec<Message>) -> Context {
        match messages.len() {
            0 => self.clone(),
            _ => {
                let mut this_messages = messages.clone();
                let first_message = this_messages.remove(0);
                let mut new_context = self.process_message(&first_message);

                for message in this_messages {
                    new_context = new_context.process_message(&message);
                }

                new_context
            }
        }
    }

    pub fn advance_tick(&self) -> Context {
        let mut new_context = self.clone();
        new_context.tick += 1;
        new_context
    }

    pub fn events(&self) -> Vec<Event> {
        self.events_for_tick(self.tick)
    }

    fn events_for_tick(&self, tick_number: i32) -> Vec<Event> {
        let mut events = vec![];
        let mut step_sequencer_events = self.step_sequencer.events_for_tick(tick_number);
        events.append(&mut step_sequencer_events);
        events
    }

    fn select_sequence(&self, sequence_number: usize) -> Context {
        Context {
            selected_sequence: sequence_number,
            mode: Mode::StepEdit,
            ..self.clone()
        }
    }

    fn mute_sequence(&self, sequence_number: usize) -> Context {
        let mut sequences = self.sequences.clone();
        let muted_sequence = self.sequences[sequence_number].toggle_mute();
        mem::replace(&mut sequences[sequence_number], muted_sequence);

        Context {
            sequences: sequences,
            ..self.clone()
        }
    }

    fn toggle_step_for_selected_sequence(&self, step_number: i32) -> Context {
        let mut sequences = self.sequences.clone();
        let new_sequence = self.sequences[self.selected_sequence].toggle_step(Step(step_number));
        mem::replace(&mut sequences[self.selected_sequence], new_sequence);

        Context {
            sequences: sequences,
            ..self.clone()
        }
    }

    #[allow(dead_code)]
    fn set_step_sequencer(&self, step_sequencer: StepSequencer) -> Context {
        Context {
            step_sequencer: step_sequencer,
            ..self.clone()
        }
    }

    #[allow(dead_code)]
    fn set_mode(&self, mode: Mode) -> Context {
        Context {
            mode: mode,
            ..self.clone()
        }
    }

    fn set_shift(&self, value: bool) -> Context {
        Context {
            shift: value,
            ..self.clone()
        }
    }

    fn process_message(&self, message: &Message) -> Context {
        match self.mode {
            Mode::StepEdit => match message {
                Message::NoteOn { note_number: n } => {
                    self.toggle_step_for_selected_sequence(note_number_to_sixteenth(*n))
                }
                _ => self.clone(),
            },
            Mode::Step => match message {
                Message::ShiftOn => self.set_shift(true),
                Message::ShiftOff => self.set_shift(false),
                Message::NoteOn { note_number: n } => match self.shift {
                    false => self.select_sequence(note_number_to_sequence(*n)),
                    true => self.mute_sequence(note_number_to_sequence(*n)),
                },
                Message::KnobIncrement { number: 1 } => Context {
                    bpm: (self.bpm + 1.0).min(240.0),
                    ..self.clone()
                },
                Message::KnobDecrement { number: 1 } => Context {
                    bpm: (self.bpm - 1.0).max(30.0),
                    ..self.clone()
                },
                _ => self.clone(),
            },
        }
    }
}

fn note_number_to_sixteenth(note_number: i32) -> i32 {
    note_number - 35
}

fn note_number_to_sequence(note_number: i32) -> usize {
    (note_number - 35) as usize
}

#[test]
fn test_events() {
    let step_sequencer = StepSequencer::empty().toggle_sixteenth(2);
    let context = Context::default().set_step_sequencer(step_sequencer);

    let events = context.events_for_tick(6);
    assert_eq!(1, events.len());
    assert_eq!(36, events[0].note_number);
}

#[test]
fn test_advance_tick() {
    let context = Context::default();
    assert_eq!(0, context.tick);
    assert_eq!(1, context.advance_tick().tick);
}

#[test]
fn test_process_note_on_message_to_toggle_step() {
    // Sequence 3 corresponds to pad 4, mapped to MIDI note 39 by default
    let context = Context::default().select_sequence(3);
    let messages = vec![Message::NoteOn { note_number: 36 }];
    let processed_context = context.process_messages(messages);
    let sequence = &processed_context.sequences[3];

    assert_eq!(1, sequence.active_steps().len());
    assert_eq!(39, sequence.triggers_for_tick(0)[0].note_number)
}

#[test]
fn test_process_note_on_message_to_select_sequence() {
    let context = Context::default().set_mode(Mode::Step);
    let messages = vec![Message::NoteOn { note_number: 43 }];
    let processed_context = context.process_messages(messages);

    assert_eq!(8, processed_context.selected_sequence);
}

#[test]
fn test_process_note_on_message_to_mute_sequence() {
    let context = Context::default().set_mode(Mode::Step);
    let messages = vec![Message::ShiftOn, Message::NoteOn { note_number: 43 }];
    let processed_context = context.process_messages(messages);

    let muted_sequence = &processed_context.sequences[8];

    assert_eq!(true, muted_sequence.is_muted());
}

#[test]
fn test_process_knob_1_bpm_set_message() {
    let context = Context::default().set_mode(Mode::Step);

    let processed_context = context.process_messages(vec![Message::KnobIncrement { number: 1 }]);
    assert_eq!(121.0, processed_context.bpm);

    let processed_context = context.process_messages(vec![Message::KnobDecrement { number: 1 }]);
    assert_eq!(119.0, processed_context.bpm);
}
