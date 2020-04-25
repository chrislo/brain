use crate::control::Message;
use crate::measure::Measure;
use crate::track::Track;

#[derive(Clone)]
pub struct Context {
    pub track: Track,
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
                    new_context = self.process_message(&message);
                }

                new_context
            }
        }
    }

    fn process_message(&self, message: &Message) -> Context {
        match message {
            Message::NoteOn { note_number: n } => {
                let new_track = self.track.toggle_step(note_number_to_measure(*n));
                Context { track: new_track }
            }
            _ => self.clone(),
        }
    }
}

fn note_number_to_measure(note_number: i32) -> Measure {
    Measure(note_number - 35, 16)
}

#[test]
fn test_process_messages() {
    let context = Context {
        track: Track::empty(),
    };
    let messages = vec![Message::NoteOn { note_number: 43 }];

    let processed_context = context.process_messages(messages);
    assert_eq!(
        1,
        processed_context
            .track
            .events_between(Measure(1, 4), Measure(4, 4))
            .len()
    );
}
