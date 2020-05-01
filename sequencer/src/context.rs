use crate::control::Message;
use crate::event::Event;
use crate::track::Track;

#[derive(Debug, Clone)]
pub struct Context {
    pub track: Track,
    pub active_note_number: i32,
    pub swing_amount: i32,
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
        if self.swing_amount > 0 {
            if even_sixteenth(tick_number) {
                vec![]
            } else if even_sixteenth(tick_number - self.swing_amount) {
                self.track.events_for_tick(tick_number - self.swing_amount)
            } else {
                self.track.events_for_tick(tick_number)
            }
        } else {
            self.track.events_for_tick(tick_number)
        }
    }

    fn process_message(&self, message: &Message) -> Context {
        match message {
            Message::NoteOn { note_number: n } => {
                let new_track = self
                    .track
                    .toggle_sixteenth(note_number_to_sixteenth(*n), self.active_note_number);
                Context {
                    track: new_track,
                    active_note_number: self.active_note_number,
                    swing_amount: self.swing_amount,
                }
            }
            Message::Left => Context {
                track: self.track.clone(),
                active_note_number: self.active_note_number - 1,
                swing_amount: self.swing_amount,
            },
            Message::Right => Context {
                track: self.track.clone(),
                active_note_number: self.active_note_number + 1,
                swing_amount: self.swing_amount,
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
    let track = Track::empty().toggle_sixteenth(2, 1);

    let context = Context {
        track: track,
        active_note_number: 1,
        swing_amount: 0,
    };

    let events = context.events(6);
    assert_eq!(1, events.len());
    assert_eq!(1, events[0].note_number);
}

#[test]
fn test_events_with_swing() {
    let track = Track::empty().toggle_sixteenth(2, 1);
    let swing_amount = 2;

    let context = Context {
        track: track,
        active_note_number: 1,
        swing_amount: swing_amount,
    };

    let events = context.events(6);
    assert_eq!(0, events.len());

    let events = context.events(6 + swing_amount);
    assert_eq!(1, events.len());
    assert_eq!(1, events[0].note_number);
}

#[test]
fn test_process_note_on_message() {
    let context = Context {
        track: Track::empty(),
        active_note_number: 2,
        swing_amount: 0,
    };
    let messages = vec![Message::NoteOn { note_number: 43 }];

    let processed_context = context.process_messages(messages);

    let active_sixteenths = processed_context
        .track
        .active_sixteenths_with_note_number(2);
    assert_eq!(1, active_sixteenths.len());
}

#[test]
fn test_process_left_message() {
    let context = Context {
        track: Track::empty(),
        active_note_number: 1,
        swing_amount: 0,
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
        track: Track::empty(),
        active_note_number: 1,
        swing_amount: 0,
    };

    let processed_context = context.process_messages(vec![Message::Right]);
    assert_eq!(2, processed_context.active_note_number);
}

#[test]
fn test_process_two_messages() {
    let context = Context {
        track: Track::empty(),
        active_note_number: 1,
        swing_amount: 0,
    };
    let messages = vec![
        Message::NoteOn { note_number: 42 },
        Message::NoteOn { note_number: 43 },
    ];

    let processed_context = context.process_messages(messages);

    assert_eq!(
        2,
        processed_context
            .track
            .active_sixteenths_with_note_number(1)
            .len()
    );
}
