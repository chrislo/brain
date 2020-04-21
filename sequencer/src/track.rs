use crate::control::Message;
use crate::event::Event;
use crate::measure::Measure;

#[derive(Debug, Clone)]
pub struct Track {
    events: Vec<Event>,
}

impl Track {
    pub fn empty() -> Track {
        Track { events: vec![] }
    }

    pub fn events_between(&self, start: Measure, end: Measure) -> Vec<Event> {
        let start_float = start.reduce_to_one_bar().to_float();
        let end_float = end.reduce_to_one_bar().to_float();

        self.events
            .clone()
            .into_iter()
            .filter(|e| e.start.to_float() > start_float && e.start.to_float() <= end_float)
            .collect::<Vec<Event>>()
    }

    pub fn process_messages(&self, mut messages: Vec<Message>) -> Track {
        match messages.len() {
            0 => self.clone(),
            _ => {
                let first_message = messages.remove(0);
                let mut result = self.process_message(&first_message);

                for message in messages {
                    result = result.process_message(&message);
                }

                result
            }
        }
    }

    fn process_message(&self, message: &Message) -> Track {
        match message {
            Message::NoteOn { note_number: n } => self.toggle_step(note_number_to_measure(*n)),
            Message::Unhandled => self.clone(),
        }
    }

    fn toggle_step(&self, measure: Measure) -> Track {
        let event = Event { start: measure };
        let mut new_track = self.clone();

        if !new_track.events.contains(&event) {
            new_track.events.push(event);
        } else {
            new_track.events.retain(|e| *e != event);
        }
        new_track
    }
}

fn note_number_to_measure(note_number: i32) -> Measure {
    Measure(note_number - 35, 16)
}

#[test]
fn test_events_between() {
    let track = Track::empty().toggle_step(Measure(2, 16));

    let events = track.events_between(Measure(1, 16), Measure(3, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let events = track.events_between(Measure(3, 16), Measure(4, 16));
    assert!(events.is_empty());

    let events = track.events_between(Measure(17, 16), Measure(19, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let track = Track::empty().toggle_step(Measure(1, 16));
    let events = track.events_between(Measure(1, 32), Measure(2, 32));
    assert_eq!(Measure(1, 16), events[0].start);
}

#[test]
fn test_process_message() {
    let track = Track::empty();
    let message = Message::NoteOn { note_number: 43 };

    let processed_track = track.process_message(&message);
    assert_eq!(
        1,
        processed_track
            .events_between(Measure(1, 4), Measure(4, 4))
            .len()
    );
}

#[test]
fn test_process_messages() {
    let track = Track::empty();
    let message1 = Message::NoteOn { note_number: 43 };
    let message2 = Message::NoteOn { note_number: 47 };
    let processed_track = track.process_messages(vec![message1, message2]);
    assert_eq!(
        2,
        processed_track
            .events_between(Measure(1, 4), Measure(4, 4))
            .len()
    );
}

#[test]
fn test_process_toggle_step_message_to_remove_step() {
    let track = Track::empty();
    let message1 = Message::NoteOn { note_number: 43 };
    let message2 = Message::NoteOn { note_number: 43 };
    let processed_track = track.process_messages(vec![message1, message2]);
    assert_eq!(
        0,
        processed_track
            .events_between(Measure(1, 4), Measure(4, 4))
            .len()
    );
}

#[test]
fn test_process_messages_when_empty() {
    let track1 = Track::empty();
    let message1 = Message::NoteOn { note_number: 43 };
    let track2 = track1.process_messages(vec![message1]);
    let track3 = track2.process_messages(vec![]);

    assert_eq!(1, track3.events_between(Measure(1, 4), Measure(4, 4)).len());
}
