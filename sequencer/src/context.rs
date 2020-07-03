use crate::event::Event;
use crate::input::Message;
use crate::sequence::Sequence;
use crate::step_sequencer::StepSequencer;

#[derive(Debug, Clone)]
pub struct Context {
    pub step_sequencer: StepSequencer,
    pub sequences: Vec<Sequence>,
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
        Context {
            step_sequencer: StepSequencer::empty().toggle_sixteenth(1),
            sequences: vec![Sequence::empty(); 16],
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

    fn edit_step(&self, note_number: i32) -> Context {
        let new_step_sequencer = self.step_sequencer.set_active_note_number(note_number);

        Context {
            step_sequencer: new_step_sequencer,
            mode: Mode::StepEdit,
            ..self.clone()
        }
    }

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
                    let new_step_sequencer = self
                        .step_sequencer
                        .toggle_sixteenth(note_number_to_sixteenth(*n));
                    self.set_step_sequencer(new_step_sequencer)
                }
                _ => self.clone(),
            },
            Mode::Step => match message {
                Message::ShiftOn => self.set_shift(true),
                Message::ShiftOff => self.set_shift(false),
                Message::NoteOn { note_number: n } => match self.shift {
                    false => self.edit_step(*n),
                    true => {
                        let new_sequencer = self.step_sequencer.toggle_mute(*n);
                        self.set_step_sequencer(new_sequencer)
                    }
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
fn test_process_note_on_message() {
    let sequencer = StepSequencer::empty();
    let context = Context::default()
        .set_mode(Mode::StepEdit)
        .set_step_sequencer(sequencer);
    let messages = vec![Message::NoteOn { note_number: 43 }];

    let processed_context = context.process_messages(messages);

    let active_sixteenths = processed_context.step_sequencer.active_sixteenths();
    assert_eq!(1, active_sixteenths.len());
}

#[test]
fn test_process_two_messages() {
    let sequencer = StepSequencer::empty();
    let context = Context::default()
        .set_mode(Mode::StepEdit)
        .set_step_sequencer(sequencer);
    let messages = vec![
        Message::NoteOn { note_number: 42 },
        Message::NoteOn { note_number: 43 },
    ];

    let processed_context = context.process_messages(messages);

    assert_eq!(
        2,
        processed_context.step_sequencer.active_sixteenths().len()
    );
}

#[test]
fn test_process_knob_1_bpm_set_message() {
    let context = Context::default().set_mode(Mode::Step);

    let processed_context = context.process_messages(vec![Message::KnobIncrement { number: 1 }]);
    assert_eq!(121.0, processed_context.bpm);

    let processed_context = context.process_messages(vec![Message::KnobDecrement { number: 1 }]);
    assert_eq!(119.0, processed_context.bpm);
}
