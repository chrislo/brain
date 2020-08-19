use crate::event::Event;
use crate::input::Message;
use crate::sequence::Sequence;
use crate::sequence::Step;

#[derive(Debug, Clone)]
pub struct Context {
    pub sequences: Vec<Sequence>,
    pub selected_sequence: usize,
    pub performance_events: Vec<Event>,
    pub bpm: f32,
    pub mode: Mode,
    pub tick: i32,
}

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    SequenceEdit,
    SequenceMute,
    SequenceSelect,
    Performance,
}

impl Context {
    pub fn default() -> Context {
        let mut sequences = vec![];
        for n in 36..=51 {
            sequences.push(Sequence::with_default_note_number(n));
        }

        Context {
            sequences,
            selected_sequence: 0,
            performance_events: vec![],
            bpm: 120.0,
            mode: Mode::Performance,
            tick: 0,
        }
    }

    pub fn process_messages(&self, messages: Vec<Message>) -> Context {
        match messages.len() {
            0 => self.clone(),
            _ => {
                let mut this_messages = messages;
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

    pub fn clear_performance_events(&self) -> Context {
        let mut new_context = self.clone();
        new_context.performance_events = vec![];
        new_context
    }

    pub fn events(&self) -> Vec<Event> {
        let mut events = self.events_for_tick(self.tick);

        for event in &self.performance_events {
            events.push(*event);
        }

        events
    }

    fn events_for_tick(&self, tick_number: i32) -> Vec<Event> {
        self.sequences
            .iter()
            .flat_map(|s| s.events_for_tick(tick_number))
            .collect()
    }

    pub fn select_sequence(&self, sequence_number: usize) -> Context {
        Context {
            selected_sequence: sequence_number,
            mode: Mode::SequenceEdit,
            ..self.clone()
        }
    }

    pub fn selected_sequence(&self) -> &Sequence {
        &self.sequences[self.selected_sequence]
    }

    fn trigger_default_note(&self, sequence_number: usize) -> Context {
        let mut performance_events = self.performance_events.clone();
        let event = self.sequences[sequence_number].default_event();
        performance_events.push(event);

        Context {
            performance_events,
            ..self.clone()
        }
    }

    fn mute_sequence(&self, sequence_number: usize) -> Context {
        let mut sequences = self.sequences.clone();
        let muted_sequence = self.sequences[sequence_number].toggle_mute();
        sequences[sequence_number] = muted_sequence;

        Context {
            sequences,
            ..self.clone()
        }
    }

    pub fn toggle_step_for_selected_sequence(&self, step_number: i32) -> Context {
        let mut sequences = self.sequences.clone();
        let new_sequence = self.sequences[self.selected_sequence].toggle_step(Step(step_number));
        sequences[self.selected_sequence] = new_sequence;

        Context {
            sequences,
            ..self.clone()
        }
    }

    fn change_selected_sequence<F>(&self, f: F) -> Context
    where
        F: Fn(&Sequence) -> Sequence,
    {
        let mut sequences = self.sequences.clone();
        let new_sequence = f(&self.sequences[self.selected_sequence]);
        sequences[self.selected_sequence] = new_sequence;

        Context {
            sequences,
            ..self.clone()
        }
    }

    pub fn set_mode(&self, mode: Mode) -> Context {
        Context {
            mode,
            ..self.clone()
        }
    }

    fn process_message(&self, message: &Message) -> Context {
        match self.mode {
            Mode::SequenceEdit => match message {
                Message::NoteOn { note_number: n } => {
                    self.toggle_step_for_selected_sequence(note_number_to_sixteenth(*n))
                }
                Message::KnobIncrement { number: 1 } => {
                    self.change_selected_sequence(Sequence::increment_length)
                }
                Message::KnobDecrement { number: 1 } => {
                    self.change_selected_sequence(Sequence::decrement_length)
                }
                Message::KnobIncrement { number: 2 } => {
                    self.change_selected_sequence(Sequence::increment_euclidean_fill)
                }
                Message::KnobDecrement { number: 2 } => {
                    self.change_selected_sequence(Sequence::decrement_euclidean_fill)
                }
                Message::KnobIncrement { number: 3 } => {
                    self.change_selected_sequence(Sequence::increment_rotate)
                }
                Message::KnobDecrement { number: 3 } => {
                    self.change_selected_sequence(Sequence::decrement_rotate)
                }
                Message::SelectOn => self.set_mode(Mode::Performance),
                _ => self.clone(),
            },
            Mode::SequenceMute => match message {
                Message::NoteOn { note_number: n } => {
                    self.mute_sequence(note_number_to_sequence(*n))
                }
                Message::ShiftOff => self.set_mode(Mode::Performance),
                _ => self.clone(),
            },
            Mode::SequenceSelect => match message {
                Message::NoteOn { note_number: n } => {
                    self.select_sequence(note_number_to_sequence(*n))
                }
                Message::SelectOff => self.set_mode(Mode::Performance),
                _ => self.clone(),
            },
            Mode::Performance => match message {
                Message::NoteOn { note_number: n } => {
                    self.trigger_default_note(note_number_to_sequence(*n))
                }
                Message::ShiftOn => self.set_mode(Mode::SequenceMute),
                Message::SelectOn => self.set_mode(Mode::SequenceSelect),
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
    (note_number - 36) as usize
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

    assert_eq!(39, processed_context.events_for_tick(0)[0].note_number)
}

#[test]
fn test_process_note_on_message_to_select_sequence() {
    let context = Context::default().set_mode(Mode::Performance);
    let messages = vec![Message::SelectOn, Message::NoteOn { note_number: 43 }];
    let processed_context = context.process_messages(messages);

    assert_eq!(7, processed_context.selected_sequence);
}

#[test]
fn test_process_note_on_message_to_mute_sequence() {
    let context = Context::default().set_mode(Mode::Performance);
    let messages = vec![Message::ShiftOn, Message::NoteOn { note_number: 43 }];
    let processed_context = context.process_messages(messages);

    let muted_sequence = &processed_context.sequences[7];

    assert_eq!(true, muted_sequence.is_muted());
}

#[test]
fn test_process_knob_1_bpm_set_message() {
    let context = Context::default().set_mode(Mode::Performance);

    let processed_context = context.process_messages(vec![Message::KnobIncrement { number: 1 }]);
    assert_eq!(121.0, processed_context.bpm);

    let processed_context = context.process_messages(vec![Message::KnobDecrement { number: 1 }]);
    assert_eq!(119.0, processed_context.bpm);
}

#[test]
fn test_trigger_default_note() {
    let context = Context::default();
    let events = context.trigger_default_note(0).events();

    assert_eq!(1, events.len());
    assert_eq!(36, events[0].note_number);
}

#[test]
fn test_clear_performance_events() {
    let context = Context::default().trigger_default_note(0);

    assert_eq!(1, context.events().len());
    assert_eq!(0, context.clear_performance_events().events().len());
}
