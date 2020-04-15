use crate::event::Event;
use crate::measure::Measure;

pub struct Track {
    events: Vec<Event>,
}

impl Track {
    pub fn new() -> Track {
        Track { events: vec![] }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event)
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
}

#[test]
fn test_events_between() {
    let mut track = Track::new();
    track.add_event(Event {
        start: Measure(2, 16),
    });

    let events = track.events_between(Measure(1, 16), Measure(3, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let events = track.events_between(Measure(3, 16), Measure(4, 16));
    assert!(events.is_empty());

    let events = track.events_between(Measure(17, 16), Measure(19, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let mut track = Track::new();
    track.add_event(Event {
        start: Measure(1, 16),
    });
    let events = track.events_between(Measure(1, 32), Measure(2, 32));
    assert_eq!(Measure(1, 16), events[0].start);
}
