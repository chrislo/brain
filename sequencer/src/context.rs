use crate::control::Message;
use crate::measure::Measure;
use crate::track::Step;
use crate::track::Track;

#[derive(Clone)]
pub struct Context {
    pub track: Track,
    pub active_note_number: i32,
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

    fn process_message(&self, message: &Message) -> Context {
        match message {
            Message::NoteOn { note_number: n } => {
                let new_track = self
                    .track
                    .toggle_step(note_number_to_step(*n, self.active_note_number));
                Context {
                    track: new_track,
                    active_note_number: self.active_note_number,
                }
            }
            _ => self.clone(),
        }
    }
}

fn note_number_to_step(note_number: i32, active_note_number: i32) -> Step {
    Step {
        measure: Measure(note_number - 35, 16),
        note_number: active_note_number,
    }
}

#[test]
fn test_process_message() {
    let context = Context {
        track: Track::empty(),
        active_note_number: 2,
    };
    let messages = vec![Message::NoteOn { note_number: 43 }];

    let processed_context = context.process_messages(messages);

    let event = processed_context
        .track
        .events_between(Measure(1, 4), Measure(4, 4))[0];

    assert_eq!(Measure(8, 16), event.start);
    assert_eq!(2, event.note_number);
}

#[test]
fn test_process_two_messages() {
    let context = Context {
        track: Track::empty(),
        active_note_number: 1,
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
            .events_between(Measure(1, 4), Measure(4, 4))
            .len()
    );
}
