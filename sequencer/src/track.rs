use crate::control::Message;
use crate::event::Event;
use crate::measure::Measure;

#[derive(Clone)]
pub struct Track {
    events: Vec<Event>,
}

impl Track {
    pub fn empty() -> Track {
        Track { events: vec![] }
    }

    pub fn add_event(&mut self, event: Event) {
        if !self.events.contains(&event) {
            self.events.push(event)
        }
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

    #[allow(dead_code)]
    pub fn process_message(&self, message: Message) -> Track {
        match message {
            Message::ToggleStep { step: measure } => {
                let mut new_track = self.clone();
                new_track.add_event(Event { start: measure });
                new_track
            }
            Message::Unhandled => self.clone(),
        }
    }
}

#[test]
fn test_add_event() {
    let mut track = Track::empty();
    track.add_event(Event {
        start: Measure(2, 16),
    });

    let events = track.events_between(Measure(1, 16), Measure(3, 16));
    assert_eq!(events.len(), 1);

    // Do not allow the same event to be added twice
    track.add_event(Event {
        start: Measure(2, 16),
    });
    let events = track.events_between(Measure(1, 16), Measure(3, 16));
    assert_eq!(events.len(), 1);
}

#[test]
fn test_events_between() {
    let mut track = Track::empty();
    track.add_event(Event {
        start: Measure(2, 16),
    });

    let events = track.events_between(Measure(1, 16), Measure(3, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let events = track.events_between(Measure(3, 16), Measure(4, 16));
    assert!(events.is_empty());

    let events = track.events_between(Measure(17, 16), Measure(19, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let mut track = Track::empty();
    track.add_event(Event {
        start: Measure(1, 16),
    });
    let events = track.events_between(Measure(1, 32), Measure(2, 32));
    assert_eq!(Measure(1, 16), events[0].start);
}

#[test]
fn test_process_message() {
    let track = Track::empty();
    let message = Message::ToggleStep {
        step: Measure(2, 4),
    };
    let processed_track = track.process_message(message);
    assert_eq!(
        1,
        processed_track
            .events_between(Measure(1, 4), Measure(4, 4))
            .len()
    );
}
