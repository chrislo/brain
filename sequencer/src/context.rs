use crate::event::Event;
use crate::input::Message;
use crate::step_sequencer::StepSequencer;

#[derive(Debug, Clone)]
pub struct Context {
    pub step_sequencer: StepSequencer,
    pub active_note_number: i32,
    pub swing_amount: i32,
    pub bpm: f32,
}

impl Context {
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

    pub fn events(&self, tick_number: i32) -> Vec<Event> {
        let swing_amount = percentage_swing_to_ticks(self.swing_amount);
        if swing_amount > 0 {
            if even_sixteenth(tick_number) {
                vec![]
            } else if even_sixteenth(tick_number - swing_amount) {
                self.step_sequencer
                    .events_for_tick(tick_number - swing_amount)
            } else {
                self.step_sequencer.events_for_tick(tick_number)
            }
        } else {
            self.step_sequencer.events_for_tick(tick_number)
        }
    }

    fn process_message(&self, message: &Message) -> Context {
        match message {
            Message::NoteOn { note_number: n } => {
                let new_step_sequencer = self
                    .step_sequencer
                    .toggle_sixteenth(note_number_to_sixteenth(*n), self.active_note_number);
                Context {
                    step_sequencer: new_step_sequencer,
                    active_note_number: self.active_note_number,
                    swing_amount: self.swing_amount,
                    bpm: self.bpm,
                }
            }
            Message::Left => Context {
                step_sequencer: self.step_sequencer.clone(),
                active_note_number: self.active_note_number - 1,
                swing_amount: self.swing_amount,
                bpm: self.bpm,
            },
            Message::Right => Context {
                step_sequencer: self.step_sequencer.clone(),
                active_note_number: self.active_note_number + 1,
                swing_amount: self.swing_amount,
                bpm: self.bpm,
            },
            Message::KnobIncrement { number: 1 } => Context {
                step_sequencer: self.step_sequencer.clone(),
                active_note_number: self.active_note_number,
                swing_amount: self.swing_amount,
                bpm: (self.bpm + 1.0).min(240.0),
            },
            Message::KnobDecrement { number: 1 } => Context {
                step_sequencer: self.step_sequencer.clone(),
                active_note_number: self.active_note_number,
                swing_amount: std::cmp::max(self.swing_amount - 1, 0),
                bpm: (self.bpm - 1.0).max(30.0),
            },
            Message::KnobIncrement { number: 2 } => Context {
                step_sequencer: self.step_sequencer.clone(),
                active_note_number: self.active_note_number,
                swing_amount: std::cmp::min(self.swing_amount + 1, 100),
                bpm: self.bpm,
            },
            Message::KnobDecrement { number: 2 } => Context {
                step_sequencer: self.step_sequencer.clone(),
                active_note_number: self.active_note_number,
                swing_amount: std::cmp::max(self.swing_amount - 1, 0),
                bpm: self.bpm,
            },
            _ => self.clone(),
        }
    }
}

fn note_number_to_sixteenth(note_number: i32) -> i32 {
    note_number - 35
}

fn even_sixteenth(tick_number: i32) -> bool {
    (((tick_number - 6) % 96) % 12) == 0
}

fn percentage_swing_to_ticks(swing_percentage: i32) -> i32 {
    // Scale swing ticks between 0 and 6
    let max_ticks = 6.;
    (swing_percentage as f64 / (100. / max_ticks)).floor() as i32
}

#[test]
fn test_even_sixteenth() {
    let even_sixteenths = vec![6, 18, 30, 42, 54, 66, 78, 90, 102];
    for tick_number in even_sixteenths {
        assert!(even_sixteenth(tick_number));
    }

    assert!(!even_sixteenth(0));
    assert!(!even_sixteenth(1));
    assert!(!even_sixteenth(95));
    assert!(!even_sixteenth(96));
    assert!(!even_sixteenth(97));
}

#[test]
fn test_events() {
    let step_sequencer = StepSequencer::empty().toggle_sixteenth(2, 1);

    let context = Context {
        step_sequencer: step_sequencer,
        active_note_number: 1,
        swing_amount: 0,
        bpm: 120.0,
    };

    let events = context.events(6);
    assert_eq!(1, events.len());
    assert_eq!(1, events[0].note_number);
}

#[test]
fn test_events_with_swing() {
    let step_sequencer = StepSequencer::empty().toggle_sixteenth(2, 1);
    let swing_amount = 20;

    let context = Context {
        step_sequencer: step_sequencer,
        active_note_number: 1,
        swing_amount: swing_amount,
        bpm: 120.0,
    };

    let events = context.events(6);
    assert_eq!(0, events.len());

    let events = context.events(6 + percentage_swing_to_ticks(swing_amount));
    assert_eq!(1, events.len());
    assert_eq!(1, events[0].note_number);
}

#[test]
fn test_process_note_on_message() {
    let context = Context {
        step_sequencer: StepSequencer::empty(),
        active_note_number: 2,
        swing_amount: 0,
        bpm: 120.0,
    };
    let messages = vec![Message::NoteOn { note_number: 43 }];

    let processed_context = context.process_messages(messages);

    let active_sixteenths = processed_context
        .step_sequencer
        .active_sixteenths_with_note_number(2);
    assert_eq!(1, active_sixteenths.len());
}

#[test]
fn test_process_left_message() {
    let context = Context {
        step_sequencer: StepSequencer::empty(),
        active_note_number: 1,
        swing_amount: 0,
        bpm: 120.0,
    };

    let processed_context = context.process_messages(vec![Message::Left]);
    assert_eq!(0, processed_context.active_note_number);

    // We don't allow the number to go below zero
    let processed_context = context.process_messages(vec![Message::Left]);
    assert_eq!(0, processed_context.active_note_number);
}

#[test]
fn test_process_right_message() {
    let context = Context {
        step_sequencer: StepSequencer::empty(),
        active_note_number: 1,
        swing_amount: 0,
        bpm: 120.0,
    };

    let processed_context = context.process_messages(vec![Message::Right]);
    assert_eq!(2, processed_context.active_note_number);
}

#[test]
fn test_process_two_messages() {
    let context = Context {
        step_sequencer: StepSequencer::empty(),
        active_note_number: 1,
        swing_amount: 0,
        bpm: 120.0,
    };
    let messages = vec![
        Message::NoteOn { note_number: 42 },
        Message::NoteOn { note_number: 43 },
    ];

    let processed_context = context.process_messages(messages);

    assert_eq!(
        2,
        processed_context
            .step_sequencer
            .active_sixteenths_with_note_number(1)
            .len()
    );
}

#[test]
fn test_process_knob_1_bpm_set_message() {
    let context = Context {
        step_sequencer: StepSequencer::empty(),
        active_note_number: 1,
        swing_amount: 0,
        bpm: 120.0,
    };

    let processed_context = context.process_messages(vec![Message::KnobIncrement { number: 1 }]);
    assert_eq!(121.0, processed_context.bpm);

    let processed_context = context.process_messages(vec![Message::KnobDecrement { number: 1 }]);
    assert_eq!(119.0, processed_context.bpm);
}

#[test]
fn test_process_knob_2_swing_set_message() {
    let context = Context {
        step_sequencer: StepSequencer::empty(),
        active_note_number: 1,
        swing_amount: 0,
        bpm: 120.0,
    };

    let processed_context = context.process_messages(vec![Message::KnobIncrement { number: 2 }]);
    assert_eq!(1, processed_context.swing_amount);

    let processed_context = context.process_messages(vec![Message::KnobDecrement { number: 2 }]);
    assert_eq!(0, processed_context.swing_amount);
}
